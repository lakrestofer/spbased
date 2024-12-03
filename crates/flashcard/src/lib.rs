use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;

pub fn handle_command(command: cli::Command) -> Result<Option<String>> {
    Ok(match command {
        cli::Command::Create { pretty } => {
            let question: String = Input::new()
                .with_prompt("Question")
                .interact_text()
                .map_err(|e| anyhow!("could not prompt for question: {:?}", e))?;
            let answer: String = Input::new()
                .with_prompt("Answer")
                .interact_text()
                .map_err(|e| anyhow!("could not prompt for question: {:?}", e))?;
            let flashcard = model::FlashCard { question, answer };
            let output = if pretty {
                serde_json::to_string_pretty(&flashcard)?
            } else {
                serde_json::to_string(&flashcard)?
            };
            Some(format!("{}", output))
        }
        cli::Command::Edit { pretty, flashcard } => {
            let question: String = Input::new()
                .with_prompt("Question")
                .default(flashcard.question)
                .interact_text()
                .map_err(|e| anyhow!("could not prompt for question: {:?}", e))?;
            let answer: String = Input::new()
                .with_prompt("Answer")
                .default(flashcard.answer)
                .interact_text()
                .map_err(|e| anyhow!("could not prompt for question: {:?}", e))?;
            let flashcard = model::FlashCard { question, answer };
            let output = if pretty {
                serde_json::to_string_pretty(&flashcard)?
            } else {
                serde_json::to_string(&flashcard)?
            };
            Some(format!("{}", output))
        }
        cli::Command::ReadQuestion { flashcard } => Some(format!("{}", flashcard.question)),
        cli::Command::ReadAnswer { flashcard } => Some(format!("{}", flashcard.answer)),
    })
}

pub mod cli {
    use std::path::PathBuf;

    use super::*;

    use crate::model::FlashCard;

    #[derive(Debug, Parser)]
    /// cli to create and manage flashcards. Provides a way to create, edit and read flashcards
    ///
    pub struct Cli {
        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        pub debug: u8,
        #[command(subcommand)]
        pub command: Command,
        #[arg(short, long, global = true)]
        /// optional output file
        pub output: Option<PathBuf>,
    }

    #[derive(Subcommand, Debug)]
    pub enum Command {
        Create {
            #[arg(short, long)]
            pretty: bool,
        },
        Edit {
            #[arg(short, long)]
            pretty: bool,
            #[arg(value_parser = parser::flashcard)]
            flashcard: FlashCard,
        },
        ReadQuestion {
            #[arg(value_parser = parser::flashcard)]
            flashcard: FlashCard,
        },
        ReadAnswer {
            #[arg(value_parser = parser::flashcard)]
            flashcard: FlashCard,
        },
    }

    pub mod parser {
        use super::*;
        pub fn flashcard(s: &str) -> Result<crate::model::FlashCard> {
            // if s is wrapped in quotes, remove them
            let s = s.trim_matches('\"');
            Ok(serde_json::from_str(s)?)
        }
    }
}

pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FlashCard {
        pub question: String,
        pub answer: String,
    }
}
