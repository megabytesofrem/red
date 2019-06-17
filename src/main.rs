// red editor
// Copyright (c) animojis 2019-forever

mod editor;
extern crate pancurses;

use editor::{Editor, EditorMode};
use pancurses::{Window, Input};

fn main() {
    let window = pancurses::initscr();
    let size = window.get_max_yx();

    window.printw(format!("{:?}", size));

    // Create a subwindow to hold the contents
    let scroll_window = window.subwin(size.0, size.1, 0, 0)
        .expect("Error creating scroll_window");
    
    scroll_window.scrollok(true);
    scroll_window.setscrreg(0, size.0);

    // Use the subwindow for the editor
    let editor = Editor::new(scroll_window);

    pancurses::noecho();

    loop {
        let window = editor.get_window();

        if let Some(c) = window.getch() {
            match c {
                Input::Character('\u{7f}') => {
                    // Move to the previous character and delete it
                    window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                    window.delch();
                    window.refresh();
                },
                Input::Character(c) => { window.addch(c); },
                _ => ()
            }
        }
    }

    pancurses::endwin();
}
