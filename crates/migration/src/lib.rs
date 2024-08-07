pub use sea_orm_migration::prelude::*;

mod m20240801_172130_review_item_table;
mod m20240801_190425_tags_table;
mod m20240801_194326_review_log_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240801_172130_review_item_table::Migration),
            Box::new(m20240801_190425_tags_table::Migration),
            Box::new(m20240801_194326_review_log_table::Migration),
        ]
    }
}
