use std::fs;

use muxde::{run, Config};

fn main() -> std::io::Result<()> {
    let config = fs::read_to_string("config/default.toml")?;
    let config: Config = toml::from_str(&config).unwrap();
    run(config)?;
    Ok(())
}
