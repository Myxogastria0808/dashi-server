use crate::entity::data_type::individual_item;

#[derive(Debug)]
pub enum Individual {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {
    Data(individual_item::IndividualItemData),
}
