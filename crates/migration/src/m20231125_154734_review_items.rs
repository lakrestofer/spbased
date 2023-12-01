use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ReviewItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReviewItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReviewItem::Difficulty).float().not_null())
                    .col(ColumnDef::new(ReviewItem::Stability).float().not_null())
                    .col(ColumnDef::new(ReviewItem::LastReviewDate).date().not_null())
                    .col(ColumnDef::new(ReviewItem::URI).string().not_null())
                    .col(ColumnDef::new(ReviewItem::Data).string().not_null())
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
pub enum ReviewItem {
    Table,
    Id,
    Difficulty,
    Stability,
    LastReviewDate,
    // NextReviewDate,
    URI,
    Data,
}
