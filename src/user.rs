use color_eyre::owo_colors::OwoColorize;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde_json::Value;
use std::time::Duration;

use crate::cat;

const BASE_API: &str = "https://discord.com/api/v9";

pub struct User {
    pub client: reqwest::Client,
    pub headers: HeaderMap,
}

impl User {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, token.parse().unwrap());

        Self {
            client: reqwest::Client::new(),
            headers,
        }
    }

    pub async fn get_messages(&self, target_id: u64) -> Result<Vec<Value>, reqwest::Error> {
        let msg_api = format!("{}/channels/{}/messages", BASE_API, target_id);
        let res = self
            .client
            .get(&msg_api)
            .headers(self.headers.clone())
            .send()
            .await?;

        let messages: Vec<Value> = res.json().await?;
        Ok(messages)
    }

    pub async fn delete_message(
        &self,
        target_id: u64,
        message_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let delete_msg_api = format!(
            "{}/channels/{}/messages/{}",
            BASE_API, target_id, message_id
        );

        let _ = self
            .client
            .delete(&delete_msg_api)
            .headers(self.headers.clone())
            .send()
            .await?;

        Ok(())
    }

    pub async fn process_messages(
        &self,
        channel_id: u64,
        user_id: u64,
        messages_to_delete: usize,
        delay_seconds: u64,
    ) {
        let mut has_more_messages = true;

        while has_more_messages {
            cat::print_giant_cat();
            match self.get_messages(channel_id).await {
                Ok(messages) => {
                    let message_ids: Vec<&str> = messages
                        .iter()
                        .filter(|value| {
                            value
                                .get("author")
                                .and_then(|author| author.get("id").and_then(|id| id.as_str()))
                                .map_or(false, |id| id.parse::<u64>().unwrap() == user_id)
                        })
                        .take(messages_to_delete)
                        .filter_map(|message| message.get("id").and_then(|id| id.as_str()))
                        .collect();

                    if !message_ids.is_empty() {
                        for message_id in &message_ids {
                            if let Err(err) = self.delete_message(channel_id, message_id).await {
                                eprintln!(
                                    "{} {}: {:?}",
                                    "Error deleting message".red(),
                                    message_id.white(),
                                    err.red()
                                );
                            } else {
                                println!(
                                    "{}: {}",
                                    "Message deleted".bright_magenta(),
                                    message_id.white()
                                );
                            }
                        }
                    } else {
                        has_more_messages = false;
                    }
                }
                Err(err) => {
                    eprintln!("{}: {:?}", "Error retrieving messages".red(), err.red());
                    has_more_messages = false;
                }
            }

            println!();
            println!(
                "{} {}{}",
                "Waiting".red(),
                delay_seconds.bright_magenta(),
                "...".red()
            );
            tokio::time::sleep(Duration::from_secs(delay_seconds)).await;
            println!()
        }
    }
}
