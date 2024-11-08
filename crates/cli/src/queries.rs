use super::*;
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
    pub fn query(c: &mut Connection) -> Result<Item> {
        Err(anyhow!("not yet implemented"))
    }
}
// tags
pub mod tag {
    use super::*;
    pub fn add(c: &mut Connection, tag: &str) -> Result<i32> {
        let mut stmt = c.prepare("insert into tag (name) values (?1) returning id")?;
        let mut id = stmt
            .query_map((tag,), |r| Ok(r.get(0)?))?
            .filter_map(Result::ok);
        Ok(id.next().context("retrieving item from db")?)
    }
    pub fn edit(c: &mut Connection, id: i32, name: &str) -> Result<()> {
        c.execute("update tag set name = ?1 where id = ?2", (name, id))?;
        Ok(())
    }
    pub fn get(c: &mut Connection, id: i32) -> Result<Tag> {
        let mut stmt = c.prepare("select * from tag  where id = ?1")?;
        let mut tag = stmt
            .query_map((id,), |r| {
                Ok(Tag {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    updated_at: r.get(2)?,
                    created_at: r.get(3)?,
                })
            })?
            .filter_map(Result::ok);
        Ok(tag.next().context("retrieving tag from db")?)
    }
    pub fn query(c: &mut Connection) -> Result<Item> {
        Err(anyhow!("not yet implemented"))
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
        tag::edit(&mut c, id, "edaf35").unwrap();
        let tag = tag::get(&mut c, id).unwrap();
        assert_eq!(&tag.name, "edaf35");
    }
    // -------------
}
