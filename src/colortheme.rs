use crossterm::style::Color;
use toml::{Table, Value};

pub struct ColorTheme {
    pub background: Color,
    pub command_bar: Color,
    pub cmdbar_prompt: Color,
    pub cmdbar_cmdexist: Color,
    pub cmdbar_cmdunexist: Color,
}

impl From<&Table> for ColorTheme {
    fn from(value: &Table) -> Self {
        let f = |name| {
            if let Value::Array(arr) = value.get(name).unwrap() {
                Color::Rgb {
                    r: arr[0].as_integer().unwrap() as u8,
                    g: arr[1].as_integer().unwrap() as u8,
                    b: arr[2].as_integer().unwrap() as u8,
                }
            } else {
                panic!();
            }
        };
        let background = f("background");
        let command_bar = f("command-bar");
        let cmdbar_prompt = f("cmdbar-prompt");
        let cmdbar_cmdexist = f("cmdbar-cmdexist");
        let cmdbar_cmdunexist = f("cmdbar-cmdunexist");
        Self {
            background,
            command_bar,
            cmdbar_prompt,
            cmdbar_cmdexist,
            cmdbar_cmdunexist,
        }
    }
}
