mod bot;
mod db;
mod error;
mod models;

use crate::bot::bot::TarantulaBot;
use crate::error::BotError;
use rusqlite::Result;

pub type BotResult<T> = Result<T, BotError>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| BotError::OperationError("TELEGRAM_BOT_TOKEN not set".to_string()))?;

    let bot = TarantulaBot::new(&token);

    log::info!("Starting tarantula management bot...");
    bot.run().await;

    Ok(())
}
