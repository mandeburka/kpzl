extern crate ncurses;

use std::rand::{task_rng, Rng};
use std::num::Float;
use std::collections::DList;
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

    fn collapse_left(numbers: &[uint]) -> Vec<uint> {
    	let mut result = Vec::from_elem(numbers.len(), 0u);
    	let mut list: DList<uint> = numbers.iter().filter(|&x| *x > 0).map(|&x| x).collect();
    	loop {
    		if list.len() == 0 {
    			break;
    		}
    		let mut num = list.pop_front().expect("No elements in list");
    		let neighbour = *list.front().unwrap_or(&0u);
    		if num == neighbour {
    			list.pop_front();
    			num += neighbour;
    		}
    		result.push(num);
    	}
    	result
    }

    fn collapse_cols(&mut self, reversed: bool) {
    	for row in range(0, 4) {
	    	let mut cols: Vec<uint> = range(0u, 4).collect();
	    	let mut numbers = Vec::new();
	    	
	    	if reversed {
	    		cols.as_mut_slice().reverse();
	    	}
	    	
	    	for col in cols.iter() {
	    		numbers.push(self.desk[row][*col]);
	    	}
	    	
	    	let mut collapsed = Super2048::collapse_left(numbers.as_slice());
	    	
	    	if reversed {
	    		collapsed.as_mut_slice().reverse();
	    	}
	    	
	    	for col in cols.iter() {
	    		self.desk[row][*col] = collapsed.pop().unwrap_or(0);
	    	}
	    }

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
				Move::LEFT => {
					self.collapse_cols(false);
				},
				Move::RIGHT => {
					self.collapse_cols(true);
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
