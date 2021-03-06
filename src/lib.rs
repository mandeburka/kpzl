extern crate ncurses;
extern crate collections;

use collections::enum_set::CLike;

pub enum Move {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    None
}

impl CLike for Move {
    fn to_uint(&self) -> usize {
        match *self {
            Move::LEFT => 1,
            Move::RIGHT => 2,
            Move::UP => 3,
            Move::DOWN => 4,
            _ => 0
        }
    }

    fn from_uint(v: usize) -> Move {
        match v {
            1 => Move::LEFT,
            2 => Move::RIGHT,
            3 => Move::UP,
            4 => Move::DOWN,
            _ => Move::None,
        }
    }
}

impl Copy for Move {}

pub trait Game {
	fn new() -> Self;
	fn is_finished(&self) -> bool;
	fn apply_move(&mut self, m: Move) -> bool;
	fn score(&self) -> u32;
	fn drow(&self, window: ncurses::WINDOW);
	fn window_size(&self) -> (u32, u32);
}


pub mod utils;
pub mod little15;
pub mod ui;
pub mod super2048;
