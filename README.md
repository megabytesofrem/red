# red editor
Rust text editor using ncurses (pancurses crate).

## Planned Features
- [ ] Insert mode toggle with ESC + -i
- [ ] Cursor mode toggle with ESC + -c
- [x] Scrolling when in cursor mode, using arrow keys; support for `vi/vim` style `hjkl` keys will be considered later on.
- [ ] Ability to save and open files within the editor, using ESC + -o and ESC + -s.
- [ ] Command line parameters
- [ ] Ability to open multiple files side by side in what `vi/vim` and `emacs` call "buffers". Wouldn't be too hard to add, since we are using curses for the editor which supports this feature with `newwin`.

## Usage
Run `cargo run`.
