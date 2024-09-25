use crate::window::Window;

pub enum PipeObj {
    NewWindow(Box<dyn Window>),
}
