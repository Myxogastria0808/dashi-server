# Service and Container

## RDB (Container)

- PostgreSQL

## GraphDB (Container)

- Neo4j

## Meilisearch (Container)

- Meilisearch

## Object Strage (Service)

※ initilize のときのみ使用

- Cloudflare R2

# 構造

server では、Layered Architecture を採用している

※ SeaORM を採用している関係で構造が乱れているが、 SeaORM を使用する場合はこの構造のままの方が使いやすいので、この構造のままにしている

```mermaid
flowchart TD
    presentation --> application
    application --> domain
    infrastructure --> domain
```

## presentation

- src/presentation 以下の binary crate

## application

- src/application 以下の library crate

## domain

- src/domain 以下の library crate

## infrastructure

- src/infrastructure 以下の library crate
- migration
- entity

# server の起動

```sh
cargo run --bin presentation
```

# migration の実行

```sh
DATABASE_URL="postgres://<username>:<password>@<ip_address>:<port>/<database_name>" sea-orm-cli migrate refresh
```

# initilizer の実行

```sh
cargo run --bin init
```

# 初期データ

## RDB

```mermaid
erDiagram
    Item |o--|| Label : "VisibleId<->VisibleId"
    Item {
        i32 Id PK "1"
        String VisibleId FK "0000"
        boolean IsWaste "false"
        String Name "筑波大學"
        String ProductNumber ""
        String Description "ルートの物品です"
        Option_i32 PurchaseYear ""
        Option_i32 PurchasePrice ""
        Option_i32 Durability ""
        boolean IsDepreciation "false"
        Json Connector "vec![]"
        boolean IsRent "false"
        String Color ""
        datetime CreatedAt "Utc::now().naive_utc()"
        datetime UpdatedAt "Utc::now().naive_utc()"
    }
    Label {
        i32 Id PK "1"
        String VisibleId PK "0000"
        String Record "Record::Nothing"
    }
```

## Meilisearch

```mermaid
erDiagram
    Item {
        i32 Id PK "1"
        String VisibleId FK "0000"
        String Record "Record::Nothing"
        boolean IsWaste "false"
        String Name "筑波大學"
        String ProductNumber ""
        String Description "ルートの物品です"
        Option_i32 PurchaseYear ""
        Option_i32 PurchasePrice ""
        Option_i32 Durability ""
        boolean IsDepreciation "false"
        Json Connector "vec![]"
        boolean IsRent "false"
        String Color ""
        datetime CreatedAt "Utc::now().naive_utc()"
        datetime UpdatedAt "Utc::now().naive_utc()"
    }
```

## GraphDB

```mermaid
flowchart TD
    id1(("id: 1"))
```
