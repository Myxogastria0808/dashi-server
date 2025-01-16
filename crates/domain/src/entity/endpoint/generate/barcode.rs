#[derive(Debug)]
pub enum Barcode {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {
    Data(Vec<String>),
}
