use filter_language::AstNode;

use super::*;

///       _______  ___  ___   ___________
///      / __/ _ \/ _ )/ _ | / __/ __/ _ \
///     _\ \/ ___/ _  / __ |_\ \/ _// // /
///    /___/_/  /____/_/ |_/___/___/____/
///    Content agnostic spaced repetition
#[derive(Parser)]
#[command(version, about, long_about, verbatim_doc_comment)]
pub struct Cli {
    // /// Turn debugging information on
    // #[arg(long, action = clap::ArgAction::Count)]
    // pub debug: u8,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
    Init { directory: Option<PathBuf> },
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
    /// Add a new new review item
    Add {
        /// The item model, which describes the format of this item.
        #[clap(long)]
        model: String,
        /// Data in json format.
        #[clap(long, value_parser = parser::json_value)]
        data: serde_json::Value,
        /// A list of tags delimited by ' ' that should be associated with the item.
        #[clap(long,value_delimiter=' ', num_args=1..)]
        tags: Vec<String>,
    },
    /// Edit a review item
    Edit {
        /// The id of the item that is to be edited.
        id: i32,
        #[clap(long, value_parser = parser::json_value)]
        /// The new item model, describing the new format for this item.
        #[clap(long)]
        model: Option<String>,
        /// New data in json format.
        data: Option<serde_json::Value>,
        /// A list of tags delimited by ' ' that should be associated with the item.
        #[clap(long,value_delimiter=' ', num_args=1..)]
        add_tags: Vec<String>,
        /// A list of tags delimited by ' ' that should no longer be associated with the item.
        #[clap(long)]
        remove_tags: Vec<String>,
    },
    /// Delete a review item
    Delete {
        /// The id of the item that is to be edited.
        id: i32,
    },
    /// Retrieve tags associated with review item.
    GetTags {
        /// The id of the item whose tags we want to retrieve.
        id: i32,
        #[arg(long)]
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        post_filter: Option<String>,
        /// Whether to pretty print output
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
    /// Retrieve all review items that match some combination of filters and tag lists.
    Query {
        // filter based on
        #[arg(long, value_parser = parser::ast_node)]
        pre_filter: Option<AstNode>,
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        #[arg(long)]
        post_filter: Option<String>,
        /// Filter items that contain tags
        #[arg(long)]
        include_tags: Vec<String>,
        /// Filter items that do not contain tags
        #[arg(long)]
        exclude_tags: Vec<String>,
        /// Whether to pretty print output
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
}

#[derive(Subcommand)]
pub enum ReviewCommand {
    /// Review the most urgent review item that is due
    #[command(subcommand)]
    Next(NextReviewCommand),
    /// Return how many many items are due
    #[command(subcommand)]
    QueryCount(QueryCountCommand),
    /// score how well the review of an item went
    Score {
        /// id of the item
        id: i32,
        /// "again", "hard", "good", "easy"
        #[arg(value_parser = parser::grade)]
        grade: sra::model::Grade,
    },
}
#[derive(Subcommand)]
pub enum QueryCountCommand {
    New {
        #[arg(long, value_parser = parser::ast_node)]
        // filter based on
        filter: Option<AstNode>,
    },
    Due {
        #[arg(long, value_parser = parser::ast_node)]
        // filter based on
        filter: Option<AstNode>,
    },
}
#[derive(Subcommand)]
pub enum NextReviewCommand {
    New {
        #[arg(long, value_parser = parser::ast_node)]
        // filter based on
        pre_filter: Option<AstNode>,
        #[arg(long)]
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        post_filter: Option<String>,
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
    Due {
        #[arg(long, value_parser = parser::ast_node)]
        // filter based on
        pre_filter: Option<AstNode>,
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
        #[arg(long, value_parser = parser::ast_node)]
        /// querying logic applied before handling the json result
        pre_filter: Option<AstNode>,
        #[arg(long)]
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        post_filter: Option<String>,
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
}

pub mod parser {
    use super::*;
    pub fn grade(s: &str) -> Result<sra::model::Grade, String> {
        use sra::model::Grade;
        let lower = s.to_lowercase();
        match lower.as_str() {
            "again" => Ok(Grade::Again),
            "hard" => Ok(Grade::Hard),
            "good" => Ok(Grade::Good),
            "easy" => Ok(Grade::Easy),
            _ => Err("unknown grade: {s}".into()),
        }
    }

    pub fn ast_node(s: &str) -> Result<AstNode, String> {
        filter_language::FilterLangParser::parse(s).map_err(|e| e.to_string())
    }

    pub fn json_value(s: &str) -> Result<serde_json::Value, String> {
        serde_json::from_str(s)
            .context("could not parse data as json")
            .map_err(|e| e.to_string())
    }
}

pub mod filter_language_2 {
    use super::*;

    pub struct Lexer<'a> {
        input: &'a str,
        pos: usize,
    }

    pub struct Token {
        kind: TokenKind,
        start: usize,
        end: usize,
    }

    impl Token {
        pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
            Self { kind, start, end }
        }
        pub fn invalid(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Invalid,
                start,
                end,
            }
        }
        pub fn eof(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::EOF,
                start,
                end,
            }
        }
        pub fn openparen(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::OpenParen,
                start,
                end,
            }
        }
        pub fn closeparen(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::CloseParen,
                start,
                end,
            }
        }
        pub fn and(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::And,
                start,
                end,
            }
        }
        pub fn or(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Or,
                start,
                end,
            }
        }
        pub fn eq(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Eq,
                start,
                end,
            }
        }
        pub fn neq(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Neq,
                start,
                end,
            }
        }
        pub fn le(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Le,
                start,
                end,
            }
        }
        pub fn leq(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Leq,
                start,
                end,
            }
        }
        pub fn ge(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Ge,
                start,
                end,
            }
        }
        pub fn geq(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Geq,
                start,
                end,
            }
        }
        pub fn string(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::String,
                start,
                end,
            }
        }
        pub fn integer(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Integer,
                start,
                end,
            }
        }
        pub fn float(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Float,
                start,
                end,
            }
        }
        pub fn boolean(start: usize, end: usize) -> Self {
            Self {
                kind: TokenKind::Boolean,
                start,
                end,
            }
        }
    }

    pub enum TokenKind {
        Invalid,
        EOF,
        OpenParen,
        CloseParen,
        And,
        Or,
        Eq,
        Neq,
        Le,
        Leq,
        Ge,
        Geq,
        String,
        Integer,
        Float,
        Boolean,
    }

    impl<'a> Lexer<'a> {
        pub fn init(input: &'a str) -> Self {
            Self { input, pos: 0 }
        }
    }

    impl<'a> Iterator for Lexer<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            _ = self.intertoken_space();

            if self.pos >= self.input.len() {
                return None;
            }

            let start = self.pos;

            self.pos += 1; // consume at least on char such that we do not get stuck in an infinite loop

            Some(Token::invalid(start, self.pos))
        }
    }

    impl<'a> Lexer<'a> {
        pub fn intertoken_space(&mut self) -> bool {
            return true;
        }
    }

    #[cfg(test)]
    mod test {}
}

pub mod filter_language {
    use super::*;
    use pest::iterators::Pairs;
    use pest::pratt_parser::PrattParser;
    use pest::Parser;
    use pest_derive::Parser;
    use std::fmt::Display;
    use std::sync::LazyLock;

    #[derive(Parser)]
    #[grammar = "../grammars/filter_lang.pest"]
    // NOTE: not used directly.
    struct FilterLangPrimitiveParser;

    #[derive(Clone, Copy)]
    pub struct FilterLangParser;

    impl TryFrom<String> for AstNode {
        type Error = anyhow::Error;

        fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
            FilterLangParser::parse(&value)
        }
    }

    impl FilterLangParser {
        pub fn parse(input: &str) -> Result<AstNode> {
            let mut primitive_parser = FilterLangPrimitiveParser::parse(Rule::filter, input)?;
            // the 'filter' rule will consume the whole input
            // therefore we will only need the first result of the primitive parser
            let inner = primitive_parser
                .next()
                .ok_or(anyhow!("could not parse filter node from input"))?
                .into_inner();
            let result = parse_filter_expr(inner);
            Ok(result)
        }
    }

    fn parse_filter_expr(pairs: Pairs<Rule>) -> AstNode {
        static FILTER_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
            use pest::pratt_parser::{Assoc::*, Op};
            use Rule::*;
            let parser = PrattParser::new()
                .op(Op::infix(eq, Left)
                    | Op::infix(neq, Left)
                    | Op::infix(le, Left)
                    | Op::infix(leq, Left)
                    | Op::infix(ge, Left)
                    | Op::infix(geq, Left))
                .op(Op::infix(or, Left))
                .op(Op::infix(and, Left));
            parser
        });
        FILTER_PARSER
            .map_primary(|p| match p.as_rule() {
                Rule::column_identifier => AstNode::Identifier(p.as_str().into()),
                Rule::string => {
                    let s = p.as_str();
                    AstNode::String(s[1..(s.len() - 1)].into())
                }
                Rule::integer => AstNode::Integer(p.as_str().parse().unwrap()),
                Rule::float => AstNode::Float(p.as_str().parse().unwrap()),
                Rule::boolean => AstNode::Bool(p.as_str().parse().unwrap()),
                rule => unreachable!("expected atom but got: {:?}", rule),
            })
            .map_infix(|lhs, op, rhs| {
                use AstNode::*;
                use Operator::*;
                use Rule::*;

                let op_rule = op.as_rule();

                let op = match op_rule {
                    and => And,
                    or => Or,
                    eq => Eq,
                    neq => Neq,
                    le => Le,
                    leq => Leq,
                    ge => Ge,
                    geq => Geq,
                    _ => unreachable!(),
                };

                match (lhs, op, rhs) {
                    (lhs, And | Or, rhs) => AstNode::logical_filter(lhs, op, rhs),
                    (
                        Identifier(c),
                        Eq | Neq | Le | Leq | Ge | Geq,
                        v @ String(_) | v @ Integer(_) | v @ Float(_) | v @ Bool(_),
                    ) => AstNode::comparative_filter(c, op, v),
                    rule => unreachable!("expected operator but got: {:?}", rule),
                }
            })
            .parse(pairs)
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum AstNode {
        LogicalFilter {
            lhs: Box<AstNode>,
            op: Operator,
            rhs: Box<AstNode>,
        },
        ComparativeFilter {
            column: String,
            op: Operator,
            value: Box<AstNode>,
        },
        Identifier(String),
        String(String),
        Integer(i32),
        Float(f32),
        Bool(bool),
    }
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Operator {
        And,
        Or,
        Eq,
        Neq,
        Le,
        Leq,
        Ge,
        Geq,
    }

    impl Display for Operator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Operator::*;
            write!(
                f,
                "{}",
                match self {
                    And => "AND",
                    Or => "OR",
                    Eq => "==",
                    Neq => "!=",
                    Le => "<",
                    Leq => "<=",
                    Ge => ">",
                    Geq => ">=",
                }
            )
        }
    }

    impl AstNode {
        pub fn logical_filter<T: Into<Box<AstNode>>>(lhs: T, op: Operator, rhs: T) -> Self {
            Self::LogicalFilter {
                lhs: lhs.into(),
                op,
                rhs: rhs.into(),
            }
        }
        pub fn comparative_filter<T2: Into<Box<AstNode>>, T: Into<String>>(
            column: T,
            op: Operator,
            value: T2,
        ) -> Self {
            Self::ComparativeFilter {
                column: column.into(),
                op,
                value: value.into(),
            }
        }
        pub fn identifier<I: Into<String>>(i: I) -> Self {
            Self::Identifier(i.into())
        }
        pub fn string<I: Into<String>>(i: I) -> Self {
            Self::String(i.into())
        }
        pub fn integer(i: i32) -> Self {
            Self::Integer(i)
        }
        pub fn float(f: f32) -> Self {
            Self::Float(f)
        }
        pub fn bool(b: bool) -> Self {
            Self::Bool(b)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use Operator::*;

        fn test_ast_node_parser(expr: &str, expected: AstNode) {
            let actual = FilterLangParser::parse(expr).unwrap();
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_parser() {
            test_ast_node_parser(
                "id == 3",
                AstNode::comparative_filter("id", Eq, AstNode::integer(3)),
            );
            test_ast_node_parser(
                "data != 'hello'",
                AstNode::comparative_filter("data", Neq, AstNode::string("hello")),
            );
            test_ast_node_parser(
                "id == 1 && model == 'flashcard'",
                AstNode::logical_filter(
                    AstNode::comparative_filter("id", Eq, AstNode::integer(1)),
                    And,
                    AstNode::comparative_filter("model", Eq, AstNode::string("flashcard")),
                ),
            );
        }
    }
}
