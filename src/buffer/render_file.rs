use std::io::stdout;

use anyhow::Error;
use crossterm::{cursor, execute, style::Print};

use crate::logger::logger::Logger;

use super::buffer::Buffer;

impl Buffer {
    pub fn render_file(&mut self) -> anyhow::Result<(), Error> {
        let mut file_cursor = (0u16, 0u16);
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        let start = self.offset as usize;
        let end = self.get_end() as usize;
        let cloned_lines = &self.lines[start..end];
        for l in cloned_lines {
            execute!(stdout(), Print(format!("{}", l)))?;
            file_cursor.1 += 1;
            execute!(stdout(), cursor::MoveTo(file_cursor.0, file_cursor.1))?;
        }
        execute!(
            stdout(),
            cursor::MoveTo(self.cursor_position.0, self.cursor_position.1)
        )?;
        Ok(())
    }

    fn get_end(&self) -> u16 {
        let mut end: u16 = 0;
        if (self.offset + (self.terminal_size.1 - 2)) as usize >= self.lines.len() {
            end = self.offset + (self.lines.len() - self.offset as usize) as u16;
        } else {
            end = self.offset + self.terminal_size.1 - 2;
        }
        return end;
    }
}
