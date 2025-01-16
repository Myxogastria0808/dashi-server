use crate::entity::data_type::update_item;

#[derive(Debug)]
pub enum Update {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {
    Data(update_item::UpdateItemData),
}

#[derive(Debug)]
pub enum Response {}
