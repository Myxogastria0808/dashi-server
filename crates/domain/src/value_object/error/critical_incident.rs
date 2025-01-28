//This func is used to rollback the transaction in case of any error
pub async fn rollback_error() {
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    tracing::error!("Rollback Error: A critical incident has occurred.");
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}

pub async fn conflict_error() {
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    tracing::error!("Conflict Error: A critical incident has occurred.");
    tracing::error!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}
