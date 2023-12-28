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
                        ColumnDef::new(ReviewItem::Name)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ReviewItem::CreateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ReviewItem::UpdateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ReviewItem::Status).integer().not_null())
                    .col(ColumnDef::new(ReviewItem::Difficulty).double().not_null())
                    .col(ColumnDef::new(ReviewItem::Stability).double().not_null())
                    .col(
                        ColumnDef::new(ReviewItem::NextReviewDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ReviewItem::ItemType).string().not_null())
                    .col(ColumnDef::new(ReviewItem::URL).string().not_null())
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
    Name, // uuid
    CreateTime,
    UpdateTime,
    Status, // inbox, review, burried
    // Priority,
    Difficulty,
    Stability,
    NextReviewDate,
    ItemType, // type of review item, most commonly the name of the "frontend" for this particular item
    URL,
    Data,
}
