pub mod application;
pub mod colortheme;
pub mod window;
pub mod command;

use std::io;

use application::Application;
use toml::map::Map;

pub type Config = Map<String, toml::Value>;

pub fn iterate_config<'a, 'b: 'a, F>(config: &'b Config, f: &F) -> Option<(String, toml::Value)>
where
    F: Fn(&'a str, &'b toml::Value) -> bool,
{
    for (name, value) in config {
        if let toml::Value::Table(map) = value {
            if let Some(p) = iterate_config(map, f) {
                return Some(p);
            }
        } else if f(name, value) {
            return Some((name.clone(), value.clone()));
        }
    }
    None
}

pub fn run(config: Config) -> io::Result<()> {
    let mut app = Application::new(config)?;
    app.exec()?;
    Ok(())
}
