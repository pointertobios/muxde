use crossterm::event::{self, KeyEvent, KeyModifiers};

use crate::{iterate_config, Config};

pub struct CommandProcessor {
    command_mode: bool,
    command: String,
    unknown_cmd: bool,
    cmdbar_show: String,
    cmdbar_prompt: String,
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self {
            command_mode: true,
            command: String::new(),
            unknown_cmd: false,
            cmdbar_show: String::new(),
            cmdbar_prompt: String::from("New workspace"),
        }
    }

    pub fn get_show(&self) -> &String {
        &self.cmdbar_show
    }

    pub fn get_prompt(&self) -> &String {
        &self.cmdbar_prompt
    }

    pub fn unknown(&self) -> bool {
        self.unknown_cmd
    }

    pub fn is_command_mode(&self) -> bool {
        self.command_mode
    }

    pub fn process(&mut self, map: &Config) -> bool {
        let f = |_, value| {
            if let &toml::Value::String(ref s) = value {
                s == &self.command
            } else {
                false
            }
        };
        if let Some((name, _)) = iterate_config(map, &f) {
            self.command.clear();
            self.cmdbar_prompt = format!(" - {}", name);
            self.unknown_cmd = false;
            match name.as_str() {
                "quit" => return true,
                _ => (),
            }
        } else if !self.cmdbar_show.is_empty() {
            self.cmdbar_prompt = String::from(" | Unknown command");
            self.unknown_cmd = true;
        } else {
            self.cmdbar_prompt.clear();
        }
        false
    }

    pub fn key_event(&mut self, key: KeyEvent) {
        let KeyEvent {
            code, modifiers, ..
        } = key;
        match code {
            event::KeyCode::Backspace => {
                if self.command_mode {
                    self.command.pop();
                    self.cmdbar_show.clone_from(&self.command);
                }
            }
            event::KeyCode::Enter => {
                if self.command_mode {
                    self.command.clear();
                    self.cmdbar_show.clear();
                }
            }
            event::KeyCode::Left => (),
            event::KeyCode::Right => (),
            event::KeyCode::Up => (),
            event::KeyCode::Down => (),
            event::KeyCode::Home => (),
            event::KeyCode::End => (),
            event::KeyCode::PageUp => (),
            event::KeyCode::PageDown => (),
            event::KeyCode::Tab => (),
            event::KeyCode::BackTab => (),
            event::KeyCode::Delete => (),
            event::KeyCode::Insert => (),
            event::KeyCode::F(_) => (),
            event::KeyCode::Char(c) => {
                if self.command_mode {
                    let mut b = true;
                    if modifiers.contains(KeyModifiers::CONTROL) {
                        self.command += "C-";
                    } else if modifiers.contains(KeyModifiers::ALT) {
                        self.command += "A-";
                    } else {
                        b = false;
                    }
                    self.command.push(c);
                    if b {
                        self.command.push(' ');
                    }
                    self.cmdbar_show.clone_from(&self.command);
                }
            }
            event::KeyCode::Null => (),
            event::KeyCode::Esc => (),
            event::KeyCode::CapsLock => (),
            event::KeyCode::ScrollLock => (),
            event::KeyCode::NumLock => (),
            event::KeyCode::PrintScreen => (),
            event::KeyCode::Pause => (),
            event::KeyCode::Menu => (),
            event::KeyCode::KeypadBegin => (),
            event::KeyCode::Media(media) => (),
            event::KeyCode::Modifier(modifier) => (),
        }
    }
}
