use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GenerateDataRequest {
    pub record: String,
    pub quantity: u32,
}
