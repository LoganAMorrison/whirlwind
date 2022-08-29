use anyhow::Result;
use clap;
use dialoguer;
use serde::{Deserialize, Serialize};
use std::process;

/// pd file.mro output --vdrmode=disable
/// name: pd
/// args: file, output
///
/// type of file: File
///
/// type of output: path
///
/// options: vdrmode

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCommand {
    pub name: String,
    pub exec: String,
    pub args: Vec<UserCommandArg>,
    pub options: Vec<UserCommandOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCommandArg {
    pub name: String,
    pub ty: UserCommandArgType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCommandOption {
    pub name: String,
    pub short: Option<String>,
    pub long: Option<String>,
    pub ty: UserCommandArgType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserCommandArgType {
    Other,
    StringArg,
    IntArg,
    FloatArg,
    FileArg,
    PathArg,
}

impl UserCommand {
    pub fn from_cli() -> Result<UserCommand> {
        let name = UserCommand::ask_for_string("name")?;
        let exec = UserCommand::ask_for_string("exec")?;
        let args = UserCommand::ask_for_args()?;
        let options = UserCommand::ask_for_options()?;

        Ok(UserCommand {
            name,
            exec,
            args,
            options,
        })
    }

    pub fn run_clap(&self) -> clap::ArgMatches {
        let mut cmd = clap::Command::new(self.name.clone());

        for arg in &self.args {
            cmd = cmd.arg(
                clap::Arg::new(&*arg.name)
                    .value_parser(clap::value_parser!(String))
                    .required(true),
            );
        }

        for opt in &self.options {
            cmd = cmd.arg(
                clap::Arg::new(&*opt.name)
                    .value_parser(clap::value_parser!(String))
                    .required(false),
            );
        }

        cmd.get_matches()
    }

    pub fn run_interactive(&self) -> Result<(String, Vec<String>)> {
        let mut args: Vec<String> = Vec::new();
        for arg in &self.args {
            let val: String = dialoguer::Input::new()
                .with_prompt(&arg.name)
                .interact_text()?;
            args.push(val);
        }

        for opt in &self.options {
            let val: String = dialoguer::Input::new()
                .with_prompt(&opt.name)
                .interact_text()?;
            if !val.is_empty() {
                args.push(format!("--{}={}", opt.name, val));
            }
        }

        Ok((self.name.clone(), args))
    }
}

impl UserCommand {
    fn ask_for_string(text: &str) -> Result<String> {
        let name: String = dialoguer::Input::new().with_prompt(text).interact_text()?;
        Ok(name)
    }
    fn ask_for_bool(text: &str) -> Result<bool> {
        let name = dialoguer::Confirm::new().with_prompt(text).interact()?;
        Ok(name)
    }

    fn ask_for_args() -> Result<Vec<UserCommandArg>> {
        let arg_type_options = vec!["string", "int", "float", "file", "path", "other"];

        let mut add_arg: bool = UserCommand::ask_for_bool("Add arguments?")?;
        let mut args: Vec<UserCommandArg> = Vec::new();

        while add_arg {
            let arg_name = UserCommand::ask_for_string("Argument name")?;

            let arg_type = dialoguer::Select::new()
                .items(&arg_type_options)
                .with_prompt("Argument type")
                .interact()?;

            match arg_type {
                0 => {
                    args.push(UserCommandArg {
                        name: arg_name,
                        ty: UserCommandArgType::StringArg,
                    });
                }
                1 => {
                    args.push(UserCommandArg {
                        name: arg_name,
                        ty: UserCommandArgType::IntArg,
                    });
                }
                2 => {
                    args.push(UserCommandArg {
                        name: arg_name,
                        ty: UserCommandArgType::FloatArg,
                    });
                }
                3 => {
                    args.push(UserCommandArg {
                        name: arg_name,
                        ty: UserCommandArgType::FileArg,
                    });
                }
                4 => {
                    args.push(UserCommandArg {
                        name: arg_name,
                        ty: UserCommandArgType::PathArg {},
                    });
                }
                _ => {
                    args.push(UserCommandArg {
                        name: arg_name,
                        ty: UserCommandArgType::Other {},
                    });
                }
            }

            add_arg = UserCommand::ask_for_bool("Add another agument?")?;
        }

        Ok(args)
    }

    fn ask_for_options() -> Result<Vec<UserCommandOption>> {
        let arg_type_options = vec!["string", "int", "float", "file", "path", "other"];

        let mut add_arg: bool = UserCommand::ask_for_bool("Add options?")?;
        let mut args: Vec<UserCommandOption> = Vec::new();

        while add_arg {
            let arg_name = UserCommand::ask_for_string("Argument name")?;

            let arg_type = dialoguer::Select::new()
                .items(&arg_type_options)
                .with_prompt("Argument type")
                .interact()?;

            match arg_type {
                0 => {
                    args.push(UserCommandOption {
                        name: arg_name,
                        short: None,
                        long: None,
                        ty: UserCommandArgType::StringArg,
                    });
                }
                1 => {
                    args.push(UserCommandOption {
                        name: arg_name,
                        short: None,
                        long: None,
                        ty: UserCommandArgType::IntArg,
                    });
                }
                2 => {
                    args.push(UserCommandOption {
                        name: arg_name,
                        short: None,
                        long: None,
                        ty: UserCommandArgType::FloatArg,
                    });
                }
                3 => {
                    args.push(UserCommandOption {
                        name: arg_name,
                        short: None,
                        long: None,
                        ty: UserCommandArgType::FileArg,
                    });
                }
                4 => {
                    args.push(UserCommandOption {
                        name: arg_name,
                        short: None,
                        long: None,
                        ty: UserCommandArgType::PathArg {},
                    });
                }
                _ => {
                    args.push(UserCommandOption {
                        name: arg_name,
                        short: None,
                        long: None,
                        ty: UserCommandArgType::Other {},
                    });
                }
            }

            add_arg = UserCommand::ask_for_bool("Add another agument?")?;
        }

        Ok(args)
    }
}
