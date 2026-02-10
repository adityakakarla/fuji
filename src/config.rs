use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AppConfig {
    grok_api_key: String,
}

pub fn get_api_key() -> String {
    String::new()
}

pub fn save_api_key(api_key: &String) {}

pub fn delete_api_key() {}
