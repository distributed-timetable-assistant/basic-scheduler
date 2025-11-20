mod adapters;
mod core;
mod infrastructure;

use std::net::SocketAddr;
use std::sync::Arc;

use adapters::http::routes::router;
use infrastructure::config::AppConfig;
use infrastructure::server::state::AppState;

#[tokio::main]
async fn main() {
    const PARSE_ADDR_ERR: &str = "Parse Address -> FAILED: Invalid address format";

    let config = AppConfig::load();

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port)
        .parse()
        .expect(PARSE_ADDR_ERR);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let state = AppState { config };
    let app = router().with_state(Arc::new(state));

    axum::serve(listener, app).await.unwrap();
}
