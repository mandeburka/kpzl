extern crate ncurses;
extern crate collections;

use std::rand::{thread_rng, Rng};
use collections::enum_set::EnumSet;
use std::iter::repeat;
use super::{Move, Game};
use utils::Color;

const SIZE: usize = 4;

pub struct Little15 {
    desk: Vec<Vec<u32>>,
    empty_pos: (usize, usize),
    num_of_moves: u32
}

impl Little15 {

    fn _is_solvable(data: &[u32], zero_row: u32) -> bool {
        let mut sum = 0u32;
        for n in range(1u32, 16) {
            sum += data.iter().skip_while(|&x| *x != n).filter(|&x| *x > n).count() as u32;
        }
        (sum + zero_row) % 2 == 0
    }

    fn available_moves(&self) -> EnumSet<Move> {
        let mut moves: EnumSet<Move> = EnumSet::new();
        
        moves.insert(Move::LEFT);
        moves.insert(Move::RIGHT);
        moves.insert(Move::UP);
        moves.insert(Move::DOWN);
        
        let (row, col) = self.empty_pos;
        if col == 0 {
            moves.remove(&Move::RIGHT);
        } else if col == SIZE - 1 {
            moves.remove(&Move::LEFT);
        }

        if row == 0 {
            moves.remove(&Move::DOWN);
        } else if row == SIZE - 1 {
            moves.remove(&Move::UP);
        }
        
        moves
    }
}

impl Game for Little15 {
    fn new() -> Little15 {
        let mut rng = thread_rng();
        let mut vec: Vec<Vec<u32>> = (0..4).map(|_| repeat(0u32).take(SIZE).collect()).collect();
        let mut numbers = range(1u32, 16).collect::<Vec<u32>>();
        
        loop {
            rng.shuffle(numbers.as_mut_slice());
            if Little15::_is_solvable(numbers.as_slice(), 3) {
                break;
            }
        }
        
        let mut i = 0;
        for n in numbers.iter() {
            vec[i / SIZE][i % SIZE] = *n;
            i += 1;
        }
        
        Little15 {desk: vec, empty_pos: (3, 3), num_of_moves: 0 }
    }

    fn is_finished(&self) -> bool {
        if self.empty_pos != (3, 3) {
            false
        } else {
            let mut prev = 0u32;
            for row in self.desk.iter().rev() {
                for n in row.iter().rev() {
                    if prev != 0 && prev != *n + 1 {
                        return false;
                    }
                    prev = *n;
                }
            }
            true
        }
    }

    fn apply_move(&mut self, m: Move) -> bool {
        if self.available_moves().contains(&m) {
            let (row, col) = self.empty_pos;
            match m {
                Move::LEFT => {
                    self.empty_pos = (row, col + 1)
                },
                Move::RIGHT => {
                    self.empty_pos = (row, col - 1)
                },
                Move::UP => {
                    self.empty_pos = (row + 1, col)
                },
                Move::DOWN => {
                    self.empty_pos = (row - 1, col)
                },
                Move::None => {
                    return false;
                }
            }
            let (new_row, new_col) = self.empty_pos;
            self.desk[row][col] = self.desk[new_row][new_col];
            self.desk[new_row][new_col] = 0;
            self.num_of_moves += 1;
            true
        } else {
            false
        }
        
    }

    fn score(&self) -> u32 {
        self.num_of_moves
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
                    _ => {
                        (el.to_string(), ncurses::COLOR_PAIR(Color::CYAN as i16))
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

    fn window_size(&self) -> (u32, u32) {
        (4, 16)
    }

}
