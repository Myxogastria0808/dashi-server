# デイレクトリ構造

```
/
src/
├─ presentation/
│  ├─ src/
│  │  ├─ route/
│  │  │  ├─ login.rs
│  │  │  ├─ healthcheck.rs
│  │  │  ├─ item.rs
│  │  │  ├─ rent.rs
│  │  │  ├─ generate.rs
│  │  │  ├─ csv.rs
│  │  │  └─ lib.rs
│  │  └─ handler/
│  │     ├─ login.rs
│  │     ├─ healthcheck.rs
│  │     ├─ item.rs
│  │     ├─ rent.rs
│  │     ├─ generate.rs
│  │     ├─ csv.rs
│  │     └─ lib.rs
│  └─ Cargo.toml
├─ application/
│  ├─ src/
│  │     ├─ login/
│  │     │  └─ login.rs
│  │     ├─ healthcheck/
│  │     │  └─ healthcheck.rs
│  │     ├─ item/
│  │     │  ├─ search.rs
│  │     │  ├─ each_item.rs
│  │     │  ├─ connector.rs
│  │     │  ├─ cable.rs
│  │     │  ├─ register.rs
│  │     │  ├─ update.rs
│  │     │  └─ delete.rs
│  │     ├─ rent/
│  │     │  ├─ rent.rs
│  │     │  └─ return.rs
│  │     ├─ generate/
│  │     │  ├─ qr.rs
│  │     │  └─ barcode.rs
│  │     ├─ csv/
│  │     │  ├─ depreiation.rs
│  │     │  └─ item.rs
│  │     └─ lib.rs
│  └─ Cargo.toml
├─ domain/
│  ├─ src/
│  │  ├─ value_object/
│  │  ├─ entity/
│  │  │  ├─ request_type.rs
│  │  │  └─ response_type.rs
│  │  ├─ domain_service/
│  │  └─ repository/
│  └─ Cargo.toml
├─ infrastructure/
│  ├─ src/
│  │  ├─ login.rs
│  │  ├─ healthcheck.rs
│  │  ├─ item.rs
│  │  ├─ rent.rs
│  │  ├─ generate.rs
│  │  ├─ csv.rs
│  │  └─ lib.rs
│  └─ Cargo.toml
└─ main.rs
migration/
├─ src/
│  ├─ lib.rs
│  ├─ m20220101_000001_item_table.rs
│  ├─ m20220101_000001_label_table.rs
│  ├─ m20220101_000001_rent_table.rs
│  └─ main.rs
├─ README.md
└─ Cargo.toml
entity/
├─ src/
│  ├─ item.rs
│  ├─ label.rs
│  ├─ mod.rs
│  ├─ prelude.rs
│  └─ rent.rs
└─ Cargo.toml
```

元データ

```
/
src/
 presentation/
  src/
   route/
    login.rs
    healthcheck.rs
    item.rs
    rent.rs
    generate.rs
    csv.rs
    lib.rs
   handler/
    login.rs
    healthcheck.rs
    item.rs
    rent.rs
    generate.rs
    csv.rs
    lib.rs
  Cargo.toml
 application/
  src/
    login/
     login.rs
    healthcheck/
     healthcheck.rs
    item/
     search.rs
     each_item.rs
     connector.rs
     cable.rs
     register.rs
     update.rs
     delete.rs
    rent/
     rent.rs
     return.rs
    generate/
     qr.rs
     barcode.rs
    csv/
     depreiation.rs
     item.rs
    lib.rs
  Cargo.toml
 domain/
  src/
   value_object/
   entity/
    request_type.rs
    response_type.rs
   domain_service/
   repository/
  Cargo.toml
 infrastructure/
  src/
   login.rs
   healthcheck.rs
   item.rs
   rent.rs
   generate.rs
   csv.rs
   lib.rs
  Cargo.toml
 main.rs
migration/
 src/
  lib.rs
  m20220101_000001_item_table.rs
  m20220101_000001_label_table.rs
  m20220101_000001_rent_table.rs
  main.rs
 README.md
 Cargo.toml
entity/
 src/
  item.rs
  label.rs
  mod.rs
  prelude.rs
  rent.rs
 Cargo.toml
```
