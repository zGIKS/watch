use std::time::Duration;


/// Result of a color effect: color pair and attribute
#[allow(dead_code)]
pub struct ColorResult {
    pub color: i16,
    pub attr: Option<u32>,
}

/// Trait for color effect strategies
#[allow(dead_code)]
pub trait ColorEffect {
    /// Returns the ncurses color and attribute to use for the given time (in ms)
    fn get_color(&self, time: Duration) -> ColorResult;
}

/// Static color effect (always the same color)
pub struct StaticEffect(pub i16);
impl ColorEffect for StaticEffect {
    fn get_color(&self, _time: Duration) -> ColorResult {
        ColorResult { color: self.0, attr: None }
    }
}

/// Breathe effect (cycles between white and dim white)
pub struct BreatheEffect;
impl ColorEffect for BreatheEffect {
    fn get_color(&self, time: Duration) -> ColorResult {
        let ms = time.as_millis() % 2000;
        if ms < 1000 {
            ColorResult { color: COLOR_WHITE, attr: None }
        } else {
            ColorResult { color: COLOR_WHITE, attr: Some(A_DIM()) }
        }
    }
}

/// Strobe effect (flashes between white and black)
pub struct StrobeEffect;
impl ColorEffect for StrobeEffect {
    fn get_color(&self, time: Duration) -> ColorResult {
        let ms = time.as_millis() % 1000;
        if ms < 500 {
            ColorResult { color: COLOR_WHITE, attr: None }
        } else {
            ColorResult { color: COLOR_BLACK, attr: None }
        }
    }
}

/// Rainbow effect (cycles through all colors)
pub struct RainbowEffect;
impl ColorEffect for RainbowEffect {
    fn get_color(&self, time: Duration) -> ColorResult {
        let colors = [COLOR_RED, COLOR_YELLOW, COLOR_GREEN, COLOR_CYAN, COLOR_BLUE, COLOR_MAGENTA, COLOR_WHITE];
        let idx = ((time.as_millis() / 200) % colors.len() as u128) as usize;
        ColorResult { color: colors[idx], attr: None }
    }
}

/// Pulse effect (quickly flashes white)
pub struct PulseEffect;
impl ColorEffect for PulseEffect {
    fn get_color(&self, time: Duration) -> ColorResult {
        let ms = time.as_millis() % 800;
        if ms < 100 {
            ColorResult { color: COLOR_WHITE, attr: None }
        } else {
            ColorResult { color: COLOR_BLACK, attr: None }
        }
    }
}

/// Factory for color effects
#[allow(dead_code)]
pub fn color_effect_factory(mode: ColorMode) -> Box<dyn ColorEffect> {
    match mode {
        ColorMode::Static => Box::new(StaticEffect(COLOR_WHITE)),
        ColorMode::Breathe => Box::new(BreatheEffect),
        ColorMode::Strobe => Box::new(StrobeEffect),
        ColorMode::Rainbow => Box::new(RainbowEffect),
        ColorMode::Pulse => Box::new(PulseEffect),
    }
}
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorMode {
    Static,
    Breathe,
    Strobe,
    Rainbow,
    Pulse,
}
// colors.rs
// Centralized color management for the TUI clock

use ncurses::*;

// Color pair IDs
pub const COLOR_PAIR_DIGIT: i16 = 1;
pub const COLOR_PAIR_DATE: i16 = 2;

/// Initialize color pairs for the application.
pub fn init_colors() {
    if has_colors() == true {
        start_color();
        // Digits: White on default background
        init_pair(COLOR_PAIR_DIGIT, COLOR_WHITE, -1);
        // Date: White on black background
        init_pair(COLOR_PAIR_DATE, COLOR_WHITE, COLOR_BLACK);
        use_default_colors();
    }
}
