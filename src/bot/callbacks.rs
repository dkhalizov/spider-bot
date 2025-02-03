use crate::bot::bot::TarantulaBot;
use crate::models::enums::HealthStatus;
use crate::BotError;
use crate::BotResult;
use async_trait::async_trait;
use bot_macros::BotCallback;
use std::sync::Arc;
use teloxide::dispatching::dialogue::{GetChatId};
use teloxide::prelude::{CallbackQuery, Requester};

#[derive(BotCallback, Debug, Clone)]
pub enum BotCallback {
    MainMenu,
    ListTarantulas,
    FeedingSchedule,
    HealthAlerts,
    Maintenance,
    Colonies,
    StatusOverview,
    RecordFeeding,
    RecordHealthCheck,
    MoltHistory,
    RecordMolt,
    ColonyMaintenance,
    ViewRecords,
    ViewFeedingRecords,
    ViewHealthRecords,
    ViewMoltRecords,

    FeedTarantula(i64),
    HealthCheck(i64),
    HealthStatus(i64, i64), // tarantula_id, health_status_id
    MoltSimple(i64),        // size cm after, tarantula_id
    ColonyMaintenanceMenu(i64),
    FeedSelectColony(i64, i64), // tarantula_id, colony_id
    FeedConfirm(i64, i64, i32), // tarantula_id, colony_id, count
    ColonyGetCount(i64),
    ColonyCountUpdate(i64, i32), // colony_id, adjustment

    ViewFeedingSchedule(i64), // tarantula_id
}

#[async_trait]
trait CallbackCommand {
    async fn callback(&self, bot: Arc<TarantulaBot>, query: CallbackQuery) -> BotResult<()>;
}

impl TarantulaBot {
    pub(crate) async fn handle_callback(
        &self,
        arc: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(data) = &query.data {
            log::debug!("Received callback query: {:?}", data);
        }

        arc.bot.answer_callback_query(query.id.clone()).await?;

        if let Some(data) = query.clone().data {
            let callback = data.parse::<BotCallback>()?;
            callback.callback(arc.clone(), query).await?;
        }

        Ok(())
    }
}

impl BotCallback {
    async fn handle_main_menu(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            bot.send_welcome_message(chat_id, query.from.id.0).await?;
        };
        Ok(())
    }

    async fn handle_list_tarantulas(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        //log::info!("{:?}", query);
        if let Some(chat_id) = query.chat_id() {
            bot.list_tarantulas(chat_id, query.message.unwrap().id(), query.from.id.0)
                .await?;
        };
        Ok(())
    }

    async fn handle_feeding_schedule(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.feeding_schedule(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_feed_tarantula(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        tarantula_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.feed_command(chat_id, msg.id(), *tarantula_id, query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_health_alerts(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.health_alerts(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_maintenance(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.maintenance(chat_id, msg.id(), query.from.id.0).await?;
            }
        };
        Ok(())
    }
    async fn handle_colonies(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.colonies(chat_id, msg.id(), query.from.id.0).await?;
            }
        };
        Ok(())
    }

    async fn handle_status_overview(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.status_overview(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }
    async fn handle_record_feeding(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.record_feeding_menu(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }
    async fn handle_record_health_check(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.record_health_check_menu(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_molt_history(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.molt_history(chat_id, msg.id(), query.from.id.0).await?;
            }
        };
        Ok(())
    }

    async fn handle_record_molt(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.record_molt_menu(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_colony_maintenance(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.colony_maintenance(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_view_records(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.view_records(chat_id, msg.id()).await?;
            }
        };
        Ok(())
    }

    async fn handle_view_feeding_records(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.view_feeding_records(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_view_health_records(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.view_health_records(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_view_molt_records(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.view_molt_records(chat_id, msg.id(), query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_health_check(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        tarantula_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.health_check_command(chat_id, msg.id(), *tarantula_id, query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_health_status(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        tarantula_id: &i64,
        health_status_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.health_status_command(
                    chat_id,
                    msg.id(),
                    *tarantula_id,
                    HealthStatus::from_id(*health_status_id),
                    query.from.id.0,
                )
                .await?;
            }
        };
        Ok(())
    }

    async fn handle_molt_simple(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        tarantula_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            bot.molt_simple_callback(chat_id, *tarantula_id).await?
        };
        Ok(())
    }

    async fn handle_colony_maintenance_menu(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        colony_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.colony_maintenance_menu(chat_id, msg.id(), *colony_id, query.from.id.0)
                    .await?;
            }
        }
        Ok(())
    }

    async fn handle_feed_select_colony(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        tarantula_id: &i64,
        colony_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.feed_colony_selection(
                    chat_id,
                    msg.id(),
                    *tarantula_id,
                    *colony_id,
                    query.from.id.0,
                )
                .await?;
            }
        };
        Ok(())
    }

    async fn handle_feed_confirm(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        tarantula_id: &i64,
        colony_id: &i64,
        count: &i32,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.feed_confirmation(
                    chat_id,
                    msg.id(),
                    *tarantula_id,
                    *colony_id,
                    *count,
                    query.from.id.0,
                )
                .await?;
            }
        };
        Ok(())
    }

    async fn handle_colony_get_count(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        colony_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.colony_count(chat_id, msg.id(), *colony_id, query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }

    async fn handle_colony_count_update(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        colony_id: &i64,
        adjustment: &i32,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.colony_count_update(
                    chat_id,
                    msg.id(),
                    *colony_id,
                    *adjustment,
                    query.from.id.0,
                )
                .await?;
            }
        };
        Ok(())
    }
    async fn handle_view_feeding_schedule(
        &self,
        bot: &Arc<TarantulaBot>,
        query: CallbackQuery,
        tarantula_id: &i64,
    ) -> BotResult<()> {
        if let Some(chat_id) = query.chat_id() {
            if let Some(msg) = query.message {
                bot.view_feeding_schedule(chat_id, msg.id(), *tarantula_id, query.from.id.0)
                    .await?;
            }
        };
        Ok(())
    }
}
