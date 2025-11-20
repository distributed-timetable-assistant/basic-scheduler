use crate::infrastructure::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}
