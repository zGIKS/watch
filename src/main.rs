extern crate ncurses;
use ncurses::*;
use chrono::Local;
use chrono::Timelike;

// Matriz corregida: cada número es 3x5 (3 columnas, 5 filas)
pub const NUMBER: [[bool; 15]; 10] = [
    // 0: █████
    //    █   █
    //    █   █
    //    █   █
    //    █████
    [true,true,true,true,false,true,true,false,true,true,false,true,true,true,true],  // 0
    
    // 1:   █
    //      █
    //      █
    //      █
    //      █
    [false,true,false,false,true,false,false,true,false,false,true,false,false,true,false],  // 1
    
    // 2: █████
    //        █
    //    █████
    //    █
    //    █████
    [true,true,true,false,false,true,true,true,true,true,false,false,true,true,true],  // 2
    
    // 3: █████
    //        █
    //    █████
    //        █
    //    █████
    [true,true,true,false,false,true,true,true,true,false,false,true,true,true,true],  // 3
    
    // 4: █   █
    //    █   █
    //    █████
    //        █
    //        █
    [true,false,true,true,false,true,true,true,true,false,false,true,false,false,true],  // 4
    
    // 5: █████
    //    █
    //    █████
    //        █
    //    █████
    [true,true,true,true,false,false,true,true,true,false,false,true,true,true,true],  // 5
    
    // 6: █████
    //    █
    //    █████
    //    █   █
    //    █████
    [true,true,true,true,false,false,true,true,true,true,false,true,true,true,true],  // 6
    
    // 7: █████
    //        █
    //        █
    //        █
    //        █
    [true,true,true,false,false,true,false,false,true,false,false,true,false,false,true],  // 7
    
    // 8: █████
    //    █   █
    //    █████
    //    █   █
    //    █████
    [true,true,true,true,false,true,true,true,true,true,false,true,true,true,true],  // 8
    
    // 9: █████
    //    █   █
    //    █████
    //        █
    //    █████
    [true,true,true,true,false,true,true,true,true,false,false,true,true,true,true],  // 9
];

fn draw_number(win: WINDOW, number: usize, x: i32, y: i32, ch: chtype) {
    let matrix = &NUMBER[number];
    for i in 0..15 {
        if matrix[i] {
            let dx = (i % 3) as i32;  // 3 columnas
            let dy = (i / 3) as i32;  // 5 filas
            mvwaddch(win, y + dy, x + dx, ch);
        }
    }
}

fn main() {
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
        let digit_width = 5;  // 3 columnas + 2 espacios para más aire
        let ch = 'X' as chtype; // Bloque minimalista

        // Dibujar HH:MM:SS minimalista
        draw_number(stdscr(), (hour / 10) as usize, x + 0 * digit_width, y, ch);
        draw_number(stdscr(), (hour % 10) as usize, x + 1 * digit_width, y, ch);

        // Separador : (dos puntos minimalista)
        let colon_x = x + 2 * digit_width;
        mvwaddch(stdscr(), y + 1, colon_x, '.'.into());
        mvwaddch(stdscr(), y + 3, colon_x, '.'.into());

        draw_number(stdscr(), (minute / 10) as usize, x + 3 * digit_width, y, ch);
        draw_number(stdscr(), (minute % 10) as usize, x + 4 * digit_width, y, ch);

        // Segundo separador :
        let colon2_x = x + 5 * digit_width;
        mvwaddch(stdscr(), y + 1, colon2_x, '.'.into());
        mvwaddch(stdscr(), y + 3, colon2_x, '.'.into());

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