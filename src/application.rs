use std::{
    collections::HashMap,
    io::{self, stdout, Stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::{
    cursor, event, queue,
    style::{self, Color},
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};

use unicode_width::UnicodeWidthStr;

use crate::{
    colortheme::ColorTheme, command::CommandProcessor, innerpipe::PipeObj, window::Window, Config,
};

pub struct Application {
    config: Config,
    colortheme: ColorTheme,
    stdout: Stdout,
    size: (u16, u16),
    windows: HashMap<usize, Box<dyn Window>>,
    cmdprocessor: CommandProcessor,
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
            cmdprocessor: CommandProcessor::new(),
        })
    }

    pub fn exec(&mut self) -> io::Result<()> {
        self.render()?;
        loop {
            if event::poll(Duration::from_secs(5))? {
                match event::read()? {
                    event::Event::FocusGained => queue!(self.stdout, cursor::Show)?,
                    event::Event::FocusLost => queue!(self.stdout, cursor::Hide)?,
                    event::Event::Key(key_event) => {
                        if self.cmdprocessor.is_command_mode() {
                            self.cmdprocessor.key_event(key_event);
                        }
                    }
                    event::Event::Mouse(mouse_event) => (),
                    event::Event::Paste(_) => (),
                    event::Event::Resize(w, h) => self.size = (w, h),
                }
                if self.command_process() {
                    break;
                }
                self.render()?;
            } else {
            }
        }
        self.render()?;
        thread::sleep(Duration::from_millis(200));
        Ok(())
    }

    fn command_process(&mut self) -> bool {
        if let toml::Value::Table(map) = &self.config.get("command").unwrap() {
            let (tx, rx) = mpsc::channel();
            if self.cmdprocessor.process(map, tx) {
                while let Ok(pipeobj) = rx.recv() {
                    match pipeobj {
                        PipeObj::NewWindow(win) => {
                            self.windows.insert(win.get_id(), win);
                        }
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn render(&mut self) -> io::Result<()> {
        let mut show = String::new();
        for c in self.cmdprocessor.get_show().chars() {
            show.push(c);
            if UnicodeWidthStr::width(show.as_str()) as u16 >= self.size.0 - 1 {
                break;
            }
        }
        let x = UnicodeWidthStr::width(show.as_str()) as u16;
        let mut suggestion = String::new();
        for c in self.cmdprocessor.get_suggestion().chars() {
            suggestion.push(c);
            if UnicodeWidthStr::width(suggestion.as_str()) as u16 >= self.size.0 - 1 - x {
                break;
            }
        }
        let x = x + UnicodeWidthStr::width(suggestion.as_str()) as u16;
        let mut prompt = String::from(if x == 0 { "" } else { " - " });
        for c in self.cmdprocessor.get_prompt().chars() {
            prompt.push(c);
            if UnicodeWidthStr::width(prompt.as_str()) as u16 >= self.size.0 - 1 - x {
                break;
            }
        }
        let x = x + UnicodeWidthStr::width(prompt.as_str()) as u16;
        queue!(
            self.stdout,
            style::SetBackgroundColor(self.colortheme.command_bar),
            cursor::MoveTo(0, self.size.1 - 1),
            style::SetForegroundColor(if self.cmdprocessor.unknown() {
                self.colortheme.cmdbar_cmdunexist
            } else {
                self.colortheme.cmdbar_cmdexist
            }),
            style::Print(show),
            cursor::SavePosition,
            style::SetForegroundColor(self.colortheme.cmdbar_prompt),
            style::Print(suggestion),
            style::Print(prompt),
            style::Print(" ".repeat((self.size.0 - x) as usize)),
            cursor::RestorePosition,
        )?;
        self.stdout.flush()?;
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
