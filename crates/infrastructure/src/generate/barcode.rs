use domain::value_object::error::generate::GenerateError;
use entity::label::{self, Entity as Label};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub(super) async fn generate_qr(
    rdb: DatabaseConnection,
    quantity: u16,
) -> Result<Vec<String>, GenerateError> {
    let max_label_model = Label::find()
        .filter(label::Column::IsMax.eq(true))
        .all(&rdb)
        .await;
    Ok(vec!["sample".to_string()])
}
