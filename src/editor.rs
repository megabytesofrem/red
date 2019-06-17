// red editor
// Copyright (c) animojis 2019-forever

extern crate pancurses;
use pancurses::{Window, Input};

pub enum EditorMode {
    Insert,
    Cursor,
}

pub struct Editor {
    window: Window,
    curr_mode: EditorMode
}

impl Editor {

    /// Creates a new Editor
    ///
    /// ## Arguments
    /// - `window` - A pancurses window to use
    ///
    pub fn new(window: Window) -> Editor {
        Editor { window: window, curr_mode: EditorMode::Insert }
    }

    /// Checks if the current mode allows editing or not
    /// 
    /// Returns `true` if the current mode allows editing, `false`
    /// if it doesn't allow editing.
    pub fn can_edit(&self) -> bool {
        match &self.curr_mode {
            EditorMode::Insert => true,
            EditorMode::Cursor => false
        }
    }

    /// Getter for the `pancurses` window
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    /// Getter for the current editor mode. Can be one of the following
    /// 
    /// - `EditorMode::Insert` - Editor is insert mode, text can be inserted
    /// - `EditorMode::Cursor` - Editor is in cursor mode, arrow-keys navigation
    pub fn get_curr_mode(&self) -> &EditorMode {
        &self.curr_mode
    }
}