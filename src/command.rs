use std::sync::mpsc::Sender;

use crossterm::event::{self, KeyEvent, KeyModifiers};

use crate::{api, innerpipe::PipeObj, iterate_config, Config};

pub struct CommandProcessor {
    command_mode: bool,
    command: String,
    unknown_cmd: bool,
    cmdbar_show: String,
    cmdbar_prompt: String,
    cmdbar_suggestion: String,
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self {
            command_mode: true,
            command: String::new(),
            unknown_cmd: false,
            cmdbar_show: String::new(),
            cmdbar_prompt: String::from("New workspace"),
            cmdbar_suggestion: String::new(),
        }
    }

    pub fn get_show(&self) -> &String {
        &self.cmdbar_show
    }

    pub fn get_prompt(&self) -> &String {
        &self.cmdbar_prompt
    }

    pub fn get_suggestion(&self) -> &String {
        &self.cmdbar_suggestion
    }

    pub fn unknown(&self) -> bool {
        self.unknown_cmd
    }

    pub fn is_command_mode(&self) -> bool {
        self.command_mode || self.command.starts_with("C-") || self.command.starts_with("A-")
    }

    pub fn process(
        &mut self,
        map: &Config,
        screen_size: (u16, u16),
        sender: Sender<PipeObj>,
    ) -> bool {
        if let Some((name, cmd, full)) = iterate_config(map, &self.command.trim()) {
            self.cmdbar_prompt = format!("{}", name);
            if !full {
                let mut s = cmd.as_str().unwrap().to_string();
                for _ in self.command.chars() {
                    s.remove(0);
                }
                if self.command.is_empty() {
                    self.cmdbar_suggestion.clear();
                    self.cmdbar_prompt.clear();
                } else {
                    self.cmdbar_suggestion = s;
                }
            } else {
                self.cmdbar_suggestion.clear();
            }
            self.unknown_cmd = false;
            if full {
                self.command.clear();
                match name.as_str() {
                    "quit" => return true,
                    "new-window" => {
                        let win = api::new_window(screen_size);
                        sender.send(PipeObj::NewWindow(win)).unwrap();
                        self.command_mode = false;
                    }
                    "edit-mode" => {
                        self.command_mode = false;
                    }
                    "command-mode" => {
                        self.command_mode = true;
                    }
                    _ => (),
                }
            }
        } else if !self.cmdbar_show.is_empty() {
            self.cmdbar_prompt = String::from("Unknown command");
            self.cmdbar_suggestion.clear();
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
                if self.command_mode
                    || modifiers.contains(KeyModifiers::CONTROL)
                    || modifiers.contains(KeyModifiers::ALT)
                {
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
