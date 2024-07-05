use piston_window::types::{Color, Width};
use piston_window::*;

use crate::draw::{draw_block, draw_rectangle};
use rand::{thread_rng, Rng};

const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    width: i32,
    height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game { width, height }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(BORDER_COLOR, 0, y, self.width, 1, con, g);
    }
}
