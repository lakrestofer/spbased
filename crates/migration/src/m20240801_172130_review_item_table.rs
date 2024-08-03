use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // table of item types

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
            .col(text(ReviewItem::Maturity))
            .col(string(ReviewItem::ItemType))
            .col(text(ReviewItem::Data))
            .col(timestamp(ReviewItem::Created).default(Expr::current_timestamp()))
            .col(timestamp(ReviewItem::Updated).default(Expr::current_timestamp()))
            .to_owned();
        manager.create_table(review_item_table).await?;

        let maturity_index = Index::create()
            .name("review_item_maturity_index")
            .table(ReviewItem::Table)
            .col(ReviewItem::Maturity)
            .to_owned();
        manager.create_index(maturity_index).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReviewItem::Table).to_owned())
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
    Maturity,
    ItemType,
    Data,
    Created,
    Updated,
}
