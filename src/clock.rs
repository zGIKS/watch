// clock.rs
use ncurses::WINDOW;
use crate::digits::{NUMBER, DIGIT_SIZE};
use crate::style::DIGIT_STYLES;
use ncurses::{getmaxyx, stdscr};


use crate::style::{HorizontalAlign, VerticalAlign, HourFormat};

pub struct Clock {
    pub digit_width: i32,
    pub style_index: usize,
    pub hour_format: HourFormat,
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
}

impl Clock {
    pub fn new(style_index: usize, hour_format: HourFormat, h_align: HorizontalAlign, v_align: VerticalAlign) -> Self {
        Clock {
            digit_width: 7, // 5 columns + 2 spaces
            style_index: style_index.min(DIGIT_STYLES.len() - 1),
            hour_format,
            h_align,
            v_align,
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        let mut max_y = 0;
        let mut max_x = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let clock_w = 8 * self.digit_width - 1;
        let clock_h = 5;
        let x = match self.h_align {
            HorizontalAlign::Left => 0,
            HorizontalAlign::Center => (max_x - clock_w) / 2,
            HorizontalAlign::Right => max_x - clock_w,
        };
        let y = match self.v_align {
            VerticalAlign::Top => 0,
            VerticalAlign::Center => (max_y - clock_h) / 2,
            VerticalAlign::Bottom => max_y - clock_h,
        };
        (x.max(0), y.max(0))
    }

    pub fn format_hour(&self, hour: u32) -> (u32, Option<&'static str>) {
        match self.hour_format {
            HourFormat::H24 => (hour, None),
            HourFormat::H12 => {
                let h = match hour {
                    0 => 12,
                    1..=12 => hour,
                    _ => hour - 12,
                };
                let ampm = if hour < 12 { "AM" } else { "PM" };
                (h, Some(ampm))
            }
        }
    }

    pub fn draw_digit(&self, win: WINDOW, number: usize, x: i32, y: i32) {
        let matrix = &NUMBER[number];
        let ch = DIGIT_STYLES[self.style_index];
        for i in 0..DIGIT_SIZE * DIGIT_SIZE {
            if matrix[i] {
                let dx = (i % DIGIT_SIZE) as i32;
                let dy = (i / DIGIT_SIZE) as i32;
                // Color is handled in main.rs
                ncurses::mvwaddstr(win, y + dy, x + dx, ch);
            }
        }
    }

    pub fn draw_colon(&self, win: WINDOW, x: i32, y: i32) {
        ncurses::mvwaddstr(win, y + 1, x, ".");
        ncurses::mvwaddstr(win, y + 3, x, ".");
    }

    pub fn draw_time(&self, win: WINDOW, hour: u32, minute: u32, second: u32) {
        let (x, y) = self.get_position();
        let w = self.digit_width;
        let (h, ampm) = self.format_hour(hour);

        self.draw_digit(win, (h / 10) as usize, x + 0 * w, y);
        self.draw_digit(win, (h % 10) as usize, x + 1 * w, y);

        self.draw_colon(win, x + 2 * w, y);

        self.draw_digit(win, (minute / 10) as usize, x + 3 * w, y);
        self.draw_digit(win, (minute % 10) as usize, x + 4 * w, y);

        self.draw_colon(win, x + 5 * w, y);

        self.draw_digit(win, (second / 10) as usize, x + 6 * w, y);
        self.draw_digit(win, (second % 10) as usize, x + 7 * w, y);

        if let Some(ampm) = ampm {
            ncurses::mvwaddstr(win, y + 6, x + 8 * w, ampm);
        }
    }
}
