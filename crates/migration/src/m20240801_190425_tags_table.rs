use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240801_172130_review_item_table::ReviewItem;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // the table of tags
        let tag_table = Table::create()
            .table(Tag::Table)
            .if_not_exists()
            .col(pk_auto(Tag::Id))
            .col(string(Tag::Tag))
            .to_owned();
        manager.create_table(tag_table).await?;

        let tag_map_table = Table::create()
            .table(TagMap::Table)
            .if_not_exists()
            .col(integer(TagMap::TagId))
            .col(integer(TagMap::ItemId))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_id_to_tag")
                    .from(TagMap::Table, TagMap::TagId)
                    .to(Tag::Table, Tag::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("FK_id_to_tag")
                    .from(TagMap::Table, TagMap::ItemId)
                    .to(ReviewItem::Table, ReviewItem::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();
        manager.create_table(tag_map_table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TagMap::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Tag,
}

#[derive(DeriveIden)]
enum TagMap {
    Table,
    TagId,
    ItemId,
}
