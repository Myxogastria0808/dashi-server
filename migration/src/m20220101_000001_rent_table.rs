use crate::m20220101_000001_item_table::Item;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rent::ItemId).integer().not_null())
                    .col(ColumnDef::new(Rent::Recipient).string().not_null())
                    .col(
                        ColumnDef::new(Rent::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Rent::RentAt).timestamp().not_null())
                    .col(ColumnDef::new(Rent::ReturnAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Rent::Table, Rent::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Rent {
    Table,
    Id,
    ItemId,
    Recipient,
    Description,
    RentAt,
    ReturnAt,
}
