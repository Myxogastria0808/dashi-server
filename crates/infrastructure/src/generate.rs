use crate::{connection, generate::generate_module::generate};
use domain::{
    repository::{connection::ConnectionRepository, generate::GenerateRepository},
    value_object::error::AppError,
};
use entity::label::Record;

pub mod generate_module;

#[derive(Clone)]
pub struct Generate {
    pub quantity: u32,
    pub qr_or_barcode: Record,
}

impl GenerateRepository for Generate {
    async fn new(quantity: u32, qr_or_barcode: Record) -> Self {
        Self {
            quantity,
            qr_or_barcode,
        }
    }
    async fn generate(&self) -> Result<Vec<String>, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = generate(rdb.to_owned(), self.quantity, self.qr_or_barcode.to_owned()).await;
        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e.into()),
        }
    }
}
