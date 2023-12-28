pub use sea_orm_migration::prelude::*;

mod m20231125_154734_review_items;
mod m20231127_152729_review_events_log;
mod test;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231125_154734_review_items::Migration),
            Box::new(m20231127_152729_review_events_log::Migration),
        ]
    }
}
