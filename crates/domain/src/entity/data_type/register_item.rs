use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterItemData {
    pub parent_visible_id: String,
    pub visible_id: String,
    pub name: String,
    pub product_number: String,
    pub description: String,
    pub purchase_year: Option<i32>,
    pub purchase_price: Option<i32>,
    pub durability: Option<i32>,
    pub is_depreciation: bool,
    pub connector: Vec<String>,
    pub color: String,
}
