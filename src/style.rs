/// Returns a formatted date string for the given chrono::DateTime.
pub fn format_date<T>(dt: &chrono::DateTime<T>) -> String
where
    T: chrono::TimeZone,
    T::Offset: std::fmt::Display,
{
    dt.format("%A, %d %B %Y").to_string()
}
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HourFormat {
    H24,
    H12,
}
// style.rs
// List of available digit styles for the clock

pub const DIGIT_STYLES: [&str; 8] = [
    "█", "▓", "▒", "░", "▀", "▄", "▌", "▐"
];
