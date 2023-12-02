use sea_orm_migration::prelude::*;

use crate::m20231125_154734_review_items::ReviewItem;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ScheduledReviewEvent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScheduledReviewEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ScheduledReviewEvent::ReviewItemName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ScheduledReviewEvent::ScheduledDate)
                            .date()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK__review-event_review-item")
                            .from(
                                ScheduledReviewEvent::Table,
                                ScheduledReviewEvent::ReviewItemName,
                            )
                            .to(ReviewItem::Table, ReviewItem::Name),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ScheduledReviewEvent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ScheduledReviewEvent {
    Table,
    Id,
    ReviewItemName,
    ScheduledDate,
}
