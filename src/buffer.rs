pub struct Buffer {
    content: Vec<String>,
    cursor_pos: (u16, u16),
    focused: bool,
}
