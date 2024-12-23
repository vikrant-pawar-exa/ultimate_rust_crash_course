use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;
pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for x in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for y in 0..NUM_ROWS {
            if y == 0 {
                if x == 0 {
                    col.push("┌");
                } else if x == NUM_COLS - 1 {
                    col.push("┐");
                } else {
                    col.push("─");
                }
            } else if y == NUM_ROWS - 1 {
                if x == 0 {
                    col.push("└");
                } else if x == NUM_COLS - 1 {
                    col.push("┘");
                } else {
                    col.push("─");
                }
            } else if x == 0 || x == NUM_COLS - 1 {
                col.push("│");
            } else {
                col.push(" ");
            }
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

