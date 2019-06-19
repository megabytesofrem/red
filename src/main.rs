// red editor
// Copyright (c) animojis 2019-forever

mod editor;

use editor::{Editor, EditorMode};
use pancurses::{Input, Window};
use pancurses::Input::*;

fn main() {
    let window = pancurses::initscr();
    let size = window.get_max_yx();

    // Create a subwindow to hold the contents
    let content_window = window
        .subwin(size.0, size.1, 0, 0)
        .expect("Error creating content_window");

    content_window.keypad(true);
    content_window.nodelay(true);
    content_window.scrollok(true);
    content_window.setscrreg(0, size.0);

    let mut editor = Editor::new(window, content_window);
    pancurses::noecho();

    editor.handle_input();

    pancurses::endwin();
}
