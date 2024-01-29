pub mod cat;
pub mod config;
pub mod user;

use termion::color;

fn credits() {
    println!(
        "{}This selfbot was created by lcly. Check out lcly's GitHub at https://github.com/clonidine.{}",
        color::Fg(color::LightBlue),
        color::Fg(color::Reset)
    );
}

#[tokio::main]
async fn main() {
    credits();

    let config_content = match std::fs::read_to_string("config.json") {
        Ok(content) => content,
        Err(_) => {
            eprintln!(
                "{}Error reading config.json. Make sure the file exists and contains valid JSON.{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
            );
            return;
        }
    };

    let config = match config::load_config(&config_content) {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!(
                "{}Error parsing config.json. Make sure the file contains valid JSON.{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
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

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!();
    println!();
    cat::print_giant_cat();
}
