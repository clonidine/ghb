pub mod cat;
pub mod config;
pub mod user;

use color_eyre::owo_colors::OwoColorize;

fn credits() {
    println!(
        "{}",
        "This selfbot was created by clonidine. Check out clonidine's GitHub at https://github.com/clonidine.".bright_blue(),
    );
}

#[tokio::main]
async fn main() {
    credits();
    println!("{}", "Starting selfbot...".red());
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let config_content = match std::fs::read_to_string("config.json") {
        Ok(content) => content,
        Err(_) => {
            eprintln!(
                "{}",
                "Error reading config.json. Make sure the file exists and contains valid JSON.",
            );
            return;
        }
    };

    let config = match config::load_config(&config_content) {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!(
                "{}",
                "Error parsing config.json. Make sure the file contains valid JSON.".red(),
            );
            return;
        }
    };

    let user = user::User::new(&config.auth_token);

    user::User::process_messages(
        &user,
        config.channel_id,
        config.user_id,
        config.messages_to_delete,
        config.delay_seconds,
    )
    .await;

    println!();
    println!();
    cat::print_giant_cat();
}
