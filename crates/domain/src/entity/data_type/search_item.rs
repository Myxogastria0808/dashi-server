use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchItemData {
    pub id: i32,
    pub visible_id: String,
    pub name: String,
    pub connector: Vec<String>,
    pub is_rent: bool,
    pub color: String,
}
