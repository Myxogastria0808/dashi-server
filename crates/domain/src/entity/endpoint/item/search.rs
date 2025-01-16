use crate::entity::data_type::search_item;

#[derive(Debug)]
pub enum Search {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {
    Data(search_item::SearchItemData),
}
