extern crate ncurses;
use std::collections::enum_set::CLike;
use std::mem;

#[repr(uint)]
pub enum Move {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    None
}

impl CLike for Move {
    fn to_uint(&self) -> uint {
        *self as uint
    }

    fn from_uint(v: uint) -> Move {
        unsafe { mem::transmute(v) }
    }
}

pub trait Game {
	fn new() -> Self;
	fn is_finished(&self) -> bool;
	fn apply_move(&mut self, m: Move) -> bool;
	fn score(&self) -> uint;
	fn drow(&self, window: ncurses::WINDOW);
	fn window_size(&self) -> (uint, uint);
}


pub mod utils;
pub mod little15;
pub mod ui;
