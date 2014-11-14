extern crate ncurses;
use std::char;
use std::mem;
use std::rand::{task_rng, Rng};
use std::collections::enum_set::{EnumSet, CLike};

pub const WIDTH: uint = 4;
pub const SIZE: uint = 4;

#[repr(uint)]
enum Move {
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

struct Desk15 {
	desk: Vec<Vec<uint>>,
	empty_pos: (uint, uint)
}

impl Desk15 {
	
	fn new() -> Desk15 {
		let mut rng = task_rng();
		let mut vec = Vec::from_fn(4, |_| Vec::from_elem(SIZE, 0u));
		let mut numbers = range(1u, 16).collect::<Vec<uint>>();
		
		rng.shuffle(numbers.as_mut_slice());
		
		let mut i = 0;
		for n in numbers.iter() {
			vec[i / SIZE][i % SIZE] = *n;
			i += 1;
		}
		
		Desk15{desk: vec, empty_pos: (3, 3)}
	}
	
	fn drow(&self, window: ncurses::WINDOW) {
		let mut i = 0;
		for row in self.desk.iter() {
			let mut j = 0;
			for el in row.iter() {
				let mut val = ".".to_string();
				match el {
					&0 => {},
					_ => { val = el.to_string() }
				}
				ncurses::mvwprintw(window, i, j, format_middle(val, WIDTH as uint).as_slice());
				j += WIDTH as i32;
			}
			i += 1;
		}
		ncurses::wrefresh(window);
	}

	fn apply_move(&mut self, m: Move) -> bool {
		if self.available_moves().contains(&m) {
			let (row, col) = self.empty_pos;
			match m {
				LEFT => {
					self.empty_pos = (row, col + 1)
				},
				RIGHT => {
					self.empty_pos = (row, col - 1)
				},
				UP => {
					self.empty_pos = (row + 1, col)
				},
				DOWN => {
					self.empty_pos = (row - 1, col)
				},
				None => {
					return false;
				}
			}
			let (new_row, new_col) = self.empty_pos;
			self.desk[row][col] = self.desk[new_row][new_col];
			self.desk[new_row][new_col] = 0;
			true
		} else {
			false
		}
		
	}

	fn available_moves(&self) -> EnumSet<Move> {
		let mut moves: EnumSet<Move> = EnumSet::new();
		
		moves.insert(LEFT);
		moves.insert(RIGHT);
		moves.insert(UP);
		moves.insert(DOWN);
		
		let (row, col) = self.empty_pos;
		if col == 0 {
			moves.remove(&RIGHT);
		} else if col == SIZE - 1 {
			moves.remove(&LEFT);
		}

		if row == 0 {
			moves.remove(&DOWN);
		} else if row == SIZE - 1 {
			moves.remove(&UP);
		}
		
		moves
	}
}

pub fn play() {

	init_ncurses();

	let mut game_desk = Desk15::new();
	let game_window = ncurses::newwin(SIZE as i32, (WIDTH * SIZE) as i32, 2, WIDTH as i32);

	let rows = ncurses::getmaxy(ncurses::stdscr);
	// let cols = ncurses::getmaxx(ncurses::stdscr);

	
	ncurses::attron(ncurses::A_REVERSE());
	ncurses::mvprintw(rows - 3, 1, "'Q' to exit");
	ncurses::attroff(ncurses::A_REVERSE());
	ncurses::refresh();

	game_desk.drow(game_window);

	/* Wait for a key press. */
	loop {
		let ch = ncurses::getch();
		let letter = char::from_u32(ch as u32).expect("Not a char");
		if letter == 'q' || letter == 'Q' {
			break;
		}

		let mut m: Move = None;
		match ch {
			ncurses::KEY_UP 	=> { ncurses::mvprintw(rows - 2, 1, "UP       "); m = UP; },
			ncurses::KEY_DOWN 	=> { ncurses::mvprintw(rows - 2, 1, "DOWN     "); m = DOWN; },
			ncurses::KEY_LEFT 	=> { ncurses::mvprintw(rows - 2, 1, "LEFT     "); m = LEFT; },
			ncurses::KEY_RIGHT 	=> { ncurses::mvprintw(rows - 2, 1, "RIGHT    "); m = RIGHT },
			_					=> { ncurses::mvprintw(rows - 2, 1, "Not arrow"); }
		}
		ncurses::mvprintw(rows - 1, 1, format!("Applied: {}  ", game_desk.apply_move(m)).as_slice());
		game_desk.drow(game_window);
	}

	/* Terminate ncurses. */
	ncurses::endwin();
}

fn init_ncurses() {
	ncurses::initscr();
	ncurses::cbreak();
	ncurses::keypad(ncurses::stdscr, true);
	ncurses::noecho();
	ncurses::curs_set(ncurses::CURSOR_INVISIBLE);
	ncurses::start_color();
}

fn format_middle(val: String, width: uint) -> String {
	let len = val.len();
	let mut res: Vec<String> = vec![];
	if len < width {
		let end = (width - len) / 2;
		let start = width - len - end;
		res.push(String::from_char(start, ' '));
		res.push(val);
		res.push(String::from_char(end, ' '));
	} else {
		res.push(val.to_string());
	}
	res.concat()
}