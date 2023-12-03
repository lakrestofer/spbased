#[cfg(test)]
mod test {
    use crate::Migrator;
    use sea_orm_migration::prelude::*;
    use sea_orm_migration::sea_orm::Database;

    #[async_std::test]
    async fn fresh_migration_test() {
        let temp_dir = std::env::temp_dir();

        let db_path = temp_dir.join("test_db.sqlite");
        let db_path_str = db_path
            .to_str()
            .expect("Could not convert path to string. Is Path valid utf8?");
        let protocol_str = format!("sqlite://{db_path_str}?mode=rwc");
        let db = Database::connect(protocol_str)
            .await
            .expect("could not return");

        Migrator::up(&db, None)
            .await
            .expect("Could not perform a fresh migration");

        std::fs::remove_file(db_path).expect("could not remove file");
    }
}
