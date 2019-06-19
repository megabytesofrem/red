// red editor
// Copyright (c) animojis 2019-forever

use pancurses::{Input, Window};
use Input::*;

#[derive(PartialEq, Copy, Clone)]
pub enum EditorMode {
    Command,
    Insert,
    Cursor,
}

pub struct Editor {
    win: Window,
    content_win: Window,
    curr_mode: EditorMode,

    text_buf: String,
    command_buf: String,
}

impl Editor {

    /// Creates a new Editor
    ///
    /// # Arguments
    /// - `window` - The main window to use
    /// - `content_win` - The content sub-window to use
    ///
    pub fn new(window: Window, content_win: Window) -> Editor {
        Editor {
            win: window,
            content_win: content_win,
            curr_mode: EditorMode::Insert,
            text_buf: "".to_owned(),
            command_buf: "".to_owned(),
        }
    }

    /// Checks if the current mode allows editing or not
    ///
    /// Returns `true` if the current mode allows editing, `false`
    /// if it doesn't allow editing.
    pub fn can_edit(&self) -> bool {
        match self.curr_mode {
            EditorMode::Command => true, // true, but not to text buffer
            EditorMode::Insert => true,
            EditorMode::Cursor => false,
        }
    }

    /// Checks if the editor is in command mode or not.
    pub fn is_command_mode(&self) -> bool {
        self.get_curr_mode() == EditorMode::Command
    }

    /// Gets the main window associated with the editor
    ///
    /// This is not the content window, and cannot be scrolled.
    pub fn get_win(&self) -> &Window {
        &self.win
    }

    /// Gets the content window associated with the editor
    ///
    /// Unlike the main window (which can be got using `get_win`),
    /// the content window supports scrolling when the content is larger
    /// than itself.
    pub fn get_content_win(&mut self) -> &Window {
        &mut self.content_win
    }

    /// Gets the current editor mode. Can be one of the following
    ///
    /// - `EditorMode::Command`- Editor is in command mode, text is interpreted as commands
    /// - `EditorMode::Insert` - Editor is insert mode, text can be inserted
    /// - `EditorMode::Cursor` - Editor is in cursor mode, arrow-keys navigation
    ///
    /// In command mode, although editing is allowed any text that is inputted is not sent to
    /// the text buffer and is instead treated as a command and handled as such.
    pub fn get_curr_mode(&self) -> EditorMode {
        self.curr_mode
    }

    /// Sets the current editor mode. Can be one of the following
    ///
    /// - `EditorMode::Command`- Editor is in command mode, text is interpreted as commands
    /// - `EditorMode::Insert` - Editor is insert mode, text can be inserted
    /// - `EditorMode::Cursor` - Editor is in cursor mode, arrow-keys navigation
    pub fn set_curr_mode(&mut self, mode: EditorMode) {
        self.curr_mode = mode;

        if mode == EditorMode::Command {
            // Display the prompt if we are in command mode
            let maxy = self.get_content_win().get_max_y();
            self.get_content_win().mvaddstr(maxy - 1, 0, ":");
            self.get_content_win().refresh();
        }
    }

    /// Gets the contents of the text buffer
    ///
    /// This buffer is used for text content in the file, and is seperate
    /// than the command buffer.
    pub fn get_text_buf(&mut self) -> &mut String {
        &mut self.text_buf
    }

    /// Gets the contents of the command buffer
    ///
    /// This buffer is seperate from the text buffer (see `get_text_buf`) and
    /// is only used internally for storing commands to process when in command mode.
    pub fn get_command_buf(&mut self) -> &mut String {
        &mut self.command_buf
    }

    /// Handles input for a single character. Depending on whether we 
    /// are in text mode or command mode, we may want to do something different.
    fn handle_char_input(&mut self, c: char) {
        if self.is_command_mode() {
            // In command mode, don't append to text buffer
            self.get_command_buf()
                .push_str(&format!("{}", c));
            
            // Offset first char to be after the prompt
            let win = self.get_content_win();
            let x = win.get_cur_x();
            let max_y = win.get_max_y();
            win.mvaddch(max_y - 1, if x != 0 { x } else { x + 1 }, c);
            win.refresh();
        }
        else {
            // Not in command mode
        }
    }

    /// Handles the backspace key being pressed. On macOS, this is tricky necause
    /// the backspace key does not emit a `Input::KeyBackspace`, but rather `\u7f` instead.
    fn handle_backspace(&mut self) {
        let win = self.get_content_win();
        let x = win.get_cur_x();
        let y = win.get_cur_y();
        
        // Move to the previous char and delete it
        win.mv(y, x - 1);
        win.delch();
        win.refresh();
    }

    /// Handles movement with the cursor keys.
    ///
    /// Left and right movement are only allowed when we are in the cursor mode, however
    /// moving up and down to scroll the contents of the screen is allowed in all modes.
    fn handle_cursor_key(&mut self, key: Input) {
        let win = self.get_content_win();
        let x = win.get_cur_x();
        let y = win.get_cur_y();

        match key {
            KeyUp => {
                // Scroll the content window up
                win.mvwin(y - 1, x);
                win.mv(y - 1, 0);
                win.refresh();
            },
            _ => ()
        }
    }

    /// Handles keyboard input for the editor
    pub fn handle_input(&mut self) {
        //let win = self.get_content_win();

        'input: loop {
            match self.get_content_win().getch() {
                Some(Character('\x1b')) => self.set_curr_mode(EditorMode::Command),
                Some(Character(c)) => self.handle_char_input(c),
                Some(key) if [KeyUp, KeyDown, KeyLeft, KeyRight].contains(&key) => self.handle_cursor_key(key),
                Some(key) if [KeyBackspace, Character('\x7f')].contains(&key) => self.handle_backspace(),
                None => (),
                _ => (),
            }
        }
    }
}