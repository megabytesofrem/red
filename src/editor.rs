// red editor
// Copyright (c) animojis 2019-forever

use pancurses::{Window, Input};

pub enum EditorMode {
    Insert,
    Cursor,
}

pub struct Editor {
    window: Window,
    content_window: Window,  
    curr_mode: EditorMode
}

impl Editor {

    /// Creates a new Editor
    ///
    /// ## Arguments
    /// - `window` - The main window to use
    /// - `content_window` - The content sub-window to use
    ///
    pub fn new(window: Window, content_window: Window) -> Editor {
        Editor { window: window, content_window: content_window, curr_mode: EditorMode::Insert }
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

    /// Gets the main window associated with the editor
    /// 
    /// This is not the content window, and cannot be scrolled.
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    /// Gets the content window associated with the editor
    /// 
    /// Unlike the main window (which can be got using `get_window`),
    /// the content window supports scrolling when the content is larger
    /// than itself.
    pub fn get_content_window(&self) -> &Window {
        &self.content_window
    }

    /// Getter for the current editor mode. Can be one of the following
    /// 
    /// - `EditorMode::Insert` - Editor is insert mode, text can be inserted
    /// - `EditorMode::Cursor` - Editor is in cursor mode, arrow-keys navigation
    pub fn get_curr_mode(&self) -> &EditorMode {
        &self.curr_mode
    }
}