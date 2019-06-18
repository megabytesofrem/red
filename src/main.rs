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
    content_window.nodelay(true);
    content_window.scrollok(true);
    content_window.setscrreg(0, size.0);

    let mut editor = Editor::new(window, content_window);

    // pancurses::raw();
    pancurses::noecho();

    'main: loop {
        match editor.get_content_win().getch() {
            Some(Input::Character('\u{1b}')) => {
                // Exit to command mode
                match editor.get_curr_mode() {
                    // Already in command mode, don't have to do anything
                    EditorMode::Command => (),

                    // Set the current mode to the command mode when we hit ESC
                    _ => {
                        editor.set_curr_mode(EditorMode::Command);

                        let max_y = editor.get_content_win().get_max_y();
                        editor.get_content_win().mvaddstr(max_y - 1, 0, ":");
                        editor.get_content_win().refresh();
                    }
                };
            }
            Some(Input::KeyBackspace) => {
                // Move to previous char and delete it
                let x = editor.get_content_win().get_cur_x();
                let y = editor.get_content_win().get_cur_y();

                editor.get_content_win().mv(y, x - 1);
                editor.get_content_win().delch();
                editor.get_content_win().refresh();
            }
            Some(Input::KeyUp) => {
                // Scroll the content window up
                let x = editor.get_content_win().get_cur_x();
                let y = editor.get_content_win().get_cur_y();

                editor.get_content_win().mvwin(y - 1, x);
                editor.get_content_win().mv(y - 1, 0);
                editor.get_content_win().refresh();
            }
            Some(Input::Character(c)) => {

                if editor.is_command_mode() {
                    let x = editor.get_content_win().get_cur_x();
                    let y = editor.get_content_win().get_cur_y();

                    // Now in command mode, do not append to the text buffer
                    let mut buf = editor.get_command_buf();
                    buf.push_str(&format!("{}", c).to_owned());

                    // Offset the first character to be after the prompt
                    let max_y = editor.get_content_win().get_max_y();
                    editor
                        .get_content_win()
                        .mvaddch(max_y - 1, if x != 0 { x } else { x + 1 }, c);

                    editor.get_content_win().refresh();
                } else {
                    // Append to the text buffer
                    let mut buf = editor.get_text_buf();
                    buf.push_str(&format!("{}", c).to_owned());

                    editor.get_content_win().addch(c);
                }
            }
            Some(_) => (),
            None => (),
        }
    }

    pancurses::endwin();
}
