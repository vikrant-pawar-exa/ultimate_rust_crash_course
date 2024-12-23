use crate::frame::Frame;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use std::io::{Stdout, Write};





pub fn render(stdout: &mut Stdout, last_frame: &Frame , curr_frame: &Frame , force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    for ( x , col) in curr_frame.iter().enumerate() {
        for ( y , cell) in col.iter().enumerate() {
           if *cell != last_frame[x][y] || force {
               stdout.queue(MoveTo(x as u16 , y as u16)).unwrap();
               print!("{}", *cell);
           }
        }
    }
    stdout.flush().unwrap();
}

// pub fn render(frame: &Frame) {
//     for row in frame {
//         for col in row {
//             print!("{}", col);
//         }
//         println!();
//     }
// }