use std::{
    collections::HashMap,
    io::{self, stdout, Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor, event, queue,
    style::{self, Color},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};

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
                    event::Event::Key(key_event) => match key_event.code {
                        event::KeyCode::Backspace => {
                            if self.command_mode {
                                self.command.pop();
                                self.cmdbar_show.clone_from(&self.command);
                            }
                        }
                        event::KeyCode::Enter => (),
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
                                self.command.push(c);
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
                        event::KeyCode::Media(media_key_code) => (),
                        event::KeyCode::Modifier(modifier_key_code) => (),
                    },
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
                            "print" => queue!(
                                self.stdout,
                                style::SetBackgroundColor(self.colortheme.background),
                                cursor::SavePosition,
                                cursor::MoveTo(0, 0),
                                style::Print("Hello"),
                                cursor::RestorePosition,
                            )?,
                            _ => (),
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
            if cursor::position().unwrap().0 >= self.size.0 - 1 {
                break;
            }
        }
        queue!(
            self.stdout,
            style::SetBackgroundColor(self.colortheme.command_bar),
            cursor::MoveTo(0, self.size.1 - 1),
            style::Print(show),
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
