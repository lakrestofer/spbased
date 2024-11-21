use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::Editor;

pub fn handle_command(command: cli::Command) -> Result<()> {
    match command {
        cli::Command::Create { pretty } => {
            let question = Editor::new()
                .extension(".md")
                .edit("# Question\n\n")
                .unwrap();
            let answer = Editor::new().extension(".md").edit("# Answer\n\n").unwrap();
            if let (Some(question), Some(answer)) = (question, answer) {
                let flashcard = model::FlashCard { question, answer };
                let output = if pretty {
                    serde_json::to_string_pretty(&flashcard)?
                } else {
                    serde_json::to_string(&flashcard)?
                };
                println!("{}", output);
            }
        }
        cli::Command::Edit { pretty, flashcard } => {
            let question = Editor::new()
                .extension(".md")
                .edit(&flashcard.question)
                .unwrap();
            let answer = Editor::new()
                .extension(".md")
                .edit(&flashcard.answer)
                .unwrap();
            if let (Some(question), Some(answer)) = (question, answer) {
                let flashcard = model::FlashCard { question, answer };
                let output = if pretty {
                    serde_json::to_string_pretty(&flashcard)?
                } else {
                    serde_json::to_string(&flashcard)?
                };
                println!("{}", output);
            }
        }
        cli::Command::ReadQuestion { flashcard } => println!("{}", flashcard.question),
        cli::Command::ReadAnswer { flashcard } => println!("{}", flashcard.answer),
    }
    Ok(())
}

pub mod cli {
    use super::*;

    use crate::model::FlashCard;

    #[derive(Debug, Parser)]
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
