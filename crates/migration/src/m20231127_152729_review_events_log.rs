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
                    .table(ReviewEventLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReviewEventLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ReviewEventLog::ReviewItemName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ReviewEventLog::ScheduledDate)
                            .date()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ReviewEventLog::ReviewDate).date().not_null())
                    .col(
                        ColumnDef::new(ReviewEventLog::Grade)
                            .enumeration(
                                Grade::Enumeration,
                                [Grade::Again, Grade::Hard, Grade::Pass, Grade::Easy],
                            )
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK__review-event_review-item")
                            .from(ReviewEventLog::Table, ReviewEventLog::ReviewItemName)
                            .to(ReviewItem::Table, ReviewItem::Name),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ReviewEventLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Grade {
    Enumeration,
    Again = 1,
    Hard = 2,
    Pass = 3,
    Easy = 4,
}

#[derive(DeriveIden)]
enum ReviewEventLog {
    Table,
    Id,
    ReviewItemName,
    ScheduledDate,
    ReviewDate,
    Grade,
}
