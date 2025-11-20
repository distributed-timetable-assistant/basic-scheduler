use std::sync::Arc;

use axum::extract::State;

use crate::infrastructure::server::state::AppState;

pub async fn health(State(state): State<Arc<AppState>>) -> String {
    format!(
        "basic scheduler running on {}:{}",
        state.config.server.host, state.config.server.port
    )
}
