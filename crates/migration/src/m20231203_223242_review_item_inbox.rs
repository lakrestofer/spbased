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
                    .table(NewReviewItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NewReviewItem::Name)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NewReviewItem::CreateTime).date().not_null())
                    .col(ColumnDef::new(NewReviewItem::UpdateTime).date().not_null())
                    .col(ColumnDef::new(NewReviewItem::URL).string().not_null())
                    .col(ColumnDef::new(NewReviewItem::Data).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(NewReviewItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum NewReviewItem {
    Table,
    Name, // uuid
    CreateTime,
    UpdateTime,
    URL,
    Data,
}
