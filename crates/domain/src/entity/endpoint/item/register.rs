use crate::entity::data_type::register_item;

#[derive(Debug)]
pub enum Register {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {
    Data(register_item::RegisterItemData),
}

#[derive(Debug)]
pub enum Response {}
