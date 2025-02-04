use crate::value_object::error::AppError;
use async_std::future::Future;

pub trait DeleteItemRepository {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn delete(
        &self,
        delete_item_data: DeleteItemInterface,
    ) -> impl Future<Output = Result<(), AppError>> + Send;
}

pub struct DeleteItemInterface {
    pub visible_id: String,
}

impl DeleteItemInterface {
    pub async fn new(visible_id: String) -> Self {
        Self { visible_id }
    }
}
