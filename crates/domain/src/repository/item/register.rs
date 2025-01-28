use crate::{entity::data_type::register_item::RegisterItemData, value_object::error::AppError};
use async_std::future::Future;

pub trait RegisterItemRepository {
    fn new(register_item_data: RegisterItemData) -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
    fn register(&self) -> impl Future<Output = Result<(), AppError>> + Send;
}
