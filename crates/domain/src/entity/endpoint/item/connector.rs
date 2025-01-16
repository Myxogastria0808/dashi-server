use crate::entity::data_type::search_item;

#[derive(Debug)]
pub enum Connector {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {
    Data(Vec<search_item::SearchItemData>),
}
