use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub auth_token: String,
    pub channel_id: u64,
    pub user_id: u64,
    pub messages_to_delete: usize,
    pub delay_seconds: u64,
}

pub fn load_config(content: &str) -> Result<Config, serde_json::Error> {
    serde_json::from_str(content)
}
