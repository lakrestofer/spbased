use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReviewItem::Table)
                    .if_not_exists()
                    .col(pk_auto(ReviewItem::Id))
                    .col(float(ReviewItem::Stability))
                    .col(float(ReviewItem::Difficulty))
                    .col(date(ReviewItem::LastReview))
                    .col(date(ReviewItem::Due))
                    .col(integer(ReviewItem::Reviews))
                    .col(integer(ReviewItem::FailedReviews))
                    .col(text(ReviewItem::Type))
                    .col(text(ReviewItem::Data))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ReviewItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ReviewItem {
    Table,
    Id,
    Stability,
    Difficulty,
    LastReview,
    Due,
    Reviews,
    FailedReviews,
    Type,
    Data,
}
