use super::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Struct defining the arguments and commands that the cli takes.
/// The outward facing api of the cli
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
    Init { directory: PathBuf },
    /// Query, or CRUD items
    #[command(subcommand)]
    Item(ItemCommand),
    /// Query, or CRUD tags
    #[command(subcommand)]
    Tags(TagCommand),
    /// Review the items
    #[command(subcommand)]
    Review(ReviewCommand),
}

#[derive(Subcommand)]
pub enum ItemCommand {
    Add {
        model: String,
        data: String,
        tags: Vec<String>,
    },
    Edit {
        id: i32,
        model: String,
        data: String,
    },
    // TODO add filters, for now simply list all options
    Query {
        filter: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ReviewCommand {
    /// Review the most urgent review item that is due
    Next,
    /// Retrieve information about a review event
    Query { filter: Option<String> },
}
#[derive(Subcommand)]
pub enum TagCommand {
    /// Add a new tag
    Add { name: String },
    /// Edit a tag
    Edit { old_name: String, new_name: String },
    /// List tags
    /// TODO: add query options
    Query { filter: Option<String> },
}
