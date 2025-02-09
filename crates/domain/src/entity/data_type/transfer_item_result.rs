use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum TransferItemResultEnum {
    Ok,
    Err,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TransferItemResult {
    pub new_parent_visible_id: String,
    pub visible_id: String,
    pub result: TransferItemResultEnum,
}
