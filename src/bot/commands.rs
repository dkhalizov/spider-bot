use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub(crate) enum Command {
    #[command(description = "show this message.")]
    Help,
    #[command(description = "start bot interaction.")]
    Start,
    #[command(description = "add a new tarantula. use /addtarantula name species_id acqusition_date age_months notes ", parse_with = "split")]
    AddTarantula(String, i64, String, i64, String),
    #[command(description = "add a new cricket colony. use /addcolony name size_type_id current_count last_count_date notes", parse_with = "split")]
    AddColony(String, i64, i32, String, String),
}