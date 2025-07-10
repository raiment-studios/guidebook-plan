use crate::internal::*;
use anyhow::Result;
use std::process::Command;

pub fn command_open() -> Result<()> {
    let app = App::new();
    let filename = app.find_data_filename()?;
    let home = dirs::home_dir().unwrap();
    let pretty_filename = filename
        .to_string_lossy()
        .replace(&home.to_string_lossy().to_string(), "~");

    cprintln!("", "Opening file: [{}](filename)", pretty_filename);

    // TODO: this is hard-coded to use VS Code!!
    Command::new("code").arg(&filename).spawn()?;
    Ok(())
}
