use std::env;
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;

static DEFAULT_YAML: &str = include_str!("../../../config/default.yml");

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Self {
        const GET_CWD_ERR: &str = "Get Current Working Directory -> FAILED";
        const CFG_LOAD_ERR: &str = "Config Load -> FAILED";
        const CFG_DESERIALIZE_ERR: &str = "Config Deserialize -> FAILED";

        let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
        let cwd = env::current_dir().expect(GET_CWD_ERR);

        let mut config_builder = Config::builder().add_source(File::from_str(DEFAULT_YAML, FileFormat::Yaml));

        let conf_paths = vec![
            (cwd.join("../../config/default.yml"), false),
            (cwd.join(&format!("config/{}.yml", env)), false),
            (cwd.join("../../config/local.yml"), false),
            (cwd.join("/etc/basic-scheduler/config.yml"), false),
        ];
        for (config_path, required) in conf_paths {
            config_builder = config_builder.add_source(File::from(config_path).required(required));
        }

        config_builder
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
            .expect(CFG_LOAD_ERR)
            .try_deserialize()
            .expect(CFG_DESERIALIZE_ERR)
    }
}