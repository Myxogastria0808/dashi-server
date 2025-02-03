use async_std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct SharedState {}

pub type RwLockSharedState = Arc<RwLock<SharedState>>;
