use crate::config::WhirlwindConfig;
use ansi_term::Color;
use anyhow::Result;
use std::env::current_dir;
use std::fs::read_dir;

/// Initialize the workspace
pub fn initialize_workspace() -> Result<()> {
    let curdir = current_dir()?;
    let dir_entries = read_dir(&curdir)?;

    println!("{}", Color::Blue.paint("Initializing workspace"));

    for entry in dir_entries {
        if let Ok(entry) = entry {
            if entry.path().ends_with("whirlwind.toml") {
                println!("{}", Color::Red.paint("Config already exists. Skipping"));
                return Ok(());
            }
        }
    }

    let mut config = WhirlwindConfig::new();
    config.workspace.directory = curdir;
    config.write()?;
    println!("{}", Color::Green.paint("Workspace config generated."));

    Ok(())
}
