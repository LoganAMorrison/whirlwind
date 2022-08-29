use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use dialoguer;
use std::env::current_dir;
use std::process;

mod command;
mod config;
mod init;
mod theme;
mod unicode;

use config::WhirlwindConfig;

#[derive(Debug, Parser)]
#[clap(name = "whirlwind")]
#[clap(author = "Logan A. Morrison")]
#[clap(version = "0.1.0")]
#[clap(about = "A workspace manager")]
struct Cli {
    #[clap(subcommand)]
    commands: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Initialize the workspace
    Init {},

    /// Run a defined command
    Run,

    /// Add an item to the workspace config
    Add(Add),

    /// Add an item to the workspace config
    Remove(Remove),

    /// Add an item to the workspace config
    List(List),
}

#[derive(Debug, Args)]
struct Add {
    #[clap(subcommand)]
    command: AddCommands,
}

#[derive(Debug, Args)]
struct Remove {
    #[clap(subcommand)]
    command: RemoveCommands,
}

#[derive(Debug, Args)]
struct List {
    #[clap(subcommand)]
    command: ListCommands,
}

#[derive(Debug, Subcommand)]
enum AddCommands {
    /// Add a command
    Command,
    /// Add an environment variable
    #[clap(arg_required_else_help = true)]
    Env {
        /// Name of the environment variable
        #[clap(value_parser)]
        name: String,
        /// Value of the environment variable
        #[clap(value_parser)]
        value: String,
    },
}

#[derive(Debug, Subcommand)]
enum RemoveCommands {
    /// Remove a command
    #[clap(arg_required_else_help = true)]
    Command {
        /// Name of the command
        #[clap(value_parser)]
        name: String,
    },
    /// Remove an environment variable
    #[clap(arg_required_else_help = true)]
    Env {
        /// Name of the environment variable
        #[clap(value_parser)]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
enum ListCommands {
    /// List all commands and environment variables
    All,
    /// List all commands
    Command,
    /// List all environment variables
    Env,
}

fn get_config_from_current_dir() -> Result<WhirlwindConfig> {
    let curdir = current_dir()?;
    let conf = WhirlwindConfig::read(&curdir)?;
    Ok(conf)
}

fn execute() -> Result<()> {
    let args = Cli::parse();
    let mut conf = get_config_from_current_dir().expect("Cannot open configuration file.");

    match args.commands {
        SubCommands::Init {} => {
            init::initialize_workspace()?;
        }
        SubCommands::Run => {
            let cmd_names: Vec<String> = conf.commands.keys().map(|k| k.to_string()).collect();

            let idx = dialoguer::Select::new()
                .items(cmd_names.as_slice())
                .with_prompt("Select command to run")
                .interact()?;

            let cmd = conf
                .commands
                .get(&cmd_names[idx])
                .ok_or(anyhow!("Something went wrong during command retrieval."))?;

            let (prog, args) = cmd.run_interactive()?;

            print!("{}", prog);
            for arg in args {
                print!(" {}", arg);
            }
            println!("");
        }
        SubCommands::Add(add) => match add.command {
            AddCommands::Command => {
                conf.add_command()?;
                conf.write()?;
            }
            AddCommands::Env { name, value } => {
                conf.add_env(name, value);
                conf.write()?;
            }
        },
        SubCommands::Remove(remove) => match remove.command {
            RemoveCommands::Command { name } => {
                conf.remove_command(name);
                conf.write()?;
            }
            RemoveCommands::Env { name } => {
                conf.remove_env(name);
                conf.write()?;
            }
        },
        SubCommands::List(list) => match list.command {
            ListCommands::All => {
                conf.list_command();
                println!("");
                conf.list_env();
            }
            ListCommands::Command => {
                conf.list_command();
            }
            ListCommands::Env => {
                conf.list_env();
            }
        },
    }

    Ok(())
}

fn main() {
    match execute() {
        Ok(_) => println!("All good"),
        Err(e) => eprintln!("{}", e),
    }
}
