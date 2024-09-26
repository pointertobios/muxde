pub mod api;
pub mod application;
pub mod colortheme;
pub mod command;
pub mod innerpipe;
pub mod window;
pub mod windows;

use std::io;

use application::Application;
use toml::map::Map;

pub type Config = Map<String, toml::Value>;

pub fn iterate_config<'a, 'b: 'a>(
    config: &'b Config,
    cmd: &'a str,
) -> Option<(String, toml::Value, bool)> {
    for (name, value) in config {
        if let toml::Value::Table(map) = value {
            if let Some(p) = iterate_config(map, cmd) {
                return Some(p);
            }
        } else if cmd == value.as_str().unwrap() {
            return Some((name.clone(), value.clone(), true));
        } else if value.as_str().unwrap().starts_with(cmd) {
            return Some((name.clone(), value.clone(), false));
        }
    }
    None
}

pub fn run(config: Config) -> io::Result<()> {
    let mut app = Application::new(config)?;
    app.exec()?;
    Ok(())
}
