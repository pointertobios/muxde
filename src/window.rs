use std::io;

pub trait Window {
    fn get_pos(&self) -> (u16, u16);
    fn get_size(&self) -> (u16, u16);
    fn get_id(&self) -> usize;

    fn render(&mut self, screen_size: (u16, u16)) -> io::Result<()>;
}
