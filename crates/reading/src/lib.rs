use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use model::SourceFragment;
use normalize_path::NormalizePath;
use resolve_path::PathResolveExt;

pub fn handle_command(command: cli::Command) -> Result<()> {
    match command {
        cli::Command::Create {
            path,
            page,
            task_description,
        } => {
            path_validator(&path)?;
            let task = task_description.unwrap_or_else(|| {
                Input::new()
                    .with_prompt("Task description")
                    .validate_with(|s: &String| {
                        if s.is_empty() {
                            Err(anyhow!("task description cannot be empty"))
                        } else {
                            Ok(())
                        }
                    })
                    .interact_text()
                    .unwrap()
            });

            let fragment = SourceFragment {
                path,
                task_description: task,
                page,
            };
            println!("{}", serde_json::to_string(&fragment)?);
        }
        cli::Command::Edit { fragment } => {
            let path: String = Input::new()
                .with_prompt("Path to document")
                .validate_with(path_validator)
                .with_initial_text(fragment.path)
                .interact_text()
                .unwrap();
            let task_description: String = Input::new()
                .with_prompt("Task description")
                .validate_with(|s: &String| {
                    if s.is_empty() {
                        Err(anyhow!("task description cannot be empty"))
                    } else {
                        Ok(())
                    }
                })
                .with_initial_text(fragment.task_description)
                .interact_text()
                .unwrap();
            let page: i32 = Input::new()
                .with_prompt("Page")
                .validate_with(|s: &String| {
                    if s.parse::<i32>().is_err() {
                        Err(anyhow!("page must be a number"))
                    } else {
                        Ok(())
                    }
                })
                .with_initial_text(fragment.page.to_string())
                .interact_text()
                .unwrap()
                .parse()
                .unwrap();
            let fragment = SourceFragment {
                path,
                task_description,
                page,
            };
            println!("{}", serde_json::to_string(&fragment)?);
        }
    }
    Ok(())
}

fn resolve_and_normalize(p: PathBuf) -> Result<PathBuf> {
    let p = p.try_resolve()?.into_owned();
    let p = p.normalize();
    if !p.is_file() {
        return Err(anyhow!("path must be a file"));
    }
    Ok(p)
}

fn path_validator(s: &String) -> Result<()> {
    let p = PathBuf::from(s);
    resolve_and_normalize(p)?;
    Ok(())
}

pub mod cli {
    use super::*;

    use crate::model::SourceFragment;

    #[derive(Debug, Parser)]
    /// cli to create and manage flashcards. Provides a way to create, edit and read flashcards
    /// in a structured way.
    pub struct Cli {
        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        pub debug: u8,
        #[command(subcommand)]
        pub command: Command,
    }

    #[derive(Subcommand, Debug)]
    pub enum Command {
        Create {
            path: String,
            page: i32,
            task_description: Option<String>,
        },
        Edit {
            #[arg(value_parser = parser::fragment)]
            fragment: SourceFragment,
        },
    }

    pub mod parser {
        use super::*;
        pub fn fragment(s: &str) -> Result<crate::model::SourceFragment> {
            Ok(serde_json::from_str(s)?)
        }
    }
}

pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SourceFragment {
        /// the path to the document
        pub path: String,
        /// a description of what content should be consumed (a short quote of the initial paragraph would be fine as well)
        pub task_description: String,
        /// the starting page of the fragment
        pub page: i32,
    }
}
