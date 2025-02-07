use crate::{entity::data_type::generate::GenerateData, value_object::error::AppError};
use async_std::future::Future;
use entity::label::Record;

pub trait GenerateRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn generate(
        &self,
        generate_interface: GenerateInterface,
    ) -> impl Future<Output = Result<GenerateData, AppError>> + Send;
}

pub struct GenerateInterface {
    pub quantity: u32,
    pub record: Record,
}

impl GenerateInterface {
    pub async fn new(quantity: u32, record: Record) -> Self {
        Self { quantity, record }
    }
}
