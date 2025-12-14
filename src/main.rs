mod digits;
mod clock;
mod style;
mod colors;

use std::ffi::CString;
use libc;
use ncurses::*;
use chrono::{Local, Timelike};
use clock::Clock;
use style::{HourFormat, HorizontalAlign, VerticalAlign};

fn main() {
    // Initialize locale for UTF-8
    unsafe {
        let locale = CString::new("").unwrap();
        libc::setlocale(libc::LC_ALL, locale.as_ptr());
    }
    initscr();
    colors::init_colors();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    timeout(100);

    // You can change the style index (0..7) to select the digit style
    // Example: style 0 (█), 24h format, centered horizontally and vertically
    let clock = Clock::new(0, HourFormat::H12, HorizontalAlign::Center, VerticalAlign::Center);

    loop {
        erase();

        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();


        // Draw digits without color
        clock.draw_time(stdscr(), hour, minute, second);

        // Draw the date below the clock, centered
        let (max_y, max_x) = {
            let mut y = 0;
            let mut x = 0;
            getmaxyx(stdscr(), &mut y, &mut x);
            (y, x)
        };
        let date_str = style::format_date(&now);
        let date_x = (max_x.saturating_sub(date_str.len() as i32)) / 2;
        let date_y = (max_y / 2) + 4; // 4 lines below vertical center (clock is 5 lines tall)
        mvprintw(date_y, date_x.max(0), &date_str);

        refresh();

        let key = getch();
        if key == 'q' as i32 || key == 27 {
            break;
        }
    }

    endwin();
}