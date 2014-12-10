extern crate ncurses;

use std::rand::{task_rng, Rng};
use std::num::Float;
use super::{Move, Game};
use utils::Color;

pub struct Super2048 {
	desk: Vec<Vec<uint>>,
    score: uint
}

impl Super2048 {
	fn put_number(&mut self) {
		let choices = [2u, 4];
		let mut rng = task_rng();
		let &(row, col) = rng.choose(self.free_positions().as_slice()).expect("No free positions");
		let &number = rng.choose(&choices).expect("No choices for default number");
		self.desk[row][col] = number;
	}

	fn free_positions(&self) -> Vec<(uint, uint)> {
		let mut free: Vec<(uint, uint)> = vec![];
		for r in range(0u, 4) {
			for c in range(0u, 4) {
				if self.desk[r][c] == 0 {
					free.push((r, c))
				}
			}
		}
		free
	}

	fn get_color(&self, n: uint) -> Color {
		let colors = [Color::CYAN, Color::GREEN, Color::MAGENTA, Color::BLUE, Color::RED];
		let power = (n as f32).log2() as uint;
		colors[colors.len() % power]
    }
}

impl Game for Super2048 {
	fn new() -> Super2048 {
        let vec = Vec::from_fn(4, |_| Vec::from_elem(4, 0u));
        let mut game = Super2048 {desk: vec, score: 0 };
        game.put_number();
        game.put_number();
        game
    }

	fn is_finished(&self) -> bool {
		self.free_positions().len() == 0
	}

	fn window_size(&self) -> (uint, uint) {
		(4, 16)
	}

	fn score(&self) -> uint {
		self.score
	}

	fn apply_move(&mut self, m: Move) -> bool {
		if self.is_finished() {
			false
		} else {
			match m {
				Move::RIGHT => {
					
				},
				_ => {}
			}
			self.put_number();
			true
		}
	}

	fn drow(&self, window: ncurses::WINDOW) {
        let mut i = 0;
        for row in self.desk.iter() {
            let mut j = 0;
            for el in row.iter() {
                let (val, attrs) = match el {
                    &0 => {
                        (".".to_string(), ncurses::COLOR_PAIR(Color::YELLOW as i16))
                    },
                    &n => {
                        (el.to_string(), ncurses::COLOR_PAIR(self.get_color(n) as i16))
                    }
                };
                let cell = format!("{:>4}", val);
                
                ncurses::wattron(window, attrs);
                ncurses::mvwprintw(window, i, j, cell.as_slice());
                ncurses::wattroff(window, attrs);
                j += cell.len() as i32;
            }
            i += 1;
        }
        ncurses::wrefresh(window);
    }
}