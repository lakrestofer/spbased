use filter_language::AstNode;

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
        pre_filter: Option<AstNode>,
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
        #[arg(long)]
        /// querying logic applied before handling the json result
        pre_filter: Option<AstNode>,
        #[arg(long)]
        /// Fine grained json based filtering. Uses <https://jmespath.org/>
        post_filter: Option<String>,
        #[arg(long, default_value_t = false)]
        pretty: bool,
    },
}

impl clap::builder::ValueParserFactory for filter_language::AstNode {
    type Parser = filter_language::FilterLangParser;
    fn value_parser() -> Self::Parser {
        filter_language::FilterLangParser
    }
}

impl clap::builder::TypedValueParser for filter_language::FilterLangParser {
    type Value = filter_language::AstNode;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let str = value
            .try_into()
            .map_err(|_| clap::error::Error::new(clap::error::ErrorKind::InvalidValue))?;
        filter_language::FilterLangParser::parse(str)
            .map_err(|_| clap::error::Error::new(clap::error::ErrorKind::InvalidValue))
    }
}

pub mod filter_language {
    use super::*;
    use pest::iterators::Pairs;
    use pest::pratt_parser::PrattParser;
    use pest::Parser;
    use pest_derive::Parser;
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
                .op(Op::infix(or, Left))
                .op(Op::infix(and, Left))
                .op(Op::infix(eq, Left)
                    | Op::infix(neq, Left)
                    | Op::infix(le, Left)
                    | Op::infix(leq, Left)
                    | Op::infix(ge, Left)
                    | Op::infix(geq, Left));
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
        #[test]
        fn test_parser() {
            assert_eq!(
                FilterLangParser::parse("id == 3").unwrap(),
                AstNode::comparative_filter("id", Eq, AstNode::integer(3))
            );
            assert_eq!(
                FilterLangParser::parse("data != 'hello'").unwrap(),
                AstNode::comparative_filter("data", Neq, AstNode::string("hello"))
            );
        }
    }
}
