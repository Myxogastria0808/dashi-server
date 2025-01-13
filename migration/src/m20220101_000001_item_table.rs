use crate::m20220101_000001_label_table::Label;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Item::VisibleId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Item::IsWaste)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Item::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Item::ProductNumber)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Item::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(Item::PurchaseYear)
                            .integer()
                            .null()
                            .default(0),
                    )
                    .col(
                        ColumDef::new(Item::PurchasePrice)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Item::Table, Item::VisibleId)
                            .to(Label::Table, Label::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id, //
    VisibleId, //
    IsWaste, //
    Name, //
    ProductNumber, //
    Description, //
    PurchaseYear,
    PurchasePrice,
    Durability,
    IsDepreciation,
    Connector,
    IsRent,
    Color,
    CreatedAt,
    UpdatedAt,
}
