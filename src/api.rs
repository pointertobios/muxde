use once_cell::sync::OnceCell;

use crate::{window::Window, window::editor::Editor};

pub fn new_window(screen_size: (u16, u16)) -> Box<dyn Window> {
    static mut ID_COUNT: OnceCell<usize> = OnceCell::new();
    let tmp = unsafe { *ID_COUNT.get_or_init(|| 0) };
    let _ = unsafe { ID_COUNT.set(tmp + 1) };
    Box::new(Editor::new(tmp, screen_size))
}
