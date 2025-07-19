## Watch - Terminal Digital Clock

Watch is a modern, highly customizable digital clock for your terminal, inspired by tty-clock and written in Rust using ncurses.

### Features
- Large, clear digital digits (5x5 matrix)
- Multiple block styles: █ ▓ ▒ ░ ▀ ▄ ▌ ▐ (configurable)
- Real-time update (refreshes every 100ms)
- Minimalist and easy to read
- Press 'q' or ESC to exit

### Installation
1. Clone this repository:
   ```sh
   git clone https://github.com/zGIKS/watch.git
   cd watch
   ```
2. Build with Cargo:
   ```sh
   cargo build --release
   ```
3. Run:
   ```sh
   cargo run --release
   ```

#### Requirements
- Rust (edition 2021 or later)
- A terminal with UTF-8 support
- Linux (tested)

### Usage
By default, the clock uses the "█" style. To change the digit style, edit the third argument in `Clock::new(2, 2, style_index)` in `main.rs` (0 for "█", 1 for "▓", etc.).

### File Structure
- `src/main.rs` - Entry point, main loop
- `src/clock.rs` - Clock struct and drawing logic
- `src/digits.rs` - Digit matrix
- `src/style.rs` - List of available digit styles

### Credits
- Developed by zGIKS
