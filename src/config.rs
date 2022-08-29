use ansi_term::Color;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read, write};
use std::path::PathBuf;
use toml;

use crate::command::UserCommand;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhirlwindConfig {
    pub workspace: WhirlwindWorkspace,
    pub environment: HashMap<String, WhirlwindEnv>,
    pub commands: HashMap<String, UserCommand>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhirlwindWorkspace {
    pub directory: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhirlwindEnv {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhirlwindCommand {
    pub command: String,
}

impl WhirlwindConfig {
    /// Create a new config
    pub fn new() -> Self {
        Self {
            workspace: WhirlwindWorkspace {
                directory: PathBuf::new(),
            },
            environment: HashMap::new(),
            commands: HashMap::new(),
        }
    }
}

impl WhirlwindConfig {
    /// Write the config to a whirlwind.config file.
    pub fn write(&self) -> Result<()> {
        let ser = toml::ser::to_string_pretty(self)?;
        let dir = self.workspace.directory.clone().join("whirlwind.toml");
        write(dir, ser)?;
        Ok(())
    }

    /// Read the whirlwind.config.
    pub fn read(path: &PathBuf) -> Result<Self> {
        let config_file = read(path.join("whirlwind.toml"))?;
        let slice = config_file.as_slice();
        let conf: WhirlwindConfig = toml::from_slice(&slice)?;
        Ok(conf)
    }
}

impl WhirlwindConfig {
    pub fn add_command(&mut self) -> Result<()> {
        let command = UserCommand::from_cli()?;
        let name = command.name.clone();

        if let Some(_) = self.commands.insert(name.clone(), command) {
            println!("Updated command {}", name);
        }

        Ok(())
    }

    pub fn add_env(&mut self, name: String, value: String) {
        if let Some(old) = self.environment.insert(
            name.clone(),
            WhirlwindEnv {
                value: value.clone(),
            },
        ) {
            println!(
                "Updated env variable {} from {} to {}",
                name, old.value, value
            );
        }
    }
}

impl WhirlwindConfig {
    pub fn remove_command(&mut self, name: String) {
        if let Some(_) = self.commands.remove(&name) {
            println!("Removed command {}", Color::Red.paint(name));
        } else {
            println!(
                "{} {}",
                Color::Red.paint("Could not find command"),
                Color::Blue.paint(name)
            );
        }
    }

    pub fn remove_env(&mut self, name: String) {
        if let Some(_) = self.environment.remove(&name) {
            println!("Removed environment variable {}", Color::Red.paint(name));
        } else {
            println!(
                "{} {}",
                Color::Red.paint("Could not find environment variable"),
                Color::Blue.paint(name)
            );
        }
    }
}

impl WhirlwindConfig {
    pub fn list_command(&self) {
        println!("{}", Color::Blue.bold().underline().paint("Commands:"));
        for (name, command) in &self.commands {
            println!(
                "{}: {}",
                Color::Green.paint(name),
                Color::Purple.paint(&command.name)
            );
        }
    }

    pub fn list_env(&self) {
        println!(
            "{}",
            Color::Blue
                .bold()
                .underline()
                .paint("Environment Variables:")
        );
        for (name, var) in &self.environment {
            println!(
                "{}: {}",
                Color::Green.paint(name),
                Color::Purple.paint(&var.value)
            );
        }
    }
}
