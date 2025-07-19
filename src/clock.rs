// clock.rs
use ncurses::WINDOW;
use crate::digits::{NUMBER, DIGIT_SIZE};
use crate::style::DIGIT_STYLES;

pub struct Clock {
    pub y: i32,
    pub x: i32,
    pub digit_width: i32,
    pub style_index: usize,
}

impl Clock {
    pub fn new(y: i32, x: i32, style_index: usize) -> Self {
        Clock {
            y,
            x,
            digit_width: 7, // 5 columns + 2 spaces
            style_index: style_index.min(DIGIT_STYLES.len() - 1),
        }
    }

    pub fn draw_digit(&self, win: WINDOW, number: usize, x: i32, y: i32) {
        let matrix = &NUMBER[number];
        let ch = DIGIT_STYLES[self.style_index];
        for i in 0..DIGIT_SIZE * DIGIT_SIZE {
            if matrix[i] {
                let dx = (i % DIGIT_SIZE) as i32;
                let dy = (i / DIGIT_SIZE) as i32;
                ncurses::mvwaddstr(win, y + dy, x + dx, ch);
            }
        }
    }

    pub fn draw_colon(&self, win: WINDOW, x: i32, y: i32) {
        ncurses::mvwaddstr(win, y + 1, x, ".");
        ncurses::mvwaddstr(win, y + 3, x, ".");
    }

    pub fn draw_time(&self, win: WINDOW, hour: u32, minute: u32, second: u32) {
        let x = self.x;
        let y = self.y;
        let w = self.digit_width;

        self.draw_digit(win, (hour / 10) as usize, x + 0 * w, y);
        self.draw_digit(win, (hour % 10) as usize, x + 1 * w, y);

        self.draw_colon(win, x + 2 * w, y);

        self.draw_digit(win, (minute / 10) as usize, x + 3 * w, y);
        self.draw_digit(win, (minute % 10) as usize, x + 4 * w, y);

        self.draw_colon(win, x + 5 * w, y);

        self.draw_digit(win, (second / 10) as usize, x + 6 * w, y);
        self.draw_digit(win, (second % 10) as usize, x + 7 * w, y);
    }
}
