use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};

use super::{
    buffer::Buffer,
    mode::{InsertEnterMode, Mode},
};

impl Buffer {
    // HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
    pub fn handle_insert_mode(&mut self, event: KeyEvent) -> anyhow::Result<(), Error> {
        match event.code {
            KeyCode::Esc => {
                match self.insert_enter_mode {
                    InsertEnterMode::APPEND => {
                        self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
                    }
                    InsertEnterMode::INSERT => {}
                }
                self.mode = Mode::NORMAL;
            }
            KeyCode::Char(any_char) => {
                let index_of_file = self.get_absolute_position() as usize;
                let line = &mut self.lines[index_of_file];
                line.insert(self.cursor_position.0 as usize, any_char);
                self.cursor_position.0 += 1;
            }
            KeyCode::Backspace => {
                let index_of_file = self.get_absolute_position() as usize;
                let line = self.lines[index_of_file].to_string();
                if line.len() > 0 {
                    self.lines.remove(self.cursor_position.0 as usize - 1);
                    self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
                } else {
                    self.lines.remove(index_of_file);
                    if index_of_file > 0 {
                        let new_line = &self.lines[index_of_file - 1];
                        self.cursor_position.0 = new_line.len() as u16;
                        if self.cursor_position.1 > 0 {
                            self.cursor_position.1 = self.cursor_position.1.saturating_sub(1);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
