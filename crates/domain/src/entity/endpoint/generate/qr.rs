#[derive(Debug)]
pub enum Qr {
    Response,
    Request,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {
    Data(Vec<String>),
}
