use std::io;

pub struct Buffer {
    name: String,
    path: String,
    content: Vec<String>,
    cursor_pos: (u16, u16),
    focused: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            name: String::from("New Buffer"),
            path: String::new(),
            content: Vec::new(),
            cursor_pos: (0, 0),
            focused: true,
        }
    }

    pub fn focuse(&mut self) {
        self.focused = true;
    }

    pub fn disfocuse(&mut self) {
        self.focused = false;
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }
}
