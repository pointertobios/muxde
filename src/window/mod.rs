pub mod editor;

use std::io::{self, Stdout};

use crate::colortheme::ColorTheme;

pub trait Window {
    fn get_pos(&self) -> (u16, u16);
    fn get_size(&self) -> (u16, u16);
    fn get_id(&self) -> usize;

    fn set_pos(&mut self, pos: (u16, u16));
    fn set_size(&mut self, size: (u16, u16));

    fn render(
        &mut self,
        stdout: &mut Stdout,
        screen_size: (u16, u16),
        colortheme: &ColorTheme,
    ) -> io::Result<()>;
}
