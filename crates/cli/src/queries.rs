use super::*;
use model::*;
pub mod template {
    //! Various string builders for to dynamically generate sql queries

    /// build up template for values string "(?,?,?...),(?,?,?...),(?,?,?...)...""
    pub fn values(n_cols: usize, n_rows: usize) -> String {
        let mut cols = "?,".repeat(n_cols);
        cols.pop();
        let mut t = format!("({cols}),").repeat(n_rows);
        t.pop();
        t
    }
}

pub mod item {

    use filter_language::AstNode;

    use super::*;

    /// Add item to db
    pub fn add(c: &mut Connection, model: &str, data: &str, tags: &[&str]) -> Result<i32> {
        // insert item
        let item_id: i32 = c
            .prepare("insert into item (model,data) values (?,?) returning id")?
            .query_map(params![model, data], |r| Ok(r.get(0)?))?
            .filter_map(Result::ok)
            .next()
            .context("retrieving item from db")?;

        // insert tags if any
        if !tags.is_empty() {
            let tag_ids: Vec<i32> = c
                .prepare(&format!(
                    "insert or replace into tag (name) values {} returning id",
                    template::values(1, tags.len())
                ))?
                .query_map(params_from_iter(tags), |r| Ok(r.get::<usize, i32>(0)?))?
                .filter_map(Result::ok)
                .collect();
            c.execute(
                &format!(
                    "insert or replace into tag_item_map (tag_id, item_id) values {}",
                    template::values(2, tags.len())
                ),
                params_from_iter(tag_ids.iter().flat_map(|tag_id| [*tag_id, item_id])),
            )?;
        }
        Ok(item_id)
    }
    pub fn edit_model(c: &mut Connection, id: i32, model: &str) -> Result<()> {
        c.execute("update item set model = ?1 where id = ?2", (model, id))?;
        Ok(())
    }
    pub fn edit_data(c: &mut Connection, id: i32, data: &str) -> Result<()> {
        c.execute("update item set data = ?1 where id = ?2", (data, id))?;
        Ok(())
    }
    pub fn get(c: &mut Connection, id: i32) -> Result<Item> {
        let mut stmt = c.prepare("select * from item where id = ?1 limit 1")?;
        let mut item = stmt
            .query_map((id,), |r| {
                Ok(Item {
                    id: r.get(0)?,
                    maturity: r.get(1)?,
                    stability: r.get(2)?,
                    difficulty: r.get(3)?,
                    last_review_date: r.get(4)?,
                    model: r.get(5)?,
                    data: r.get(6)?,
                    updated_at: r.get(7)?,
                    created_at: r.get(8)?,
                })
            })?
            .filter_map(Result::ok);
        Ok(item.next().context("retrieving item from db")?)
    }
    pub fn query(c: &mut Connection, filter_expr: Option<AstNode>) -> Result<Vec<Item>> {
        let query = match filter_expr {
            Some(expr) => format!(
                "select * from item where {}",
                utils::filter_expr_to_sql(&expr)
            ),
            None => "select * from item".into(),
        };

        Ok(c.prepare(&query)?
            .query_map([], |r| {
                Ok(Item {
                    id: r.get(0)?,
                    maturity: r.get(1)?,
                    stability: r.get(2)?,
                    difficulty: r.get(3)?,
                    last_review_date: r.get(4)?,
                    model: r.get(5)?,
                    data: r.get(6)?,
                    updated_at: r.get(7)?,
                    created_at: r.get(8)?,
                })
            })?
            .filter_map(Result::ok)
            .collect())
    }
}
// tags
pub mod tag {
    use filter_language::AstNode;

    use super::*;
    pub fn add(c: &mut Connection, tag: &str) -> Result<i32> {
        let mut stmt = c
            .prepare("insert into tag (name) values (?1) returning id")
            .context("preparing sql statement")?;
        let mut id = stmt
            .query_map((tag,), |r| Ok(r.get::<usize, i32>(0)?))
            .context("retrieving tag from sql result")?;

        let id = match id.next() {
            Some(id) => id.context("retrieving i32 from sql result")?,
            _ => return Err(anyhow!("insertion of tag did not return any result")),
        };

        Ok(id)
    }
    pub fn edit(c: &mut Connection, old_name: &str, name: &str) -> Result<()> {
        c.execute("update tag set name = ?1 where name = ?2", (name, old_name))?;
        Ok(())
    }
    pub fn get(c: &mut Connection, id: i32) -> Result<Tag> {
        let mut stmt = c.prepare("select * from tag  where id = ?1")?;
        let mut tag = stmt.query_map((id,), |r| {
            Ok(Tag {
                id: r.get(0)?,
                name: r.get(1)?,
                updated_at: r.get(2)?,
                created_at: r.get(3)?,
            })
        })?;
        let tag = match tag.next() {
            Some(tag) => tag?,
            _ => return Err(anyhow!("sql query did not return any result")),
        };
        Ok(tag)
    }
    pub fn query(c: &mut Connection, filter_expr: Option<AstNode>) -> Result<Vec<Tag>> {
        let query = match filter_expr {
            Some(expr) => format!(
                "select * from tag where {}",
                utils::filter_expr_to_sql(&expr)
            ),
            None => "select * from tag".into(),
        };
        Ok(c.prepare(&query)?
            .query_map([], |r| {
                Ok(Tag {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    updated_at: r.get(2)?,
                    created_at: r.get(3)?,
                })
            })?
            .filter_map(Result::ok)
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();
        db::MIGRATIONS
            .to_latest(&mut conn)
            .context("Trying to migrate sqlite schema")
            .unwrap();
        conn.execute("PRAGMA foreign_keys = ON", ()).unwrap();
        conn
    }

    #[test]
    fn run_migration() {
        init();
    }
    // ==== items ====
    #[test]
    fn test_add_item() {
        let mut c = init();
        assert_eq!(
            item::add(
                &mut c,
                "flashcard",
                r#"{"front":"foo","back":"bar"}"#,
                &["foo", "bar"]
            )
            .unwrap(),
            1
        );
    }
    #[test]
    fn test_edit_and_get_item() {
        let mut c = init();
        let id = item::add(&mut c, "flashcard", r#"{"front":"foo","back":"bar"}"#, &[]).unwrap();
        item::edit_data(&mut c, id, r#"{"front":"foobar","back":"barbaz"}"#).unwrap();
        item::edit_model(&mut c, id, "reading").unwrap();
        let item = item::get(&mut c, id).unwrap();
        assert_eq!(item.model, "reading");
        assert_eq!(item.data, r#"{"front":"foobar","back":"barbaz"}"#);
    }
    // -------------
    // ==== items ====
    #[test]
    fn test_add_tag() {
        let mut c = init();
        let id = tag::add(&mut c, "edan35").unwrap();
        assert!(id == 1);
    }
    #[test]
    fn test_edit_tag() {
        let mut c = init();
        let id = tag::add(&mut c, "edan35").unwrap();
        assert!(id == 1);
        tag::edit(&mut c, "edan35", "edaf35").unwrap();
        let tag = tag::get(&mut c, id).unwrap();
        assert_eq!(&tag.name, "edaf35");
    }
    // -------------
}

pub mod utils {
    use super::filter_language::{AstNode, Operator};

    pub fn filter_expr_to_sql(expr: &AstNode) -> String {
        use AstNode::*;
        match expr {
            LogicalFilter { lhs, op, rhs } => format!(
                "({}) {} ({})",
                filter_expr_to_sql(lhs),
                op,
                filter_expr_to_sql(rhs)
            ),
            ComparativeFilter { column, op, value } => match column.as_str() {
                // when dealing with fields that describe time, we require that all values
                // constitute valid time formats
                // <https://www.sqlite.org/lang_datefunc.html>
                "updated_at" | "created_at" => format!(
                    "datetime({}) {} datetime({})",
                    column,
                    op,
                    filter_expr_to_sql(value)
                ),
                _ => format!("{} {} {}", column, op, filter_expr_to_sql(value)),
            },
            Identifier(i) => i.clone(),
            String(s) => format!("'{s}'"),
            Integer(i) => i.to_string(),
            Float(f) => f.to_string(),
            Bool(b) => b.to_string().to_uppercase(),
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn filter_expr_to_sql_test() {
            use Operator::*;
            let ast = AstNode::logical_filter(
                AstNode::comparative_filter("id", Le, AstNode::integer(3)),
                And,
                AstNode::comparative_filter("created_at", Ge, AstNode::string("2024-11-13")),
            );
            assert_eq!(
                filter_expr_to_sql(&ast),
                "(id < 3) AND (created_at > '2024-11-13')".to_string()
            );
        }
    }
}
