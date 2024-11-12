use super::*;

/// #     _______  ___  ___   ___________
/// #    / __/ _ \/ _ )/ _ | / __/ __/ _ \
/// #   _\ \/ ___/ _  / __ |_\ \/ _// // /
/// #  /___/_/  /____/_/ |_/___/___/____/
/// Content agnostic spaced repetition                                   
#[derive(Parser)]
#[command(version, about, long_about, verbatim_doc_comment)]
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
    /// CRUD items
    #[command(subcommand)]
    Items(ItemCommand),
    /// CRUD tags
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
        #[arg(long)]
        // filter based on
        pre_filter: Option<String>,
        #[arg(long)]
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        post_filter: Option<String>,
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
}

#[derive(Subcommand)]
pub enum ReviewCommand {
    /// Review the most urgent review item that is due
    Next,
    /// Retrieve information about a review event
    Query {
        #[arg(long)]
        // filter based on
        pre_filter: Option<String>,
        #[arg(long)]
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        post_filter: Option<String>,
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
}
#[derive(Subcommand)]
pub enum TagCommand {
    /// Add a new tag
    Add { name: String },
    /// Edit a tag
    Edit { old_name: String, new_name: String },
    /// List tags. Apply 'and' filtering using the filters
    Query {
        #[arg(long)]
        /// querying logic applied before handling the json result
        pre_filter: Option<String>,
        #[arg(long)]
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        post_filter: Option<String>,
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
}

pub mod filter_language {

    pub enum FilterExprToken {}

    pub enum FilterOp {
        And,
        Or,
        Eq,
        Neq,
        Le,
        Leq,
        Ge,
        Geq,
        Add,
        Sub,
        Mul,
        Div,
    }

    pub enum FilterExpr {
        Binop {
            lhs: Box<FilterExpr>,
            op: FilterOp,
            rhs: Box<FilterExpr>,
        },
        Integer(i32),
        Float(f32),
        String(String),
    }
}
