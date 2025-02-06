use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransferItemData {
    pub new_parent_visible_id: String,
    pub visible_id: String,
}
