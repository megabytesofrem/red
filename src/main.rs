// red editor
// Copyright (c) animojis 2019-forever

mod editor;

use editor::{Editor, EditorMode};
use pancurses::{Window, Input};

fn main() {
    let window = pancurses::initscr();
    let size = window.get_max_yx();

    window.printw(format!("{:?}", size));

    // Create a subwindow to hold the contents
    let content_window = window.subwin(size.0, size.1, 0, 0)
        .expect("Error creating scroll_window");
    
    content_window.keypad(true);
    content_window.scrollok(true);
    content_window.setscrreg(0, size.0);

    let editor = Editor::new(window, content_window);

    // pancurses::raw();
    pancurses::noecho();

    loop {
        let content_win = editor.get_content_window();

        match content_win.getch() {
            Some(Input::KeyBackspace) => {
                // Move to previous char and delete it
                content_win.mv(content_win.get_cur_y(), content_win.get_cur_x() - 1);
                content_win.delch();
                content_win.refresh();
            },
            Some(Input::Character(c)) => { content_win.addch(c); }
            Some(_) => (),
            None => (),
        }
    }

    pancurses::endwin();
}
