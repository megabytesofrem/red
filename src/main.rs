// red editor
// Copyright (c) animojis 2019-forever

mod editor;

use editor::{Editor, EditorMode};
use pancurses::{Input, Window};

fn main() {
    let window = pancurses::initscr();
    let size = window.get_max_yx();

    // Create a subwindow to hold the contents
    let content_window = window
        .subwin(size.0, size.1, 0, 0)
        .expect("Error creating scroll_window");

    content_window.keypad(true);
    content_window.scrollok(true);
    content_window.setscrreg(0, size.0);

    let mut editor = Editor::new(window, content_window);

    // pancurses::raw();
    pancurses::noecho();

    'main: loop {
        let content_win = editor.get_content_window();

        match content_win.getch() {
            Some(Input::Character('\u{1b}')) => {
                // Exit to command mode
                match editor.get_curr_mode() {
                    // Already in command mode, don't have to do anything
                    EditorMode::Command => (),

                    // Set the current mode to the command mode when we hit ESC
                    _ => {
                        editor.set_curr_mode(EditorMode::Command);
                    }
                };
            }
            Some(Input::KeyBackspace) => {
                // Move to previous char and delete it
                content_win.mv(content_win.get_cur_y(), content_win.get_cur_x() - 1);
                content_win.delch();
                content_win.refresh();
            }
            Some(Input::KeyUp) => {
                // Scroll the content window up
                content_win.mvwin(content_win.get_cur_y() - 1, content_win.get_cur_x());
                content_win.mv(0, 0);
                content_win.refresh();
            }
            Some(Input::Character(c)) => {

                if (editor.is_command_mode()) {
                    // We are now in command mode
                }
                content_win.addch(c);
            }
            Some(_) => (),
            None => (),
        }
    }

    pancurses::endwin();
}
