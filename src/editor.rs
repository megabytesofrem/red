// red editor
// Copyright (c) animojis 2019-forever

use pancurses::{Input, Window};

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
    pub fn get_content_win(&self) -> &Window {
        &self.content_win
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
}