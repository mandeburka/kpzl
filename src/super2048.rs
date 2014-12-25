extern crate ncurses;

use std::rand::{task_rng, Rng};
use std::num::Float;
use std::collections::DList;
use super::{Move, Game};
use utils::Color;

pub struct Super2048 {
	desk: Vec<Vec<uint>>,
    score: uint,
    colors: Vec<Color>
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
		let power = (n as f32).log2() as uint - 1;
		let position = power % self.colors.len();
		self.colors[position]
    }

    fn collapse_left(numbers: &[uint]) -> (Vec<uint>, uint) {
    	let mut result: Vec<uint> = Vec::new();
    	let mut list: DList<uint> = numbers.iter().filter(|&x| *x > 0).map(|&x| x).collect();
    	let mut score = 0u;
    	loop {
    		if list.len() == 0 {
    			break;
    		}
    		let mut num = list.pop_front().expect("No elements in list");
    		let neighbour = *list.front().unwrap_or(&0u);
    		if num == neighbour {
    			list.pop_front();
    			num += neighbour;
    			score += num;
    		}
    		result.push(num);
    	}
    	(result, score)
    }

    fn collapse_cols(&mut self, reversed: bool) -> bool {
    	let mut collapsed = false;
    	for row in range(0, 4) {
	    	let mut cols: Vec<uint> = range(0u, 4).collect();
	    	let mut numbers = Vec::new();
	    	
	    	if reversed {
	    		cols.as_mut_slice().reverse();
	    	}
	    	for col in cols.iter() {
	    		numbers.push(self.desk[row][*col]);
	    	}
	    	
	    	let (mut collapsed_row, score) = Super2048::collapse_left(numbers.as_slice());
	    	
    		collapsed_row.as_mut_slice().reverse();
	    	
	    	for col in cols.iter() {
	    		let new_value = collapsed_row.pop().unwrap_or(0);
	    		if !collapsed && new_value != self.desk[row][*col] {
	    			collapsed = true;
	    		}
	    		self.desk[row][*col] = new_value;
	    	}

	    	self.score += score;
	    }
	    collapsed
    }

    fn collapse_rows(&mut self, reversed: bool) -> bool {
    	let mut collapsed = false;
    	for col in range(0, 4) {
	    	let mut rows: Vec<uint> = range(0u, 4).collect();
	    	let mut numbers = Vec::new();
	    	
	    	if reversed {
	    		rows.as_mut_slice().reverse();
	    	}
	    	for row in rows.iter() {
	    		numbers.push(self.desk[*row][col]);
	    	}
	    	
	    	let (mut collapsed_row, score) = Super2048::collapse_left(numbers.as_slice());
	    	
    		collapsed_row.as_mut_slice().reverse();
	    	
	    	for row in rows.iter() {
	    		let new_value = collapsed_row.pop().unwrap_or(0);
	    		if !collapsed && new_value != self.desk[*row][col] {
	    			collapsed = true;
	    		}
	    		self.desk[*row][col] = new_value;
	    	}

	    	self.score += score;
	    }
	    collapsed
    }

    fn fmt_number(n: uint) -> String {
    	if n < 1024 {
    		n.to_string()
    	} else {
    		"1K".to_string()
    	}
    }

    fn has_moves(&self) -> bool {
    	if self.free_positions().len() > 0 {
    		true
    	} else {
	    	for row in range(0, 4) {
	    		for col in range(0, 4) {
	    			if  (row < 3 && self.desk[row][col] == self.desk[row + 1][col]) ||
	    				(col < 3 && self.desk[row][col] == self.desk[row][col + 1])
	    			{
	    				return true;
	    			}
	    		}
	    	}
	    	false
	    }
    }
}

impl Game for Super2048 {
	fn new() -> Super2048 {
        let vec = Vec::from_fn(4, |_| Vec::from_elem(4, 0u));
        let mut game = Super2048 {
        	desk: vec,
        	score: 0,
        	colors: vec![Color::WHITE, Color::CYAN, Color::GREEN, Color::MAGENTA, Color::RED]
        };
        game.put_number();
        game.put_number();
        game
    }

	fn is_finished(&self) -> bool {
		!self.has_moves()
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
			let collapsed = match m {
				Move::LEFT => {
					self.collapse_cols(false)
				},
				Move::RIGHT => {
					self.collapse_cols(true)
				},
				Move::UP => {
					self.collapse_rows(false)
				},
				Move::DOWN => {
					self.collapse_rows(true)
				},
				_ => { false }
			};
			if collapsed {
				self.put_number();
			}
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
                        (Super2048::fmt_number(n), ncurses::COLOR_PAIR(self.get_color(n) as i16))
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

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
    use super::Super2048;
    use super::super::Game;

    #[test]
    fn test_collapse_left() {
        let res = Super2048::collapse_left(&[2, 2]);
        assert_eq!(res, (vec![4], 4));

        let res = Super2048::collapse_left(&[2, 2, 4]);
        assert_eq!(res, (vec![4, 4], 4));

        let res = Super2048::collapse_left(&[2, 2, 4, 4, 16, 16]);
        assert_eq!(res, (vec![4, 8, 32], 44));

        let res = Super2048::collapse_left(&[2, 0, 2, 16]);
        assert_eq!(res, (vec![4, 16], 4));
    }

    #[test]
    fn test_collapse_desk_left() {
    	let mut game = new_blank_game();
    
    	game.desk[0] = vec![2, 2, 0, 0];
    	game.desk[1] = vec![2, 2, 16, 16];
    	game.desk[2] = vec![8, 0, 8, 16];
    	game.desk[3] = vec![0, 0, 1024, 1024];
    	game.collapse_cols(false);

    	assert_eq!(game.desk[0], vec![4, 0, 0, 0]);
    	assert_eq!(game.desk[1], vec![4, 32, 0, 0]);
    	assert_eq!(game.desk[2], vec![16, 16, 0, 0]);
    	assert_eq!(game.desk[3], vec![2048, 0, 0, 0]);
    }

    #[test]
    fn test_collapse_desk_right() {
    	let mut game = new_blank_game();
    
    	game.desk[0] = vec![2, 2, 0, 0];
    	game.desk[1] = vec![2, 2, 16, 16];
    	game.desk[2] = vec![8, 0, 8, 16];
    	game.desk[3] = vec![0, 0, 1024, 1024];
    	game.collapse_cols(true);

    	assert_eq!(game.desk[0], vec![0, 0, 0, 4]);
    	assert_eq!(game.desk[1], vec![0, 0, 4, 32]);
    	assert_eq!(game.desk[2], vec![0, 0, 16, 16]);
    	assert_eq!(game.desk[3], vec![0, 0, 0, 2048]);

    	game.desk[0] = vec![  4,  4, 32, 32];
    	game.desk[1] = vec![  0,  0,  4, 16];
    	game.desk[2] = vec![  0,  0,  0,  8];
    	game.desk[3] = vec![  0,  2,  0,  2];
    	game.collapse_cols(true);

    	assert_eq!(game.desk[0], vec![0,  0,  8, 64]);
    	assert_eq!(game.desk[1], vec![0,  0,  4, 16]);
    	assert_eq!(game.desk[2], vec![0,  0,  0,  8]);
    	assert_eq!(game.desk[3], vec![0,  0,  0,  4]);
    }

    #[test]
    fn test_collapse_desk_up() {
    	let mut game = new_blank_game();
    
    	game.desk[0] = vec![ 2,  2,  8,  0];
    	game.desk[1] = vec![ 2,  2,  0,  0];
    	game.desk[2] = vec![ 0, 16,  8, 32];
    	game.desk[3] = vec![ 0, 16, 16, 32];
    	game.collapse_rows(false);

    	assert_eq!(game.desk[0], vec![  4,  4, 16, 64]);
    	assert_eq!(game.desk[1], vec![  0, 32, 16,  0]);
    	assert_eq!(game.desk[2], vec![  0,  0,  0,  0]);
    	assert_eq!(game.desk[3], vec![  0,  0,  0,  0]);
    }

    #[test]
    fn test_collapse_desk_down() {
    	let mut game = new_blank_game();
    
    	game.desk[0] = vec![ 2,  2,  8,  0];
    	game.desk[1] = vec![ 2,  2,  0,  0];
    	game.desk[2] = vec![ 0, 16,  8, 32];
    	game.desk[3] = vec![ 0, 16, 16, 32];
    	game.collapse_rows(true);

    	assert_eq!(game.desk[0], vec![  0,  0,  0,  0]);
    	assert_eq!(game.desk[1], vec![  0,  0,  0,  0]);
    	assert_eq!(game.desk[2], vec![  0,  4, 16,  0]);
    	assert_eq!(game.desk[3], vec![  4, 32, 16, 64]);
    }

    #[test]
    fn test_get_color() {
    	let game: Super2048 = Game::new();
    	let mut number_colors = HashMap::new();
    	number_colors.insert(  2u, game.colors[0]);
    	number_colors.insert(  4u, game.colors[1]);
    	number_colors.insert(  8u, game.colors[2]);
    	number_colors.insert( 16u, game.colors[3]);
    	number_colors.insert( 32u, game.colors[4]);
    	number_colors.insert( 64u, game.colors[0]);
    	number_colors.insert(128u, game.colors[1]);

    	for (number, color) in number_colors.iter() {
    		assert_eq!(game.get_color(*number), *color);
    	}
    }

    #[test]
    fn has_moves() {
    	let mut game = new_blank_game();
    
    	game.desk[0] = vec![  8,  4,   2, 512];
    	game.desk[1] = vec![ 32, 64, 256,  16];
    	game.desk[2] = vec![  4, 16,   8,   4];
    	game.desk[3] = vec![  8,  4,   4,   2];

    	assert!(game.has_moves());
    }

    fn new_blank_game() -> Super2048 {
    	let mut game: Super2048 = Game::new();
    	for r in range(0, 4) {
    		for c in range(0, 4) {
    			game.desk[r][c] = 0;
    		}
    	}
    	game
    }
}
