use crate::entity::data_type::rent_item;

#[derive(Debug)]
pub enum Rent {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {
    Data(rent_item::RentItemData),
}

#[derive(Debug)]
pub enum Response {}
