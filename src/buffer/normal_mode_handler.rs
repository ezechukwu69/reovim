use std::io::{stdout, Cursor};

use anyhow::Error;
use crossterm::{
    event::{KeyCode, KeyEvent},
    execute,
    terminal::Clear,
};

use crate::logger::logger::Logger;

use super::{
    buffer::Buffer,
    mode::{InsertEnterMode, Mode},
};

impl Buffer {
    // HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
    pub fn handle_normal_mode(&mut self, event: KeyEvent) -> anyhow::Result<(), Error> {
        match event.code {
            KeyCode::Char('h') => {
                self.move_left()?;
            }
            KeyCode::Char('j') => {
                self.move_down()?;
            }
            KeyCode::Char('k') => {
                self.move_up()?;
            }
            KeyCode::Char('l') => {
                self.move_right()?;
            }
            KeyCode::Char('i') => {
                self.mode = Mode::INSERT;
                self.insert_enter_mode = InsertEnterMode::INSERT;
            }
            KeyCode::Char('a') => {
                let line = &self.lines[self.get_absolute_position() as usize];
                if line.len() < (self.cursor_position.0 + 1) as usize {
                    self.cursor_position.0 = line.len() as u16;
                } else {
                    self.cursor_position.0 = self.cursor_position.0 + 1;
                }
                self.mode = Mode::INSERT;
                self.insert_enter_mode = InsertEnterMode::APPEND;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_absolute_position(&self) -> u16 {
        return self.cursor_position.1 + self.offset;
    }

    fn move_down(&mut self) -> anyhow::Result<(), Error> {
        if self.get_absolute_position() >= self.lines.len() as u16 {
            return Ok(());
        }
        if self.cursor_position.1 + 1 >= self.terminal_size.1 - 2 {
            execute!(stdout(), Clear(crossterm::terminal::ClearType::All))?;
            self.offset += 1;
        } else {
            self.cursor_position.1 += 1;
        }
        let line = &self.lines[self.get_absolute_position() as usize];
        if line.len() < (self.cursor_position.0 + 1) as usize {
            self.cursor_position.0 = line.len() as u16;
        }
        Ok(())
    }

    fn move_right(&mut self) -> anyhow::Result<(), Error> {
        let line = &self.lines[self.get_absolute_position() as usize];
        if line.len() < (self.cursor_position.0 + 1) as usize {
            return Ok(());
        } else {
            self.cursor_position.0 += 1
        }
        Ok(())
    }

    fn move_left(&mut self) -> anyhow::Result<(), Error> {
        self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
        Ok(())
    }

    fn move_up(&mut self) -> anyhow::Result<(), Error> {
        if self.offset > 0 && self.cursor_position.1 == 0 {
            self.offset = self.offset.saturating_sub(1);
        } else {
            self.cursor_position.1 = self.cursor_position.1.saturating_sub(1);
        }
        let line = &self.lines[self.get_absolute_position() as usize];
        if line.len() < (self.cursor_position.0 + 1) as usize {
            self.cursor_position.0 = line.len() as u16;
        }
        Ok(())
    }
}
