use crate::bot::commands::Command;
use crate::bot::notifications::NotificationSystem;
use crate::db::db::TarantulaDB;
use crate::error::BotError;
use crate::models::enums::HealthStatus;
use crate::models::feeding::FeedingEvent;
use crate::models::models::DbDateTime;
use crate::models::user::TelegramUser;
use crate::BotResult;
use chrono::{NaiveDateTime, Utc};
use std::env;
use std::sync::Arc;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{CallbackQuery, ChatId, Message, Requester, Update};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
use teloxide::utils::command::BotCommands;
use teloxide::{dptree, filter_command, Bot};

#[derive(Clone)]
pub struct TarantulaBot {
    bot: Bot,
    pub(crate) db: Arc<TarantulaDB>,
    notification_system: Arc<NotificationSystem>,
}

impl TarantulaBot {
    pub fn new(token: &str) -> Self {
        let bot = Bot::new(token);
        let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "tarantulas.sqlite".to_string());
        let db = Arc::new(TarantulaDB::new(&db_path).expect("Failed to open database"));
        let notification_system = Arc::new(NotificationSystem::new(bot.clone(), db.clone()));

        Self {
            bot,
            db,
            notification_system,
        }
    }

    pub async fn run(self) {
        let notification_system_clone = self.notification_system.clone();
        tokio::spawn(async move {
            let system = (*notification_system_clone).clone();
            system.start().await;
        });
        let self_clone1 = self.clone();
        let self_clone2 = self.clone();
        let handler = dptree::entry()
            .branch(
                Update::filter_callback_query().endpoint(move |bot: Bot, q: CallbackQuery| {
                    let this = self_clone2.clone();
                    async move { this.handle_callback(bot, q).await }
                }),
            )
            .branch(
                Update::filter_message().branch(filter_command::<Command, _>().endpoint(
                    move |bot: Bot, msg: Message, cmd: Command| {
                        let this = self_clone1.clone();
                        async move { this.handle_command(bot, msg, cmd).await }
                    },
                )),
            ).branch(
            Update::filter_message().endpoint(move |bot: Bot, msg: Message| {
                async move {
                    if let Some(text) = msg.text() {
                        if text.starts_with('/') {
                            let command = text.split_whitespace().next().unwrap_or(text);
                            let help_message = format!(
                                "Unknown command: {}\n\nAvailable commands:\n{}",
                                command,
                                Command::descriptions().to_string()
                            );
                            bot.send_message(msg.chat.id, help_message).await?;
                        } else {
                            bot.send_message(
                                msg.chat.id,
                                "I can only respond to commands. Type /help to see available commands."
                            ).await?;
                        }
                    }
                    Ok(())
                }
            }),
        );

        Dispatcher::builder(self.bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    pub(crate) async fn handle_feed_command(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        tarantula_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let tarantula = self.db.get_tarantula_by_id(user_id, tarantula_id).await?;
        let colonies = self.db.get_colony_status(user_id).await?;

        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = colonies
            .chunks(2)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|colony| {
                        InlineKeyboardButton::callback(
                            format!("{} ({})", colony.colony_name, colony.size_type.to_db_name()),
                            format!("feed_select_colony_{}_{}", tarantula.id, colony.id),
                        )
                    })
                    .collect()
            })
            .collect();

        keyboard.push(vec![InlineKeyboardButton::callback(
            "« Cancel",
            "main_menu",
        )]);

        bot.send_message(
            chat_id,
            format!(
                "Feeding *{}*\nSelect cricket colony to use:",
                tarantula.name
            ),
        )
        .reply_markup(InlineKeyboardMarkup::new(keyboard))
        .parse_mode(ParseMode::Html)
        .await?;

        Ok(())
    }
    pub(crate) async fn handle_feed_colony_selection(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        tarantula_id: i64,
        colony_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let colony = self
            .db
            .get_colony_status(user_id)
            .await?
            .into_iter()
            .find(|c| c.id == colony_id)
            .ok_or_else(|| BotError::NotFound("Colony not found".to_string()))?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback(
                    "1 cricket",
                    format!("feed_confirm_{}_{}_{}", tarantula_id, colony_id, 1),
                ),
                InlineKeyboardButton::callback(
                    "2 crickets",
                    format!("feed_confirm_{}_{}_{}", tarantula_id, colony_id, 2),
                ),
            ],
            vec![
                InlineKeyboardButton::callback(
                    "3 crickets",
                    format!("feed_confirm_{}_{}_{}", tarantula_id, colony_id, 3),
                ),
                InlineKeyboardButton::callback(
                    "5 crickets",
                    format!("feed_confirm_{}_{}_{}", tarantula_id, colony_id, 5),
                ),
            ],
            vec![InlineKeyboardButton::callback("« Cancel", "main_menu")],
        ]);

        bot.send_message(
            chat_id,
            format!(
                "Selected colony: {} ({})\nCurrent count: {}\nHow many crickets?",
                colony.colony_name,
                colony.size_type.to_db_name(),
                colony.current_count
            ),
        )
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;

        Ok(())
    }

    pub(crate) async fn handle_feed_confirmation(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        tarantula_id: i64,
        colony_id: i64,
        count: i32,
        user_id: u64,
    ) -> BotResult<()> {
        let feeding_event = FeedingEvent {
            id: None,
            tarantula_id,
            feeding_date: DbDateTime::default(),
            cricket_colony_id: colony_id,
            number_of_crickets: count,
            feeding_status_id: 1,
            notes: None,
        };

        self.db.record_feeding(user_id, &feeding_event).await?;

        bot.send_message(chat_id, format!("✅ Feeding recorded: {} crickets", count))
            .reply_markup(Self::back_to_menu_keyboard())
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    async fn handle_command(&self, bot: Bot, msg: Message, cmd: Command) -> BotResult<()> {
        let user = msg.from.unwrap();
        let user = TelegramUser {
            telegram_id: user.id.0,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
        };
        self.db.ensure_user_exists(&user).await?;

        let result = match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
                Ok(())
            }
            Command::Start => {
                self.notification_system
                    .register_chat(user.telegram_id, msg.chat.id)
                    .await;
                self.send_welcome_message(&bot, msg.chat.id, user.telegram_id)
                    .await
            }
            Command::AddTarantula(name, species, date, age_months, notes) => {
                self.db
                    .add_tarantula(
                        user.telegram_id,
                        &*name,
                        species,
                        &*date,
                        age_months,
                        None,
                        match notes.as_str() {
                            "-" => None,
                            _ => Some(&*notes),
                        },
                    )
                    .await?;
                self.send_welcome_message(&bot, msg.chat.id, user.telegram_id)
                    .await
            }
            Command::AddColony(colony_name, size_type_id, current_count, container_name, notes) => {
                self.db
                    .add_colony(
                        user.telegram_id,
                        &*colony_name,
                        size_type_id,
                        current_count,
                        &*container_name,
                        match notes.as_str() {
                            "-" => None,
                            _ => Some(&*notes),
                        },
                    )
                    .await?;
                self.send_welcome_message(&bot, msg.chat.id, user.telegram_id)
                    .await
            }
        };

        if let Err(e) = result {
            self.handle_error(&bot, msg.chat.id, e).await?;
        }
        Ok(())
    }

    pub(crate) async fn send_welcome_message(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("🕷 List Tarantulas", "list_tarantulas"),
                InlineKeyboardButton::callback("📊 Status Overview", "status_overview"),
            ],
            vec![
                InlineKeyboardButton::callback("🍽 Due Feedings", "feeding_schedule"),
                InlineKeyboardButton::callback("📝 Record Feeding", "record_feeding"),
            ],
            vec![
                InlineKeyboardButton::callback("🏥 Health Alerts", "health_alerts"),
                InlineKeyboardButton::callback("🔍 Record Health Check", "record_health_check"),
            ],
            vec![
                InlineKeyboardButton::callback("🐾 Recent Molts", "molt_history"),
                InlineKeyboardButton::callback("📝 Record Molt", "record_molt"),
            ],
            vec![
                InlineKeyboardButton::callback("🦗 Colony Status", "colonies"),
                InlineKeyboardButton::callback("🧰 Colony Maintenance", "colony_maintenance"),
            ],
            vec![
                InlineKeyboardButton::callback("🧹 Maintenance Tasks", "maintenance"),
                InlineKeyboardButton::callback("📋 View Records", "view_records"),
            ],
        ]);
        let feeding_due = self.db.get_tarantulas_due_feeding(user_id).await?;
        let health_alerts = self.db.get_health_alerts(user_id).await?;

        let recent_molts = self
            .db
            .get_recent_molt_records(user_id, 100)
            .await?
            .into_iter()
            .filter(|r| {
                if let Ok(date) = NaiveDateTime::parse_from_str(&r.molt_date, "%Y-%m-%d %H:%M:%S") {
                    let now = Utc::now().naive_utc();
                    now.signed_duration_since(date).num_days() <= 30
                } else {
                    false
                }
            })
            .count();

        let message = format!(
            "Welcome to your Tarantula Management System! 🕷\n\n\
        *Quick Stats:*\n\
        • Feeding Due: {}\n\
        • Health Alerts: {}\n\
        • Recent Molts: {} (30 days)",
            feeding_due.len(),
            health_alerts.len(),
            recent_molts
        );
        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_error(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        error: BotError,
    ) -> Result<(), teloxide::RequestError> {
        let error_message = match error {
            BotError::NotFound(msg) => format!("❌ {}", msg),
            BotError::ValidationError(msg) => format!("⚠️ {}", msg),
            BotError::Database(e) => {
                log::error!("Database error: {:?}", e);
                "❌ A database error occurred. Please try again later.".to_string()
            }
            BotError::Telegram(e) => {
                log::error!("Telegram error: {:?}", e);
                "❌ A communication error occurred. Please try again later.".to_string()
            }
            _ => {
                log::error!("Unexpected error: {:?}", error);
                "❌ An unexpected error occurred. Please try again later.".to_string()
            }
        };

        let keyboard = Self::back_to_menu_keyboard();

        bot.send_message(chat_id, error_message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }
    pub(crate) async fn handle_list_tarantulas(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let tarantulas = self.db.get_all_tarantulas(user_id).await?;
        let mut message = String::from("🕷 *Your Tarantulas*\n\n");

        if tarantulas.is_empty() {
            message = String::from("No tarantulas found in the database.");
        }

        for t in &tarantulas {
            let feeding_display = t.days_since_feeding.map_or("Unknown".to_string(), |days| {
                if days < 1.0 {
                    "Today".to_string()
                } else {
                    format!("{:.1} days", days)
                }
            });

            message.push_str(&format!(
                "*{}* ({})\n▫️ Status: {}\n▫️ Last fed: {}\n\n",
                t.name, t.species_name, t.current_status, feeding_display
            ));
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back",
            "main_menu",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_feeding_schedule(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let due_feedings = self.db.get_tarantulas_due_feeding(user_id).await?;

        let mut message = String::from("🍽 *Feeding Schedule*\n\n");
        for t in &due_feedings {
            message.push_str(&format!(
                "*{}* needs feeding\n- Last fed: {} days ago\n\n",
                t.name,
                t.days_since_feeding.unwrap_or(0.0)
            ));
        }

        if due_feedings.is_empty() {
            message = String::from("No feedings currently due! 🎉");
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back",
            "main_menu",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_health_alerts(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let alerts = self.db.get_health_alerts(user_id).await?;

        let mut message = String::from("🏥 *Health Alerts*\n\n");
        for alert in &alerts {
            message.push_str(&format!(
                "*{}* - {}\n- Days in state: {}\n\n",
                alert.name, alert.alert_type, alert.days_in_state
            ));
        }

        if alerts.is_empty() {
            message = String::from("No health alerts! All tarantulas appear healthy. 🎉");
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back",
            "main_menu",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_maintenance(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let tasks = self.db.get_maintenance_tasks(user_id).await?;

        let mut message = String::from("🧹 *Maintenance Tasks*\n\n");
        for task in &tasks {
            message.push_str(&format!(
                "*{}* ({})\n- {}\n- Priority: {}\n\n",
                task.name, task.enclosure_number, task.required_action, task.priority
            ));
        }

        if tasks.is_empty() {
            message = String::from("No maintenance tasks currently due! 🎉");
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back",
            "main_menu",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_colonies(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let colonies = self.db.get_colony_status(user_id).await?;

        let mut message = String::from("🦗 *Cricket Colonies*\n\n");
        for colony in &colonies {
            message.push_str(&format!(
                "*{}* ({}):\n- Current count: {}\n- Used this week: {}\n- Weeks remaining: {:.1}\n\n",
                colony.colony_name,
                colony.size_type.to_db_name(),
                colony.current_count,
                colony.crickets_used_7_days,
                colony.weeks_remaining.unwrap_or(0.0)
            ));
        }

        if colonies.is_empty() {
            message = String::from("No cricket colonies found in the database.");
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back",
            "main_menu",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_record_molt_command(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        tarantula_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        self.db
            .record_molt(tarantula_id, None, None, None, user_id)
            .await?;

        let keyboard = Self::back_to_menu_keyboard();
        bot.send_message(chat_id, "Molt recorded \nThank you!".to_string())
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;
        Ok(())
    }

    pub(crate) async fn handle_health_check_command(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        tarantula_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let tarantula = self.db.get_tarantula_by_id(user_id, tarantula_id).await?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(
                "✅ Healthy",
                format!("health_status_{}_{}", tarantula.id, 1),
            )],
            vec![InlineKeyboardButton::callback(
                "⚠️ Monitor",
                format!("health_status_{}_{}", tarantula.id, 2),
            )],
            vec![InlineKeyboardButton::callback(
                "🚨 Critical",
                format!("health_status_{}_{}", tarantula.id, 3),
            )],
            vec![InlineKeyboardButton::callback("« Cancel", "main_menu")],
        ]);

        bot.send_message(
            chat_id,
            format!(
                "Health check for *{}*\nSelect current health status:",
                tarantula.name
            ),
        )
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;

        Ok(())
    }

    pub(crate) async fn handle_health_status_command(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        tarantula_id: i64,
        health_status: HealthStatus,
        user_id: u64,
    ) -> BotResult<()> {
        self.db
            .record_health_check(user_id, tarantula_id, health_status, None)
            .await?;
        let keyboard = Self::back_to_menu_keyboard();

        bot.send_message(chat_id, "Health status recorded \nThank you!".to_string())
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_colony_maintenance_command(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        colony_name: &str,
        user_id: u64,
    ) -> BotResult<()> {
        let colonies = self.db.get_colony_status(user_id).await?;
        let colony = colonies
            .iter()
            .find(|c| c.colony_name.eq_ignore_ascii_case(colony_name))
            .ok_or_else(|| {
                BotError::NotFound(format!("Colony '{}' not found", colony_name))
            })?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(
                "📝 Update Count",
                format!("colony_count_{}", colony.id),
            )],
            vec![InlineKeyboardButton::callback("« Cancel", "main_menu")],
        ]);

        bot.send_message(
            chat_id,
            format!(
                "*Cricket Colony Maintenance*\n\nColony: {}\nCurrent count: {}\nSize: {}\n\nSelect maintenance action:",
                colony.colony_name, colony.current_count, colony.size_type.to_db_name()
            ),
        )
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }
    pub(crate) async fn handle_status_overview(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let due_feedings = self.db.get_tarantulas_due_feeding(user_id).await?;
        let health_alerts = self.db.get_health_alerts(user_id).await?;
        let colonies = self.db.get_colony_status(user_id).await?;

        let message = format!(
            "*System Overview*\n\n\
            🍽 *Feeding Status*\n\
            • {} tarantulas due feeding\n\
            • Longest wait: {} days\n\n\
            🏥 *Health Status*\n\
            • {} active health alerts\n\
            • {} critical cases\n\n\
            🦗 *Colony Status*\n\
            • {} active colonies\n\
            • Total crickets: {}\n\n\
            🧹 *Maintenance*\n\
            • {} tasks due",
            due_feedings.len(),
            due_feedings
                .iter()
                .map(|t| t.days_since_feeding.unwrap_or(0.0))
                .fold(0.0, f64::max),
            health_alerts.len(),
            health_alerts
                .iter()
                .filter(|a| a.alert_type == "Critical")
                .count(),
            colonies.len(),
            colonies.iter().map(|c| c.current_count).sum::<i32>(),
            0 // TODO: Implement maintenance task count
        );

        let keyboard = Self::back_to_menu_keyboard();

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_record_feeding_menu(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let tarantulas = self.db.get_all_tarantulas(user_id).await?;

        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = tarantulas
            .chunks(2)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|t| {
                        InlineKeyboardButton::callback(
                            format!("{} ({})", t.name, t.species_name),
                            format!("feed_tarantula_{}", t.id),
                        )
                    })
                    .collect()
            })
            .collect();
        let keyboard = Self::with_back_button(keyboard);

        bot.send_message(chat_id, "*Record Feeding*\n\nSelect a tarantula:")
            .reply_markup(InlineKeyboardMarkup::new(keyboard))
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_record_health_check_menu(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let tarantulas = self.db.get_all_tarantulas(user_id).await?;

        let keyboard: Vec<Vec<InlineKeyboardButton>> = tarantulas
            .chunks(2)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|t| {
                        InlineKeyboardButton::callback(
                            format!("{} ({})", t.name, t.species_name),
                            format!("health_check_{}", t.id),
                        )
                    })
                    .collect()
            })
            .collect();
        let keyboard = Self::with_back_button(keyboard);

        bot.send_message(chat_id, "*Health Check*\n\nSelect a tarantula:")
            .reply_markup(InlineKeyboardMarkup::new(keyboard))
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_molt_history(
        &self,
        bot: &Bot,
        chat_id: ChatId,
    ) -> BotResult<()> {
        // TODO: Add database method to fetch molt history
        let message = "Recent molt history will be displayed here.";

        let keyboard = Self::back_to_menu_keyboard();

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_view_records(
        &self,
        bot: &Bot,
        chat_id: ChatId,
    ) -> BotResult<()> {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("Feeding Records", "view_feeding_records"),
                InlineKeyboardButton::callback("Health Records", "view_health_records"),
            ],
            vec![InlineKeyboardButton::callback(
                "Molt Records",
                "view_molt_records",
            )],
            vec![InlineKeyboardButton::callback(
                "« Back to Menu",
                "main_menu",
            )],
        ]);

        bot.send_message(chat_id, "*View Records*\n\nSelect record type:")
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }
    pub(crate) async fn handle_feeding_records(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let records = self.db.get_recent_feeding_records(user_id, 10).await?;

        let mut message = String::from("🍽 *Recent Feeding Records*\n\n");
        if records.is_empty() {
            message.push_str("No feeding records found.");
        } else {
            for record in records {
                message.push_str(&format!(
                    "*{}* - {}\n• {} crickets from {}\n• Status: {}\n{}\n\n",
                    record.tarantula_name,
                    record.feeding_date,
                    record.number_of_crickets,
                    record.colony_name,
                    record.status,
                    record.notes.unwrap_or_default()
                ));
            }
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back to Records",
            "view_records",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_health_records(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let records = self.db.get_recent_health_records(user_id, 10).await?;

        let mut message = String::from("🏥 *Recent Health Check Records*\n\n");
        if records.is_empty() {
            message.push_str("No health check records found.");
        } else {
            for record in records {
                let details = vec![
                    record.weight_grams.map(|w| format!("Weight: {}g", w)),
                    record.humidity_percent.map(|h| format!("Humidity: {}%", h)),
                    record.temperature_celsius.map(|t| format!("Temp: {}°C", t)),
                ];
                let details_str = details.into_iter().flatten().collect::<Vec<_>>().join(", ");

                message.push_str(&format!(
                    "*{}* - {}\n• Status: {}\n• {}\n{}\n\n",
                    record.tarantula_name,
                    record.check_date,
                    record.status,
                    if details_str.is_empty() {
                        "No measurements taken"
                    } else {
                        &details_str
                    },
                    record.notes.unwrap_or_default()
                ));
            }
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back to Records",
            "view_records",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_molt_records(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let records = self.db.get_recent_molt_records(user_id, 10).await?;

        let mut message = String::from("🐾 *Recent Molt Records*\n\n");
        if records.is_empty() {
            message.push_str("No molt records found.");
        } else {
            for record in records {
                message.push_str(&format!(
                    "*{}* - {}\n• Stage: {}\n{}{}• {}\n\n",
                    record.tarantula_name,
                    record.molt_date,
                    record.stage,
                    record
                        .pre_molt_length_cm
                        .map_or(String::new(), |l| format!("• Length: {}cm\n", l)),
                    record
                        .complications
                        .map_or(String::new(), |c| format!("• Complications: {}\n", c)),
                    record.notes.unwrap_or_default()
                ));
            }
        }

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back to Records",
            "view_records",
        )]]);

        bot.send_message(chat_id, message)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    pub(crate) async fn handle_colony_count(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        colony_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let colony = self
            .db
            .get_colony_status(user_id)
            .await?
            .into_iter()
            .find(|c| c.id == colony_id)
            .ok_or_else(|| BotError::NotFound("Colony not found".to_string()))?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback(
                    "-10",
                    format!("colony_count_update_{}_{}", colony_id, -10),
                ),
                InlineKeyboardButton::callback(
                    "-5",
                    format!("colony_count_update_{}_{}", colony_id, -5),
                ),
            ],
            vec![
                InlineKeyboardButton::callback(
                    "+1",
                    format!("colony_count_update_{}_{}", colony_id, 1),
                ),
                InlineKeyboardButton::callback(
                    "+5",
                    format!("colony_count_update_{}_{}", colony_id, 5),
                ),
                InlineKeyboardButton::callback(
                    "+10",
                    format!("colony_count_update_{}_{}", colony_id, 10),
                ),
                InlineKeyboardButton::callback(
                    "+50",
                    format!("colony_count_update_{}_{}", colony_id, 10),
                ),
            ],
            vec![InlineKeyboardButton::callback("« Cancel", "main_menu")],
        ]);

        bot.send_message(
            chat_id,
            format!(
                "*Update Colony Count*\n\nColony: {}\nCurrent count: {}\nSelect adjustment:",
                colony.colony_name, colony.current_count
            ),
        )
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;

        Ok(())
    }

    pub(crate) async fn handle_colony_count_update(
        &self,
        bot: &Bot,
        chat_id: ChatId,
        colony_id: i64,
        adjustment: i32,
        user_id: u64,
    ) -> BotResult<()> {
        self.db
            .update_colony_count(colony_id, adjustment, user_id)
            .await?;

        let keyboard = Self::back_to_menu_keyboard();

        bot.send_message(
            chat_id,
            format!("✅ Colony count updated by {}", adjustment),
        )
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;

        Ok(())
    }

    fn back_to_menu_keyboard() -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back to Menu",
            "main_menu",
        )]])
    }

    fn back_button_keyboard(text: &str, callback: &str) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            format!("« {}", text),
            callback,
        )]])
    }

    fn with_back_button(
        mut keyboard: Vec<Vec<InlineKeyboardButton>>,
    ) -> Vec<Vec<InlineKeyboardButton>> {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "« Back to Menu",
            "main_menu",
        )]);
        keyboard
    }
}
