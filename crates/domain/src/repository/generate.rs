use crate::value_object::error::AppError;
use async_std::future::Future;
use entity::label::Record;

pub trait GenerateRepository {
    fn new(quantity: u32, qr_or_barcode: Record) -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn generate(&self) -> impl Future<Output = Result<Vec<String>, AppError>> + Send;
}
