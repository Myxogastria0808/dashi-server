use crate::{connection, generate::generate_module::generate};
use domain::{
    entity::data_type::generate::GenerateData,
    repository::{
        connection::ConnectionRepository,
        generate::{GenerateInterface, GenerateRepository},
    },
    value_object::error::AppError,
};

pub mod generate_module;

#[derive(Clone)]
pub struct Generate;

impl GenerateRepository for Generate {
    async fn new() -> Self {
        Self {}
    }
    async fn generate(
        &self,
        generate_interface: GenerateInterface,
    ) -> Result<GenerateData, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = generate(
            rdb.to_owned(),
            generate_interface.quantity,
            generate_interface.record,
        )
        .await;
        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e.into()),
        }
    }
}
