
mod clock;
mod digits;
mod style;

use std::ffi::CString;
use libc;
use ncurses::*;
use chrono::{Local, Timelike};
use clock::Clock;

fn main() {
    // Initialize locale for UTF-8
    unsafe {
        let locale = CString::new("").unwrap();
        libc::setlocale(libc::LC_ALL, locale.as_ptr());
    }
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    timeout(100);

    // You can change the style index (0..7) to select the digit style
    let clock = Clock::new(2, 2, 3);

    loop {
        erase();

        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        clock.draw_time(stdscr(), hour, minute, second);

        mvprintw(10, 2, &format!("Time: {:02}:{:02}:{:02}", hour, minute, second));
        mvprintw(11, 2, "Press 'q' or ESC to exit");

        refresh();

        let key = getch();
        if key == 'q' as i32 || key == 27 {
            break;
        }
    }

    endwin();
}