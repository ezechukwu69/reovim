use std::{
    borrow::Borrow,
    fs::OpenOptions,
    io::{stdout, BufRead, BufReader, Cursor},
};

use anyhow::{Error, Result};
use crossterm::{
    cursor::{self, SetCursorStyle},
    event::KeyEvent,
    execute,
    terminal::{self, Clear},
};

use crate::components::{component::Component, status_line::StatusLine};

use super::mode::{InsertEnterMode, Mode};

pub struct Buffer {
    pub terminal_size: (u16, u16),
    pub cursor_position: (u16, u16),
    pub offset: u16,
    pub lines: Vec<String>,
    pub file_name: String,
    pub insert_enter_mode: InsertEnterMode,
    pub mode: Mode,
}

impl Buffer {
    pub fn new(file_name: String) -> anyhow::Result<Self, anyhow::Error> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(file_name.to_owned())?;
        let reader = BufReader::new(file);
        let lines = reader.lines().filter_map(Result::ok).collect();

        Ok(Buffer {
            terminal_size: terminal::size()?,
            cursor_position: (0, 0),
            offset: 0,
            lines,
            file_name,
            insert_enter_mode: InsertEnterMode::APPEND,
            mode: Mode::NORMAL,
        })
    }

    pub fn draw(&mut self) -> anyhow::Result<(), Error> {
        execute!(stdout(), Clear(terminal::ClearType::All))?;
        execute!(
            stdout(),
            cursor::MoveTo(self.cursor_position.0, self.cursor_position.1)
        )?;
        let status_line = StatusLine {
            file_name: self.file_name.to_owned(),
            terminal_size: self.terminal_size,
            mode: self.mode.clone(),
            file_cursor: (self.cursor_position.0, self.cursor_position.1 + self.offset),
        };
        status_line.draw()?;
        self.render_file()?;
        match self.mode {
            Mode::NORMAL => {
                execute!(stdout(), SetCursorStyle::SteadyBlock)?;
            }
            Mode::INSERT => {
                execute!(stdout(), SetCursorStyle::SteadyBar)?;
            }
        }
        Ok(())
    }

    pub fn handle_key(&mut self, event: KeyEvent) -> anyhow::Result<(), Error> {
        match self.mode {
            Mode::NORMAL => {
                self.handle_normal_mode(event)?;
            }
            Mode::INSERT => {
                self.handle_insert_mode(event)?;
            }
        }
        Ok(())
    }
}
