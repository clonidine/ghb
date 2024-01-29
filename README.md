# Discord Bulk Message Deletion Selfbot

**WARNING:** The use of selfbots and automated tools to interact with the Discord API may violate Discord's Terms of Service. Use this selfbot responsibly, and be aware that misuse can lead to account suspension or banning. By using this selfbot, you acknowledge the risks and agree that the creator (clonidine) is not responsible for any consequences.


## Introduction

This Discord selfbot allows you to delete multiple messages at once in a specific channel. Before using the selfbot, it's crucial to understand the configuration options in the `config.json` file.

## Configuration

The `config.json` file contains several settings that control how the selfbot behaves. Here's an explanation of each attribute:

- **auth_token**: This is like a secret key that enables the selfbot to connect to Discord securely. Keep it confidential to ensure the security of your account.

- **channel_id**: This is the unique identifier for the Discord channel where you want the selfbot to delete messages. You can find the channel ID by right-clicking on the channel and selecting "Copy ID" in Discord.

- **user_id**: This is your unique identifier in Discord. The selfbot will only delete messages from this user to prevent accidental deletions.

- **messages_to_delete**: This setting determines how many messages the selfbot will attempt to delete in each round. Adjust this based on how many messages you want to delete at once.

- **delay_seconds**: The selfbot pauses for a specific duration (in seconds) between each round of message deletion. Adjust this to prevent reaching Discord's rate limits.

## Configuration example

```json
{
    "auth_token": "YOUR_DISCORD_TOKEN",
    "channel_id": 123456789012345678,  // Replace with your target channel ID
    "user_id": 987654321098765432,    // Replace with your user ID
    "messages_to_delete": 2,
    "delay_seconds": 30
}
```

## Build Requirements

To build this selfbot from source, ensure you have [Rust](https://www.rust-lang.org/) installed on your system.

## Usage

Follow these steps to use the selfbot:

1. Clone the repository to your computer.
2. Open the `config.json` file in a text editor.
3. Fill in the values for each attribute based on your preferences.
4. Save the `config.json` file.
5. Run the selfbot using the appropriate command. For example:
   ```bash
   cargo run
   ```
