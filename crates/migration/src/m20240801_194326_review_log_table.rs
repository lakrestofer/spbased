use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ReviewLog::Table)
                    .if_not_exists()
                    .col(pk_auto(ReviewLog::Id))
                    .col(integer(ReviewLog::ItemId))
                    .col(integer(ReviewLog::Grade))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ReviewLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ReviewLog {
    Table,
    Id,
    ItemId,
    Grade,
}
