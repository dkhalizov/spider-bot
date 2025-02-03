use crate::bot::bot::TarantulaBot;
use crate::BotResult;
use std::sync::Arc;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

pub type TarantulaDialogue = Dialogue<DialogueState, InMemStorage<DialogueState>>;

#[derive(Clone, Default)]
pub enum DialogueState {
    #[default]
    Start,

    RecordMolt {
        tarantula_id: i64,
    },

    UpdateColonyCount {
        colony_id: i64,
    },
}

impl TarantulaBot {
    pub fn dialogue_handler() -> Handler<'static, DependencyMap, BotResult<()>, DpHandlerDescription>
    {
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<DialogueState>, DialogueState>()
            .branch(dptree::case![DialogueState::Start].endpoint(Self::handle_start))
            .branch(
                dptree::case![DialogueState::RecordMolt { tarantula_id }]
                    .endpoint(Self::handle_molt_dialogue),
            )
            .branch(
                dptree::case![DialogueState::UpdateColonyCount { colony_id }]
                    .endpoint(Self::handle_colony_count),
            )
    }
    async fn handle_start(dialogue: TarantulaDialogue, msg: Message) -> BotResult<()> {
        dialogue.exit().await?;
        Ok(())
    }

    async fn handle_molt_dialogue(
        bot: Arc<TarantulaBot>,
        dialogue: TarantulaDialogue,
        tarantula_id: i64,
        msg: Message,
    ) -> BotResult<()> {
        match msg.text().map(|text| text.parse::<f32>()) {
            Some(Ok(size)) => {
                bot.bot
                    .send_message(msg.chat.id, format!("Recording molt with size: {}cm", size))
                    .await?;

                bot.record_molt_command(
                    msg.chat.id,
                    msg.id,
                    tarantula_id,
                    size,
                    msg.from.unwrap().id.0,
                )
                .await?;

                dialogue.exit().await?;
            }
            _ => {
                bot.bot
                    .send_message(
                        msg.chat.id,
                        "Please send me the size in centimeters (e.g., 12.5)",
                    )
                    .await?;
            }
        }
        Ok(())
    }

    async fn handle_colony_count(
        bot: Arc<TarantulaBot>,
        dialogue: TarantulaDialogue,
        colony_id: i64,
        msg: Message,
    ) -> BotResult<()> {
        match msg.text().map(|text| text.parse::<i32>()) {
            Some(Ok(count)) => {
                bot.bot
                    .send_message(msg.chat.id, format!("Updating colony count by: {}", count))
                    .await?;
                bot.colony_count_update(
                    msg.chat.id,
                    msg.id,
                    colony_id,
                    count,
                    msg.from.unwrap().id.0,
                )
                .await?;

                dialogue.exit().await?;
            }
            _ => {
                bot.bot
                    .send_message(
                        msg.chat.id,
                        "Please send me the count adjustment (e.g., +5 or -3)",
                    )
                    .await?;
            }
        }
        Ok(())
    }
}
