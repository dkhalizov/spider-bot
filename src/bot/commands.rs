use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub(crate) enum Command {
    #[command(description = "show this message.")]
    Help,
    #[command(description = "start bot interaction.")]
    Start,
}