extern crate ncurses;

use std::{fmt, iter};
use std::fmt::Show;

pub enum Color {
	GREEN = 1,
	YELLOW = 2,
	WHITE = 3,
	CYAN = 4,
	MAGENTA = 5,
    BLUE = 6,
    RED = 7
}

impl Color {
    pub fn name(&self) -> String {
        String::from_str(
            match self {
                &Color::GREEN   => "GREEN",
                &Color::YELLOW  => "YELLOW",
                &Color::WHITE   => "WHITE",
                &Color::CYAN    => "CYAN",
                &Color::MAGENTA => "MAGENTA",
                &Color::BLUE    => "BLUE",
                &Color::RED     => "RED",
            }
        )
    }
}

impl Copy for Color {}

impl Show for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool { self.name() == other.name() }

    fn ne(&self, other: &Color) -> bool { !self.eq(other) }
}

pub fn format_middle(val: String, width: usize) -> String {
    let len = val.len();
    let mut res: Vec<String> = vec![];
    if len < width {
        let end = (width - len) / 2;
        let start = width - len - end;
        for _ in range(0, start) {

        }
        res.push(iter::repeat(' ').take(start).collect());
        res.push(val);
        res.push(iter::repeat(' ').take(end).collect());
    } else {
        res.push(val.to_string());
    }
    res.concat()
}
