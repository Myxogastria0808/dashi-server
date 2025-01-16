use crate::entity::data_type::depreiation_csv;

#[derive(Debug)]
pub enum Depreiation {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {
    Data(Vec<depreiation_csv::DepreiationCsvData>),
}
