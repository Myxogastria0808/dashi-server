pub mod depreiation;
pub mod item;

#[derive(Debug)]
pub enum Csv {
    Depreciation(depreiation::Depreiation),
    Item(item::Item),
}
