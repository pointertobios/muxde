use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::{self, Stdout},
};

use crossterm::{cursor, queue};

use crate::window::Window;

pub struct Editor {
    name: String,
    pos: (u16, u16),
    size: (u16, u16),
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
        let mut state = DefaultHasher::new();
        self.name.hash(&mut state);
        state.finish() as usize
    }

    fn render(&mut self, stdout: &mut Stdout, screen_size: (u16, u16)) -> io::Result<()> {
        queue!(stdout, cursor::SavePosition, cursor::RestorePosition)?;
        Ok(())
    }
}

impl Hash for Editor {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.name.hash(state)
    }
}
