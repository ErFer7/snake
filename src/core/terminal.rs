use std::io::{Read, Write};

use termion::{
    cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    AsyncReader,
};

use crate::{MINIMUM_HEIGHT, MINIMUM_WIDTH};

pub struct Terminal {
    width: u16,
    height: u16,
    stdin: AsyncReader,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn new() -> Terminal {
        let terminal_size = termion::terminal_size().expect("Failed to get terminal size");
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        let stdin = termion::async_stdin();
        let width = terminal_size.0;
        let height = terminal_size.1;

        if width < MINIMUM_WIDTH || height < MINIMUM_HEIGHT {
            panic!(
                "The terminal size is too small! Minimum size is {}x{}",
                MINIMUM_WIDTH, MINIMUM_HEIGHT
            );
        }

        return Terminal {
            width: terminal_size.0,
            height: terminal_size.1,
            stdin,
            stdout,
        };
    }

    pub fn width(&self) -> u16 {
        return self.width;
    }

    pub fn height(&self) -> u16 {
        return self.height;
    }

    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        self.flush();
    }

    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap();
        self.flush();
    }

    pub fn write(&mut self, text: &str) {
        write!(self.stdout, "{}", text).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        self.flush();
    }

    pub fn get_pressed_key(&mut self) -> Option<Key> {
        let keys = self.stdin.by_ref().keys();

        // ignores all keys except the first one, also ignores errors
        return keys.take(1).next().map(|k| k.unwrap());
    }
}
