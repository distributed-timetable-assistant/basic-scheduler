use crate::infrastructure::config::app_config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
}