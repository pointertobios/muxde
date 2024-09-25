use std::{
    collections::HashMap,
    io::{self, stdout, Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, KeyEvent, KeyModifiers},
    queue,
    style::{self, Color},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};

use unicode_width::UnicodeWidthStr;

use crate::{colortheme::ColorTheme, iterate_config, window::Window, Config};

pub struct Application {
    config: Config,
    colortheme: ColorTheme,
    stdout: Stdout,
    size: (u16, u16),
    windows: HashMap<usize, Window>,
    command_mode: bool,
    command: String,
    cmdbar_show: String,
    cmdbar_prompt: String,
}

impl Application {
    pub fn new(config: Config) -> Result<Self, io::Error> {
        queue!(
            stdout(),
            event::EnableMouseCapture,
            terminal::EnterAlternateScreen,
            terminal::Clear(ClearType::All),
            style::SetBackgroundColor(Color::Black)
        )?;
        enable_raw_mode()?;
        stdout().flush()?;
        Ok(Application {
            colortheme: ColorTheme::from(config.get("colortheme").unwrap().as_table().unwrap()),
            config,
            stdout: stdout(),
            size: terminal::size()?,
            windows: HashMap::new(),
            command_mode: true,
            command: String::new(),
            cmdbar_show: String::new(),
	    cmdbar_prompt: String::from("New workspace"),
        })
    }

    pub fn exec(&mut self) -> io::Result<()> {
        self.render()?;
        self.stdout.flush()?;
        loop {
            if event::poll(Duration::from_secs(5))? {
                match event::read()? {
                    event::Event::FocusGained => queue!(self.stdout, cursor::Show)?,
                    event::Event::FocusLost => queue!(self.stdout, cursor::Hide)?,
                    event::Event::Key(key_event) => {
                        let KeyEvent {
                            code, modifiers, ..
                        } = key_event;
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
                    event::Event::Mouse(mouse_event) => (),
                    event::Event::Paste(_) => (),
                    event::Event::Resize(w, h) => self.size = (w, h),
                }
                self.render()?;
                let f = |_, value| {
                    if let &toml::Value::String(ref s) = value {
                        s == &self.command
                    } else {
                        false
                    }
                };
                if let toml::Value::Table(map) = &self.config.get("command").unwrap() {
                    if let Some((name, _)) = iterate_config(map, &f) {
                        match name.as_str() {
                            "quit" => break,
                            _ => self.cmdbar_prompt = String::from("Unknown command"),
                        }
                        self.command.clear();
                    }
                }
                self.stdout.flush()?;
            } else {
            }
        }
        self.render()?;
        self.stdout.flush()?;
        thread::sleep(Duration::from_millis(200));
        Ok(())
    }

    fn render(&mut self) -> io::Result<()> {
        let mut show = String::new();
        for c in self.cmdbar_show.chars() {
            show.push(c);
            if UnicodeWidthStr::width(show.as_str()) as u16 >= self.size.0 - 1 {
                break;
            }
        }
	let x = cursor::position().unwrap().0;
	let mut prompt = String::new();
	for c in self.cmdbar_prompt.chars() {
	    show.push(c);
	    if UnicodeWidthStr::width(prompt.as_str()) as u16 >= self.size.0 - 1 - x {
		break;
	    }
	}
        queue!(
            self.stdout,
            style::SetBackgroundColor(self.colortheme.command_bar),
            cursor::MoveTo(0, self.size.1 - 1),
            style::Print(show),
	    style::SetForegroudColor(self.colortheme.cmdbar_prompt),
	    style::Print(prompt),
        )?;
        let x = cursor::position().unwrap().0;
        queue!(
            self.stdout,
            style::Print(" ".repeat((self.size.0 - x) as usize)),
            cursor::MoveTo(x, self.size.1 - 1),
        )?;
        Ok(())
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        queue!(
            self.stdout,
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture,
        )
        .unwrap();
    }
}
