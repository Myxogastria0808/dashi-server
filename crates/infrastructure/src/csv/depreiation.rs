use depreiation_modules::depreiation;
use domain::{
    entity::data_type::depreiation_csv::DepreiationCsvData,
    repository::{connection::ConnectionRepository, csv::depreiation::DepreiationCsvRepository},
    value_object::error::AppError,
};

use crate::connection;

pub mod depreiation_modules;

#[derive(Clone)]
pub struct DepreiationCsv;

impl DepreiationCsvRepository for DepreiationCsv {
    async fn new() -> Self {
        Self {}
    }
    async fn depreiation_csv(&self) -> Result<Vec<DepreiationCsvData>, AppError> {
        let rdb = connection::CollectConnection::connect_rdb().await?;
        let result = depreiation(rdb).await?;
        Ok(result)
    }
}
