use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    app: AppConfiguration,
    mqtt: MqttConfiguration,
    redis: RedisConfiguration,
}

#[derive(Debug, Deserialize)]
struct AppConfiguration {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct MqttConfiguration {
    url: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct RedisConfiguration {
    url: String,
    password: String,
    db: u32,
    dial_timeout: u64,
    read_timeout: u64,
    write_timeout: u64,
    pool_size: u32,
    pool_timeout: u64,
}

pub fn load_config() -> Configuration {
    // 获取环境变量，如果没有设置则默认为 "dev"
    let environment = env::var("RUN_MODE").unwrap_or_else(|_| "dev".to_string());
    let config_file = match environment.as_str() {
        "prod" => "src/config/config_prod.toml",
        _ => "src/config/config_dev.toml",
    };
    let config_content = fs::read_to_string(config_file).expect("Failed to read config file");

    toml::from_str(&config_content).expect("Failed to parse config file")
}
