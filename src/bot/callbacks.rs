use crate::bot::bot::TarantulaBot;
use crate::error::BotError;
use crate::models::enums::HealthStatus;
use crate::BotResult;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
use teloxide::Bot;

impl TarantulaBot {
    pub(crate) async fn handle_callback(
        &self,
        bot: Bot,
        query: CallbackQuery,
    ) -> BotResult<()> {
        log::info!("Received callback query: {:?}", query.data);
        bot.answer_callback_query(query.id).await?;

        let chat_id = if let Some(message) = query.message {
            let _result = bot.delete_message(message.chat().id, message.id()).await;
            message.chat().id
        } else {
            return Ok(());
        };

        let user_id = query.from.id.0;

        let result = match query.data.as_deref() {
            Some("main_menu") => self.send_welcome_message(&bot, chat_id, user_id).await,
            Some("list_tarantulas") => self.handle_list_tarantulas(&bot, chat_id, user_id).await,
            Some("feeding_schedule") => self.handle_feeding_schedule(&bot, chat_id, user_id).await,
            Some("health_alerts") => self.handle_health_alerts(&bot, chat_id, user_id).await,
            Some("maintenance") => self.handle_maintenance(&bot, chat_id, user_id).await,
            Some("colonies") => self.handle_colonies(&bot, chat_id, user_id).await,

            Some("status_overview") => self.handle_status_overview(&bot, chat_id, user_id).await,
            Some("record_feeding") => {
                self.handle_record_feeding_menu(&bot, chat_id, user_id)
                    .await
            }
            Some("record_health_check") => {
                self.handle_record_health_check_menu(&bot, chat_id, user_id)
                    .await
            }
            Some("molt_history") => self.handle_molt_history(&bot, chat_id).await,
            Some("record_molt") => {
                let tarantulas = self.db.get_all_tarantulas(user_id).await?;
                let mut keyboard: Vec<Vec<InlineKeyboardButton>> = tarantulas
                    .chunks(2)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|t| {
                                InlineKeyboardButton::callback(
                                    format!("{} ({})", t.name, t.species_name),
                                    format!("molt_simple_{}", t.id),
                                )
                            })
                            .collect()
                    })
                    .collect();
                keyboard.push(vec![InlineKeyboardButton::callback(
                    "« Back to Menu",
                    "main_menu",
                )]);

                bot.send_message(chat_id, "*Record Molt*\n\nSelect a tarantula:")
                    .reply_markup(InlineKeyboardMarkup::new(keyboard))
                    .parse_mode(ParseMode::Html)
                    .await
                    .map(|_| ())
                    .map_err(|e| e.into())
            }

            Some("colony_maintenance") => {
                let colonies = self.db.get_colony_status(user_id).await?;
                let mut keyboard: Vec<Vec<InlineKeyboardButton>> = colonies
                    .chunks(2)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|c| {
                                InlineKeyboardButton::callback(
                                    format!("{} ({})", c.colony_name, c.size_type.to_db_name()),
                                    format!("colony_maintenance_menu_{}", c.id),
                                )
                            })
                            .collect()
                    })
                    .collect();
                keyboard.push(vec![InlineKeyboardButton::callback(
                    "« Back to Menu",
                    "main_menu",
                )]);

                bot.send_message(chat_id, "*Colony Maintenance*\n\nSelect a colony:")
                    .reply_markup(InlineKeyboardMarkup::new(keyboard))
                    .parse_mode(ParseMode::Html)
                    .await
                    .map(|_| ())
                    .map_err(|e| e.into())
            }
            Some("view_records") => self.handle_view_records(&bot, chat_id).await,

            Some("view_feeding_records") => {
                self.handle_feeding_records(&bot, chat_id, user_id).await
            }
            Some("view_health_records") => self.handle_health_records(&bot, chat_id, user_id).await,
            Some("view_molt_records") => self.handle_molt_records(&bot, chat_id, user_id).await,

            Some(data) if data.starts_with("feed_tarantula_") => {
                let tarantula_id = data
                    .strip_prefix("feed_tarantula_")
                    .unwrap()
                    .parse::<i64>()
                    .map_err(|e| {
                        BotError::ValidationError(format!("Invalid tarantula ID: {}", e))
                    })?;
                self.handle_feed_command(&bot, chat_id, tarantula_id, user_id)
                    .await
            }
            Some(data) if data.starts_with("health_check_") => {
                let tarantula_id = data
                    .strip_prefix("health_check_")
                    .unwrap()
                    .parse::<i64>()
                    .map_err(|e| {
                        BotError::ValidationError(format!("Invalid tarantula ID: {}", e))
                    })?;
                self.handle_health_check_command(&bot, chat_id, tarantula_id, user_id)
                    .await
            }
            Some(data) if data.starts_with("health_status_") => {
                let parts: Vec<&str> = data.splitn(4, '_').collect();
                if parts.len() != 4 {
                    Err(BotError::ValidationError(
                        "Invalid health status data".to_string(),
                    ))
                } else {
                    let tarantula_id = parts[2].parse::<i64>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid tarantula ID: {}", e))
                    })?;
                    let health_status_id = parts[3].parse::<i64>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid health status ID: {}", e))
                    })?;
                    self.handle_health_status_command(
                        &bot,
                        chat_id,
                        tarantula_id,
                        HealthStatus::from_id(health_status_id),
                        user_id,
                    )
                    .await
                }
            }

            Some(data) if data.starts_with("molt_simple_") => {
                let tarantula_id = data
                    .strip_prefix("molt_simple_")
                    .unwrap()
                    .parse::<i64>()
                    .map_err(|e| {
                        BotError::ValidationError(format!("Invalid tarantula ID: {}", e))
                    })?;
                self.handle_record_molt_command(&bot, chat_id, tarantula_id, user_id)
                    .await
            }

            Some(data) if data.starts_with("colony_maintenance_menu_") => {
                let colony_id = data
                    .strip_prefix("colony_maintenance_menu_")
                    .unwrap()
                    .parse::<i64>()
                    .map_err(|e| {
                        BotError::ValidationError(format!("Invalid colony ID: {}", e))
                    })?;
                let colony = self
                    .db
                    .get_colony_status(user_id)
                    .await?
                    .into_iter()
                    .find(|c| c.id == colony_id)
                    .ok_or_else(|| BotError::NotFound("Colony not found".to_string()))?;
                self.handle_colony_maintenance_command(&bot, chat_id, &colony.colony_name, user_id)
                    .await
            }

            Some(data) if data.starts_with("feed_select_colony_") => {
                let parts: Vec<&str> = data.splitn(5, '_').collect();
                if parts.len() == 5 {
                    let tarantula_id = parts[3].parse::<i64>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid tarantula ID: {}", e))
                    })?;
                    let colony_id = parts[4].parse::<i64>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid colony ID: {}", e))
                    })?;
                    self.handle_feed_colony_selection(
                        &bot,
                        chat_id,
                        tarantula_id,
                        colony_id,
                        user_id,
                    )
                    .await
                } else {
                    Err(BotError::ValidationError(
                        "Invalid feed selection data".to_string(),
                    ))
                }
            }
            Some(data) if data.starts_with("feed_confirm_") => {
                let parts: Vec<&str> = data.splitn(5, '_').collect();
                if parts.len() == 5 {
                    let tarantula_id = parts[2].parse::<i64>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid tarantula ID: {}", e))
                    })?;
                    let colony_id = parts[3].parse::<i64>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid colony ID: {}", e))
                    })?;
                    let count = parts[4].parse::<i32>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid cricket count: {}", e))
                    })?;
                    self.handle_feed_confirmation(
                        &bot,
                        chat_id,
                        tarantula_id,
                        colony_id,
                        count,
                        user_id,
                    )
                    .await
                } else {
                    Err(BotError::ValidationError(
                        "Invalid feed confirmation data".to_string(),
                    ))
                }
            }
            Some(data) if data.starts_with("colony_count_") => {
                let colony_id = data
                    .strip_prefix("colony_count_")
                    .unwrap()
                    .parse::<i64>()
                    .map_err(|e| {
                        BotError::ValidationError(format!("Invalid colony ID: {}", e))
                    })?;
                self.handle_colony_count(&bot, chat_id, colony_id, user_id)
                    .await
            }
            Some(data) if data.starts_with("colony_count_update_") => {
                let parts: Vec<&str> = data.splitn(4, '_').collect();
                if parts.len() != 5 {
                    Err(BotError::ValidationError(
                        "Invalid colony count update data".to_string(),
                    ))
                } else {
                    let colony_id = parts[3].parse::<i64>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid colony ID: {}", e))
                    })?;
                    let adjustment = parts[4].parse::<i32>().map_err(|e| {
                        BotError::ValidationError(format!("Invalid adjustment value: {}", e))
                    })?;
                    self.handle_colony_count_update(&bot, chat_id, colony_id, adjustment, user_id)
                        .await
                }
            }
            _ => Err(BotError::ValidationError(
                "Invalid callback data".to_string(),
            )),
        };

        if let Err(e) = result {
            self.handle_error(&bot, chat_id, e).await?;
        }

        Ok(())
    }
}
