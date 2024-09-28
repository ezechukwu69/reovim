use std::io::{stdout, Write};

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
};

use crate::buffer::mode::Mode;

use super::component::Component;

pub struct StatusLine {
    pub file_name: String,
    pub mode: Mode,
    pub file_cursor: (u16, u16),
    pub terminal_size: (u16, u16),
}

impl Component for StatusLine {
    fn draw(&self) -> anyhow::Result<(), anyhow::Error> {
        let region = format!(" {}:{} ", self.file_cursor.0, self.file_cursor.1);
        let mode = format!(" {:?} ", self.mode);
        let space_width = self.terminal_size.0
            - mode.len() as u16
            - region.len() as u16
            - (self.file_name.len() + 2) as u16;
        let file_name = format!(
            " {} {:<width$}",
            self.file_name,
            " ",
            width = space_width as usize
        );
        let saved_cursor_position = self.file_cursor;
        execute!(stdout(), cursor::MoveTo(0, self.terminal_size.1 - 2))?;
        queue!(
            stdout(),
            style::PrintStyledContent(
                mode.with(style::Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255
                })
                .on(style::Color::Rgb {
                    r: 67,
                    g: 176,
                    b: 230
                })
            )
        )?;
        queue!(
            stdout(),
            style::PrintStyledContent(
                file_name
                    .with(style::Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255
                    })
                    .on(style::Color::Rgb {
                        r: 36,
                        g: 39,
                        b: 45
                    })
            )
        )?;
        queue!(
            stdout(),
            style::PrintStyledContent(region.with(style::Color::Rgb { r: 0, g: 0, b: 0 }).on(
                style::Color::Rgb {
                    r: 240,
                    g: 113,
                    b: 120
                }
            ))
        )?;
        stdout().flush()?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        Ok(())
    }
}
