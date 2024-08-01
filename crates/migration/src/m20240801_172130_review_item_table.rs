use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // table of item types
        let review_item_type = Table::create()
            .table(ReviewItemType::Table)
            .if_not_exists()
            .col(pk_auto(ReviewItemType::Id))
            .col(string(ReviewItemType::Type))
            .to_owned();
        manager.create_table(review_item_type).await?;

        let review_item_table = Table::create()
            .table(ReviewItem::Table)
            .if_not_exists()
            .col(pk_auto(ReviewItem::Id))
            .col(float(ReviewItem::Stability))
            .col(float(ReviewItem::Difficulty))
            .col(date(ReviewItem::LastReview))
            .col(date(ReviewItem::Due))
            .col(integer(ReviewItem::Reviews))
            .col(integer(ReviewItem::FailedReviews))
            .col(integer(ReviewItem::Type))
            .col(text(ReviewItem::Data))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_review_item_type")
                    .from(ReviewItem::Table, ReviewItem::Id)
                    .to(ReviewItemType::Table, ReviewItemType::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();
        manager.create_table(review_item_table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReviewItem::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ReviewItemType::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub(crate) enum ReviewItem {
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

#[derive(DeriveIden)]
enum ReviewItemType {
    Table,
    Id,
    Type,
}
