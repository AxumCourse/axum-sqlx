use std::sync::Arc;

pub struct AppState {
    pub pool: Arc<sqlx::MySqlPool>,
}
