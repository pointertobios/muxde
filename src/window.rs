use std::io::{self, Stdout};

pub trait Window {
    fn get_pos(&self) -> (u16, u16);
    fn get_size(&self) -> (u16, u16);
    fn get_name(&self) -> &str;
    fn get_id(&self) -> usize;

    fn render(&mut self, stdout: &mut Stdout, screen_size: (u16, u16)) -> io::Result<()>;
}
