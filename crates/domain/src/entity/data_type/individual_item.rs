use serde_json::Value;

#[derive(Debug)]
pub struct IndividualItemData {
    pub id: i32,
    pub visible_id: String,
    pub parent_id: i32,
    pub parent_visible_id: String,
    pub record: entity::label::Record,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<i32>,
    pub purchase_price: Option<i32>,
    pub durability: Option<i32>,
    pub is_depreciation: bool,
    pub connector: Value,
    pub is_rent: bool,
    pub color: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
