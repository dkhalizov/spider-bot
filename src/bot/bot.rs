use crate::bot::callbacks::BotCallback;
use crate::bot::callbacks::BotCallback::ColonyCountUpdate;
use crate::bot::callbacks::BotCallback::ColonyGetCount;
use crate::bot::callbacks::BotCallback::ColonyMaintenanceMenu;
use crate::bot::callbacks::BotCallback::FeedTarantula;
use crate::bot::callbacks::BotCallback::MainMenu;
use crate::bot::callbacks::BotCallback::MoltSimple;
use crate::bot::commands::Command;
use crate::bot::dialog::DialogueState;
use crate::bot::keyboards::{
    feed_command_keyboard, feed_count_selection_keyboard, welcome_keyboard,
};
use crate::bot::notifications::NotificationSystem;
use crate::db::db::{AddColonyParams, AddTarantulaParams, TarantulaDB, TarantulaOperations};
use crate::error::BotError;
use crate::models::cricket::ColonyStatus;
use crate::models::enums::HealthStatus;
use crate::models::feeding::FeedingEvent;
use crate::models::models::DbDateTime;
use crate::models::user::TelegramUser;
use crate::BotResult;
use chrono::{NaiveDateTime, Utc};
use future::BoxFuture;
use futures_core::future;
use std::collections::BTreeMap;
use std::env;
use std::fmt::Debug;
use std::sync::Arc;
use teloxide::dispatching::dialogue::{InMemStorage, Storage};
use teloxide::dispatching::{Dispatcher, DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree::Handler;
use teloxide::error_handlers::ErrorHandler;
use teloxide::payloads::{EditMessageReplyMarkupSetters, SendMessageSetters};
use teloxide::prelude::{CallbackQuery, ChatId, DependencyMap, Message, Requester, Update};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageId, ParseMode};
use teloxide::utils::command::BotCommands;
use teloxide::{dptree, filter_command, Bot, RequestError};
use BotCallback::ListTarantulas;

#[derive(Clone)]
pub struct TarantulaBot {
    pub(crate) bot: Bot,
    pub(crate) db: Arc<TarantulaDB>,
    pub(crate) notification_system: Arc<NotificationSystem>,
    dialogue: Arc<InMemStorage<DialogueState>>,
}

pub struct ChanErrHandler {
    bot: Bot,
}

impl<E> ErrorHandler<E> for ChanErrHandler
where
    E: Debug + Sync + Send + 'static,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        let default_chat = env::var("DEFAULT_CHAT_ID").unwrap();
        Box::pin(async move {
            let _ = self
                .bot
                .send_message(default_chat, format!("{:?}", error))
                .await
                .inspect_err(|e| log::error!("error {} while processing {:?}", e, error));
        })
    }
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
            dialogue: InMemStorage::<DialogueState>::new(),
        }
    }

    pub async fn run(self) {
        let arc_notif_system = self.notification_system.clone();
        tokio::spawn((*arc_notif_system).clone().start());
        let handler = Self::build_handler();

        let mut container = DependencyMap::new();
        let arc = Arc::new(self.clone());
        container.insert(arc.clone());
        container.insert(self.dialogue.clone());
        Dispatcher::builder(
            (*arc).clone().bot,
            dptree::entry()
                .branch(handler)
                .branch(TarantulaBot::dialogue_handler()),
        )
        .dependencies(container)
        .error_handler(Arc::new(ChanErrHandler {
            bot: (*arc).clone().bot,
        }))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    }

    fn build_handler() -> Handler<'static, DependencyMap, BotResult<()>, DpHandlerDescription> {
        let handler = dptree::entry()
            .branch(Update::filter_callback_query().endpoint(
                move |a: Arc<TarantulaBot>, q: CallbackQuery| async move {
                    a.handle_callback(&a.clone(), q).await
                },
            ))
            .branch(
                Update::filter_message().branch(filter_command::<Command, _>().endpoint(
                    move |a: Arc<TarantulaBot>, msg: Message, cmd: Command| async move {
                        a.handle_command(msg, cmd).await
                    },
                )),
            );
        handler
    }

    async fn handle_command(&self, msg: Message, cmd: Command) -> BotResult<()> {
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
                self.bot
                    .send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
                Ok(())
            }
            Command::Start => {
                self.notification_system
                    .register_chat(user.telegram_id, msg.chat.id)
                    .await;
                self.send_welcome_message(msg.chat.id, user.telegram_id)
                    .await
            }
            Command::AddTarantula(name, species, date, age_months, notes) => {
                self.db
                    .add_tarantula(
                        user.telegram_id,
                        AddTarantulaParams {
                            name,
                            species_id: species,
                            acquisition_date: date,
                            estimated_age_months: age_months,
                            notes: Some(notes),
                            enclosure_number: None,
                        },
                    )
                    .await?;
                self.send_welcome_message(msg.chat.id, user.telegram_id)
                    .await
            }
            Command::AddColony(colony_name, size_type_id, current_count, container_name, notes) => {
                self.db
                    .add_colony(
                        user.telegram_id,
                        AddColonyParams{
                            colony_name,
                            size_type_id,
                            current_count,
                            container_number: container_name,
                            notes: Some(notes)
                        }
                    )
                    .await?;
                self.send_welcome_message(msg.chat.id, user.telegram_id)
                    .await
            }
        };

        if let Err(e) = result {
            self.handle_command_error(msg.chat.id, e).await?;
        }
        Ok(())
    }

    async fn handle_command_error(
        &self,
        chat_id: ChatId,
        error: BotError,
    ) -> Result<(), RequestError> {
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

        self.reply_with_send(chat_id, error_message, Some(keyboard))
            .await
            .inspect_err(|e| log::error!("something went wrong {}", e))
            .expect("TODO: panic message");
        Ok(())
    }

    pub(crate) async fn feed_command(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        tarantula_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let tarantula = self.db.get_tarantula_by_id(user_id, tarantula_id).await?;
        let colonies = self.db.get_colony_status(user_id).await?;
        let keyboard = feed_command_keyboard(tarantula_id, colonies);
        self.replay_with_edit(
            chat_id,
            message_id,
            format!(
                "Feeding *{}*\nSelect cricket colony to use:",
                tarantula.name
            ),
            InlineKeyboardMarkup::new(keyboard),
        )
        .await
    }

    pub(crate) async fn feed_colony_selection(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        tarantula_id: i64,
        colony_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let colony = self.colony_status(colony_id, user_id).await?;
        let keyboard = feed_count_selection_keyboard(tarantula_id, colony_id);
        self.replay_with_edit(
            chat_id,
            message_id,
            format!(
                "Selected colony: {} ({})\nCurrent count: {}\nHow many crickets?",
                colony.colony_name,
                colony.size_type.to_db_name(),
                colony.current_count
            ),
            keyboard,
        )
        .await
    }

    async fn colony_status(&self, colony_id: i64, user_id: u64) -> Result<ColonyStatus, BotError> {
        let colony = self
            .db
            .get_colony_status(user_id)
            .await?
            .into_iter()
            .find(|c| c.id == colony_id)
            .ok_or_else(|| BotError::NotFound("Colony not found".to_string()))?;
        Ok(colony)
    }

    pub(crate) async fn feed_confirmation(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        self.db.record_feeding(user_id, feeding_event).await?;

        self.replay_with_edit(
            chat_id,
            message_id,
            format!("✅ Feeding recorded: {} crickets", count),
            Self::back_to_menu_keyboard(),
        )
        .await
    }

    pub(crate) async fn send_welcome_message(
        &self,
        chat_id: ChatId,
        user_id: u64,
    ) -> BotResult<()> {
        let keyboard = welcome_keyboard();
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
        self.reply_with_send(chat_id, message, Some(keyboard)).await
    }

    pub(crate) async fn list_tarantulas(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        let keyboard = Self::back_to_menu_keyboard();

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    async fn replay_with_edit(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        message: String,
        keyboard: InlineKeyboardMarkup,
    ) -> BotResult<()> {
        self.bot
            .edit_message_text(chat_id, message_id, message)
            .await
            .map(|_| ())?;

        self.bot
            .edit_message_reply_markup(chat_id, message_id)
            .reply_markup(keyboard)
            .await
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub(crate) async fn feeding_schedule(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        let keyboard = Self::back_to_menu_keyboard();

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn health_alerts(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        let keyboard = Self::back_to_menu_keyboard();

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn maintenance(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        let keyboard = Self::back_to_menu_keyboard();

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn colonies(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        let keyboard = Self::back_to_menu_keyboard();

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn record_molt_command(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        tarantula_id: i64,
        size: f32,
        user_id: u64,
    ) -> BotResult<()> {
        self.db
            .record_molt(tarantula_id, size, None, None, user_id)
            .await?;

        let keyboard = Self::back_to_menu_keyboard();
        self.reply_with_send(
            chat_id,
            "Molt recorded \nThank you!".to_string(),
            Some(keyboard),
        )
        .await
    }

    pub(crate) async fn health_check_command(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        tarantula_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let tarantula = self.db.get_tarantula_by_id(user_id, tarantula_id).await?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(
                "✅ Healthy",
                BotCallback::HealthStatus(tarantula_id, 1).to_string(),
            )],
            vec![InlineKeyboardButton::callback(
                "⚠️ Monitor",
                BotCallback::HealthStatus(tarantula_id, 2).to_string(),
            )],
            vec![InlineKeyboardButton::callback(
                "🚨 Critical",
                BotCallback::HealthStatus(tarantula_id, 3).to_string(),
            )],
            vec![InlineKeyboardButton::callback(
                "« Cancel",
                MainMenu.to_string(),
            )],
        ]);

        self.replay_with_edit(
            chat_id,
            message_id,
            format!(
                "Health check for *{}*\nSelect current health status:",
                tarantula.name
            ),
            keyboard,
        )
        .await
    }

    pub(crate) async fn health_status_command(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        tarantula_id: i64,
        health_status: HealthStatus,
        user_id: u64,
    ) -> BotResult<()> {
        self.db
            .record_health_check(user_id, tarantula_id, health_status, None)
            .await?;
        let keyboard = Self::back_to_menu_keyboard();

        self.replay_with_edit(
            chat_id,
            message_id,
            "Health status recorded \nThank you!".to_string(),
            keyboard,
        )
        .await
    }
    pub(crate) async fn colony_maintenance_menu(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        colony_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let colony = self.colony_status(colony_id, user_id).await?;
        self.colony_maintenance_command(chat_id, message_id, &colony.colony_name, user_id)
            .await
    }

    pub(crate) async fn colony_maintenance_command(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        colony_name: &str,
        user_id: u64,
    ) -> BotResult<()> {
        let colonies = self.db.get_colony_status(user_id).await?;
        let colony = colonies
            .iter()
            .find(|c| c.colony_name.eq_ignore_ascii_case(colony_name))
            .ok_or_else(|| BotError::NotFound(format!("Colony '{}' not found", colony_name)))?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(
                "📝 Update Count",
                ColonyGetCount(colony.id).to_string(),
            )],
            vec![InlineKeyboardButton::callback(
                "« Cancel",
                MainMenu.to_string(),
            )],
        ]);

        self.replay_with_edit(
            chat_id,
            message_id,
            format!(
                "*Cricket Colony Maintenance*\n\nColony: {}\nCurrent count: {}\nSize: {}\n\nSelect maintenance action:",
                colony.colony_name, colony.current_count, colony.size_type.to_db_name()
            ), keyboard)
            .await
    }
    pub(crate) async fn status_overview(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn record_feeding_menu(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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
                            FeedTarantula(t.id).to_string(),
                        )
                    })
                    .collect()
            })
            .collect();
        let keyboard = Self::with_back_button(keyboard);

        let msg = "*Record Feeding*\n\nSelect a tarantula:";
        self.replay_with_edit(
            chat_id,
            message_id,
            msg.to_string(),
            InlineKeyboardMarkup::new(keyboard),
        )
        .await
    }

    pub(crate) async fn record_health_check_menu(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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
                            BotCallback::HealthCheck(t.id).to_string(),
                        )
                    })
                    .collect()
            })
            .collect();
        let keyboard = Self::with_back_button(keyboard);

        self.replay_with_edit(
            chat_id,
            message_id,
            "*Health Check*\n\nSelect a tarantula:".to_string(),
            InlineKeyboardMarkup::new(keyboard),
        )
        .await
    }

    pub(crate) async fn molt_history(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        user_id: u64,
    ) -> BotResult<()> {
        self.view_molt_records(chat_id, message_id, user_id).await?;
        Ok(())
    }

    pub(crate) async fn view_records(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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
                MainMenu.to_string(),
            )],
        ]);

        let msg = "*View Records*\n\nSelect record type:";
        self.replay_with_edit(chat_id, message_id, msg.to_string(), keyboard)
            .await
    }
    pub(crate) async fn view_feeding_records(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn view_health_records(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn view_molt_records(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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

        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    pub(crate) async fn record_molt_menu(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
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
                            //todo add actual size
                            MoltSimple(t.id).to_string(),
                        )
                    })
                    .collect()
            })
            .collect();
        keyboard.push(vec![InlineKeyboardButton::callback(
            "« Back to Menu",
            MainMenu.to_string(),
        )]);

        let msg = "*Record Molt*\n\nSelect a tarantula:";
        self.replay_with_edit(
            chat_id,
            message_id,
            msg.to_string(),
            InlineKeyboardMarkup::new(keyboard),
        )
        .await
        .map(|_| ())
        .map_err(|e| e.into())
    }

    pub(crate) async fn colony_maintenance(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        user_id: u64,
    ) -> BotResult<()> {
        let colonies = self.db.get_colony_status(user_id).await?;
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = colonies
            .chunks(2)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| {
                        InlineKeyboardButton::callback(
                            format!("{} ({})", c.colony_name, c.size_type.to_db_name()),
                            ColonyMaintenanceMenu(c.id).to_string(),
                        )
                    })
                    .collect()
            })
            .collect();
        keyboard.push(vec![InlineKeyboardButton::callback(
            "« Back to Menu",
            MainMenu.to_string(),
        )]);

        self.replay_with_edit(
            chat_id,
            message_id,
            "*Colony Maintenance*\n\nSelect a colony:".to_string(),
            InlineKeyboardMarkup::new(keyboard),
        )
        .await
        .map(|_| ())
        .map_err(|e| e.into())
    }

    pub(crate) async fn colony_count(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        colony_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        let colony = self.colony_status(colony_id, user_id).await?;

        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback(
                    "-10",
                    ColonyCountUpdate(colony_id, -10).to_string(),
                ),
                InlineKeyboardButton::callback("-5", ColonyCountUpdate(colony_id, -5).to_string()),
            ],
            vec![
                InlineKeyboardButton::callback("+1", ColonyCountUpdate(colony_id, 1).to_string()),
                InlineKeyboardButton::callback("+5", ColonyCountUpdate(colony_id, 5).to_string()),
                InlineKeyboardButton::callback("+10", ColonyCountUpdate(colony_id, 10).to_string()),
                InlineKeyboardButton::callback("+50", ColonyCountUpdate(colony_id, 50).to_string()),
            ],
            vec![InlineKeyboardButton::callback(
                "« Cancel",
                MainMenu.to_string(),
            )],
        ]);

        self.replay_with_edit(
            chat_id,
            message_id,
            format!(
                "*Update Colony Count*\n\nColony: {}\nCurrent count: {}\nSelect adjustment:",
                colony.colony_name, colony.current_count
            ),
            keyboard,
        )
        .await
    }

    pub(crate) async fn colony_count_update(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        colony_id: i64,
        adjustment: i32,
        user_id: u64,
    ) -> BotResult<()> {
        self.db
            .update_colony_count(colony_id, adjustment, user_id)
            .await?;

        let keyboard = Self::back_to_menu_keyboard();

        self.replay_with_edit(
            chat_id,
            message_id,
            format!("✅ Colony count updated by {}", adjustment),
            keyboard,
        )
        .await
    }

    pub(crate) async fn view_feeding_schedule(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        tarantula_id: i64,
        user_id: u64,
    ) -> BotResult<()> {
        
        let tarantula = self.db.get_tarantula_by_id(user_id, tarantula_id).await?;

        
        let current_size = self.db.get_current_size(tarantula_id).await?;
        let schedule = self
            .db
            .get_feeding_schedule(tarantula.species_id, current_size)
            .await?
            .unwrap();

        let frequency = self
            .db
            .get_feeding_frequency(schedule.frequency_id.unwrap_or(1))
            .await?
            .unwrap();

        let message = format!(
            "*Feeding Schedule for {}*\n\n\
            🦗 *Current Stage:* {}\n\
            📏 *Size:* {:.1} cm\n\
            🍽 *Prey Size:* {}\n\
            ⏱ *Feeding Frequency:* {}\n\
            🦗 *Prey Type:* {}\n\n\
            ℹ️ {}\n\n\
            _Feeding window: Every {} to {} days_",
            tarantula.name,
            schedule.size_category,
            current_size,
            schedule.prey_size,
            schedule.feeding_frequency,
            schedule.prey_type,
            schedule.notes.unwrap_or_default(),
            frequency.min_days,
            frequency.max_days
        );

        let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back",
            ListTarantulas.to_string(),
        )]]);
        self.replay_with_edit(chat_id, message_id, message, keyboard)
            .await
    }

    async fn reply_with_send(
        &self,
        chat_id: ChatId,
        message: String,
        keyboard_markup: Option<InlineKeyboardMarkup>,
    ) -> BotResult<()> {
        let mut request = self
            .bot
            .send_message(chat_id, message)
            .parse_mode(ParseMode::Html);

        if let Some(k) = keyboard_markup {
            request = request.reply_markup(k)
        }

        request.await.map(|_| ()).map_err(|e| e.into())
    }

    fn back_to_menu_keyboard() -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
            "« Back to Menu",
            MainMenu.to_string(),
        )]])
    }

    fn with_back_button(
        mut keyboard: Vec<Vec<InlineKeyboardButton>>,
    ) -> Vec<Vec<InlineKeyboardButton>> {
        keyboard.push(vec![InlineKeyboardButton::callback(
            "« Back to Menu",
            MainMenu.to_string(),
        )]);
        keyboard
    }

    pub(crate) async fn molt_simple_callback(
        &self,
        chat_id: ChatId,
        tarantula_id: i64,
    ) -> BotResult<()> {
        self.bot
            .send_message(chat_id, "Please enter the molt size in centimeters:")
            .await?;

        self.dialogue
            .clone()
            .update_dialogue(chat_id, DialogueState::RecordMolt { tarantula_id })
            .await?;
        Ok(())
    }
}
