mod bot;
mod db;
mod error;
mod models;

use crate::bot::bot::TarantulaBot;
use crate::error::TarantulaError;
use rusqlite::Result;

pub type TarantulaResult<T> = Result<T, TarantulaError>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| TarantulaError::OperationError("TELEGRAM_BOT_TOKEN not set".to_string()))?;

    let bot = TarantulaBot::new(&token);

    log::info!("Starting tarantula management bot...");
    bot.run().await;

    Ok(())
}
