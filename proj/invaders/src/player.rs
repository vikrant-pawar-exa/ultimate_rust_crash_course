use std::time::Duration;

use crate::{frame::{Drawable, Frame}, invaders::Invaders, shot::Shot, NUM_COLS, NUM_ROWS};

pub struct Player {
    pub x: usize,
    pub y: usize,
    shots: Vec<Shot>,
}


impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            // Usable rows are 2 less than the total rows
            y: NUM_ROWS - 2, 
            shots: Vec::new(),
        }
    }
    pub fn move_left(&mut self){
        if self.x > 1 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 2 {
            self.x += 1;
        }
    }

    pub fn shot (&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y -1));  
            true
        }
        else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits (&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut(){
            if !shot.exploding && invaders.kill_invader_at(shot.x, shot.y) {
                shot.explode();
                hit_something = true;
            }
        }
        hit_something
    }

}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";

        for shot in self.shots.iter(){
            shot.draw(frame);

        }
    }
}