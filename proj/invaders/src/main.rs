use std::time::Instant;

use std::vec;
use std::{
    error::Error,
    sync::mpsc::{self},
    time::Duration,
    {io, thread},
};

use invaders::invaders::Invaders;
use rusty_audio::Audio;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::cursor::Hide;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal;
use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode};

use invaders::{frame::{self, Drawable}, player::Player, render};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode" ,"audio/explode.wav");
    audio.add("lose" ,"audio/lose.wav");
    audio.add("move", "audio/move.wav");
    audio.add("pew", "audio/pew.wav");
    audio.add("startup", "audio/startup.wav");
    audio.add("win", "audio/win.wav");

    // Terminal

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a seprate thread

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        // let mut curr_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });



    // Game loop

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();


    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();


        // Input
        while event::poll(std::time::Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => {
                        player.move_left();
                    }
                    KeyCode::Right => {
                        player.move_right();
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shot() {
                            audio.play("pew");
                        }
                    }
                    _ => {}
                }
            }
        }

        // Updates

        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw and render
        let drawables: Vec<&dyn Drawable> =  vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));


        // Win or lose ?
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }else if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }


    }


    //cleanup
    
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}
