pub use sea_orm_migration::prelude::*;

mod m20231125_154734_review_items;
mod m20231127_141033_scheduled_review_events;
mod m20231127_152729_review_events_log;
mod m20231203_223242_review_item_inbox;
mod test;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231125_154734_review_items::Migration),
            Box::new(m20231127_141033_scheduled_review_events::Migration),
            Box::new(m20231127_152729_review_events_log::Migration),
            Box::new(m20231203_223242_review_item_inbox::Migration),
        ]
    }
}
