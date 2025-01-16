pub mod barcode;
pub mod qr;

#[derive(Debug)]
pub enum Generate {
    Barcode(barcode::Barcode),
    Qr(qr::Qr),
}
