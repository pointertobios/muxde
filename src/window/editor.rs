use std::{
    collections::HashMap,
    io::{self, Stdout},
};

use crossterm::{cursor, queue, style};
use unicode_width::UnicodeWidthStr;

use crate::{buffer::Buffer, colortheme::ColorTheme, window::Window};

pub struct Editor {
    id: usize,
    name: String,
    pos: (u16, u16),
    size: (u16, u16),
    buffers: HashMap<usize, Buffer>,
}

impl Editor {
    pub fn new(id: usize, screen_size: (u16, u16)) -> Self {
        Editor {
            id,
            name: String::new(),
            pos: (0, 0),
            size: screen_size,
            buffers: HashMap::new(),
        }
    }
}

impl Window for Editor {
    fn get_pos(&self) -> (u16, u16) {
        self.pos
    }

    fn get_size(&self) -> (u16, u16) {
        self.size
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn render(
        &mut self,
        stdout: &mut Stdout,
        screen_size: (u16, u16),
        colortheme: &ColorTheme,
    ) -> io::Result<()> {
        let x = UnicodeWidthStr::width(self.name.as_str()) as u16;
        queue!(
            stdout,
            cursor::SavePosition,
            style::SetBackgroundColor(colortheme.editor_title),
            cursor::MoveTo(self.pos.0, self.pos.1),
            style::Print(self.name.clone()),
            style::Print(" ".repeat((screen_size.0 - x) as usize) + "\n\r"),
            style::SetBackgroundColor(colortheme.background),
            cursor::RestorePosition,
        )?;
        Ok(())
    }
}
