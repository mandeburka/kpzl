extern crate ncurses;

use std::char;
use super::{Move, Game};
use utils::Color;


pub fn play<T: Game>() {

    init_ncurses();

    let mut game: T = Game::new();
    let (grows, gcols) = game.window_size();
    let game_window = ncurses::newwin(grows as i32, gcols as i32, 2, 2);
    let stats_window = ncurses::newwin(3, 20, 2, gcols as i32 + 8);
    let rows = ncurses::getmaxy(ncurses::stdscr);
    
    ncurses::attron(ncurses::A_REVERSE());
    ncurses::mvprintw(rows - 2, 1, "'Q' to exit");
    ncurses::attroff(ncurses::A_REVERSE());

    ncurses::refresh();

    game.drow(game_window);
    update_stats(&game, stats_window);

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
        if game.apply_move(m) {
            game.drow(game_window);
            update_stats(&game, stats_window);
        }

        if game.is_finished() {
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
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::start_color();

    init_colors();
}

fn init_colors() {
	ncurses::init_pair(Color::GREEN as i16, ncurses::COLOR_GREEN, ncurses::COLOR_BLACK);
    ncurses::init_pair(Color::YELLOW as i16, ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK);
    ncurses::init_pair(Color::WHITE as i16, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK);
    ncurses::init_pair(Color::CYAN as i16, ncurses::COLOR_CYAN, ncurses::COLOR_BLACK);
    ncurses::init_pair(Color::MAGENTA  as i16, ncurses::COLOR_MAGENTA, ncurses::COLOR_BLACK);
}

fn update_stats(desk: &Game, window: ncurses::WINDOW) {
    ncurses::wattron(window, ncurses::COLOR_PAIR(Color::GREEN as i16));
    ncurses::mvwprintw(window, 0, 0, format!("Score: {}", desk.score()).as_slice());
    ncurses::wattroff(window, ncurses::COLOR_PAIR(Color::GREEN as i16));
    // TODO: implement best
    // ncurses::mvwprintw(window, 1, 0, format!("Best: {}", 0u).as_slice());
    if desk.is_finished() {
        ncurses::wattron(window, ncurses::COLOR_PAIR(Color::MAGENTA as i16));
        ncurses::mvwprintw(window, 2, 0, "You won!");
        ncurses::wattroff(window, ncurses::COLOR_PAIR(Color::MAGENTA as i16));
    } else {
        ncurses::mvwprintw(window, 2, 0, "            ");
    }
    ncurses::wrefresh(window);
}
