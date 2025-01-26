#[derive(Clone)]
pub struct SharedState {}

pub type RwLockSharedState = Arc<RwLock<SharedState>>;
