use std::ffi::CString;
use libc;
extern crate ncurses;
use ncurses::*;
use chrono::Local;
use chrono::Timelike;

// Matriz 5x5: cada número ocupa 5 columnas x 5 filas
pub const NUMBER: [[bool; 25]; 10] = [
    // 0
    [true,true,true,true,true,
     true,false,false,false,true,
     true,false,false,false,true,
     true,false,false,false,true,
     true,true,true,true,true],
    // 1
    [false,false,true,false,false,
     false,false,true,false,false,
     false,false,true,false,false,
     false,false,true,false,false,
     false,false,true,false,false],
    // 2
    [true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true,
     true,false,false,false,false,
     true,true,true,true,true],
    // 3
    [true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true],
    // 4
    [true,false,false,false,true,
     true,false,false,false,true,
     true,true,true,true,true,
     false,false,false,false,true,
     false,false,false,false,true],
    // 5
    [true,true,true,true,true,
     true,false,false,false,false,
     true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true],
    // 6
    [true,true,true,true,true,
     true,false,false,false,false,
     true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true],
    // 7
    [true,true,true,true,true,
     false,false,false,false,true,
     false,false,false,false,true,
     false,false,false,false,true,
     false,false,false,false,true],
    // 8
    [true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true],
    // 9
    [true,true,true,true,true,
     true,false,false,false,true,
     true,true,true,true,true,
     false,false,false,false,true,
     true,true,true,true,true],
];

fn draw_number(win: WINDOW, number: usize, x: i32, y: i32, ch: &str) {
    let matrix = &NUMBER[number];
    for i in 0..25 {
        if matrix[i] {
            let dx = (i % 5) as i32;  // 5 columnas
            let dy = (i / 5) as i32;  // 5 filas
            mvwaddstr(win, y + dy, x + dx, ch);
        }
    }
}

fn main() {
    // Inicializar locale para UTF-8
    unsafe {
        let locale = CString::new("").unwrap();
        libc::setlocale(libc::LC_ALL, locale.as_ptr());
    }
    // Iniciar ncurses
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    timeout(100);

    loop {
        // Limpiar pantalla
        erase();

        // Obtener hora actual
        let now = Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        // Coordenadas y configuración
        let y = 2;
        let x = 2;
        let digit_width = 7;  // 5 columnas + 2 espacios para más aire
        let ch = "█";

        // Dibujar HH:MM:SS minimalista
        draw_number(stdscr(), (hour / 10) as usize, x + 0 * digit_width, y, ch);
        draw_number(stdscr(), (hour % 10) as usize, x + 1 * digit_width, y, ch);

        // Separador : (dos puntos minimalista)
        let colon_x = x + 2 * digit_width;
        mvwaddstr(stdscr(), y + 1, colon_x, ".");
        mvwaddstr(stdscr(), y + 3, colon_x, ".");

        draw_number(stdscr(), (minute / 10) as usize, x + 3 * digit_width, y, ch);
        draw_number(stdscr(), (minute % 10) as usize, x + 4 * digit_width, y, ch);

        // Segundo separador :
        let colon2_x = x + 5 * digit_width;
        mvwaddstr(stdscr(), y + 1, colon2_x, ".");
        mvwaddstr(stdscr(), y + 3, colon2_x, ".");

        draw_number(stdscr(), (second / 10) as usize, x + 6 * digit_width, y, ch);
        draw_number(stdscr(), (second % 10) as usize, x + 7 * digit_width, y, ch);

        // Mostrar información de debug (opcional)
        mvprintw(10, 2, &format!("Hora: {:02}:{:02}:{:02}", hour, minute, second));
        mvprintw(11, 2, "Presiona 'q' o ESC para salir");

        refresh();

        // Verificar teclas
        let key = getch();
        if key == 'q' as i32 || key == 27 {
            break;
        }
    }

    endwin();
}