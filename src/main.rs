mod domain;
mod usecases;
mod infrastructure;
mod interface;

use crate::interface::state::AppState;
use infrastructure::config::app_config::AppConfig;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    const PARSE_ADDR_ERR: &str = "Pars Address -> FAILED: Invalid address format";

    let config = AppConfig::load();

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect(PARSE_ADDR_ERR);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let state = AppState {
        config
    };
    let app = interface::routes::get_router().with_state(Arc::new(state));

    axum::serve(listener, app).await.unwrap();
}
