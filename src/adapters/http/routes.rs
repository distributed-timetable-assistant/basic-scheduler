use std::sync::Arc;

use axum::{Router, routing::get};

use crate::infrastructure::server::state::AppState;

use super::handlers;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(handlers::health))
}
