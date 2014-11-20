extern crate ncurses;
use std::char;
use super::{Move, Game};


const WIDTH: uint = 4;
const SIZE: uint = 4;

/* Color pairs; foreground && background. */
static COLOR_PAIR_GREEN: i16 = 1;
static COLOR_PAIR_YELLOW: i16 = 2;
static COLOR_PAIR_WHITE: i16 = 3;
static COLOR_PAIR_CYAN: i16 = 4;
static COLOR_PAIR_MAGENTA: i16 = 5;

pub fn play<T: Game>() {

    init_ncurses();

    let mut game_desk: T = Game::new();
    let game_window = ncurses::newwin(SIZE as i32, (WIDTH * SIZE) as i32, 2, WIDTH as i32);
    let stats_window = ncurses::newwin(3, 20, 2, (WIDTH * SIZE) as i32 + 8);
    let rows = ncurses::getmaxy(ncurses::stdscr);
    
    ncurses::attron(ncurses::A_REVERSE());
    ncurses::mvprintw(rows - 2, 1, "'Q' to exit");
    ncurses::attroff(ncurses::A_REVERSE());

    ncurses::refresh();

    drow(&game_desk, game_window);
    update_stats(stats_window, &game_desk);

    /* Wait for a key press. */
    loop {
        let ch = ncurses::getch();
        let letter = char::from_u32(ch as u32).expect("Not a char");
        if letter == 'q' || letter == 'Q' {
            break;
        }

        let m: Move = match ch {
            ncurses::KEY_UP     => { Move::UP },
            ncurses::KEY_DOWN   => { Move::DOWN },
            ncurses::KEY_LEFT   => { Move::LEFT },
            ncurses::KEY_RIGHT  => { Move::RIGHT },
            _                   => { Move::None }
        };
        if game_desk.apply_move(m) {
            drow(&game_desk, game_window);
            update_stats(stats_window, &game_desk);
        }

        if game_desk.is_finished() {
            ncurses::getch();
            break;
        }
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

    ncurses::init_pair(COLOR_PAIR_GREEN, ncurses::COLOR_GREEN, ncurses::COLOR_BLACK);
    ncurses::init_pair(COLOR_PAIR_YELLOW, ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK);
    ncurses::init_pair(COLOR_PAIR_WHITE, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK);
    ncurses::init_pair(COLOR_PAIR_CYAN, ncurses::COLOR_CYAN, ncurses::COLOR_BLACK);
    ncurses::init_pair(COLOR_PAIR_MAGENTA, ncurses::COLOR_MAGENTA, ncurses::COLOR_BLACK);
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

fn update_stats(window: ncurses::WINDOW, desk: &Game) {
    ncurses::wattron(window, ncurses::COLOR_PAIR(COLOR_PAIR_GREEN));
    ncurses::mvwprintw(window, 0, 0, format!("Score: {}", desk.score()).as_slice());
    ncurses::wattroff(window, ncurses::COLOR_PAIR(COLOR_PAIR_GREEN));
    // TODO: implement best
    // ncurses::mvwprintw(window, 1, 0, format!("Best: {}", 0u).as_slice());
    if desk.is_finished() {
        ncurses::wattron(window, ncurses::COLOR_PAIR(COLOR_PAIR_MAGENTA));
        ncurses::mvwprintw(window, 2, 0, "You won!");
        ncurses::wattroff(window, ncurses::COLOR_PAIR(COLOR_PAIR_MAGENTA));
    } else {
        ncurses::mvwprintw(window, 2, 0, "            ");
    }
    ncurses::wrefresh(window);
}

fn drow(game: &Game, window: ncurses::WINDOW) {
        let mut i = 0;
        for row in game.desk().iter() {
            let mut j = 0;
            for el in row.iter() {
                let mut val = ".".to_string();
                let mut attrs = ncurses::COLOR_PAIR(COLOR_PAIR_YELLOW);
                match el {
                    &0 => {},
                    _ => { attrs = ncurses::COLOR_PAIR(COLOR_PAIR_CYAN); val = el.to_string() }
                }
                ncurses::wattron(window, attrs);
                ncurses::mvwprintw(window, i, j, format_middle(val, WIDTH as uint).as_slice());
                ncurses::wattroff(window, attrs);
                j += WIDTH as i32;
            }
            i += 1;
        }
        ncurses::wrefresh(window);
    }
