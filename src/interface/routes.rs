use crate::interface::state::AppState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn get_router() -> Router<Arc<AppState>>{
    Router::new()
        .route("/", get(|| async { "basic scheduler" }))
}