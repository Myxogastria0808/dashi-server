use crate::entity::data_type::item_csv;

#[derive(Debug)]
pub enum Item {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {
    Data(Vec<item_csv::ItemCsvData>),
}
