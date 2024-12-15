use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{self, Duration};
use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::ParseMode;
use tokio::sync::RwLock;
use crate::db::db::TarantulaDB;

#[derive(Clone)]
pub struct NotificationSystem {
    bot: Bot,
    db: Arc<TarantulaDB>,
    user_chats: Arc<RwLock<HashMap<u64, ChatId>>>,
}

impl NotificationSystem {
    pub fn new(bot: Bot, db: Arc<TarantulaDB>) -> Self {
        Self {
            bot,
            db,
            user_chats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start(self) {
        log::debug!("Starting notification system");
        let feeding_task = self.clone();
        let health_task = self.clone();
        let colony_task = self.clone();

        tokio::spawn(async move { feeding_task.run_feeding_checks().await });
        tokio::spawn(async move { health_task.run_health_checks().await });
        tokio::spawn(async move { colony_task.run_colony_checks().await });
    }

    pub async fn register_chat(&self, user_id: u64, chat_id: ChatId) {
        let mut user_chats = self.user_chats.write().await;
        user_chats.insert(user_id, chat_id);
        log::debug!("Registered chat_id: {} for user_id: {}", chat_id, user_id);
    }

    async fn run_feeding_checks(self) {
        log::debug!("Starting feeding checks");
        let mut interval = time::interval(Duration::from_secs(86400));
        let mut message = String::with_capacity(1024);

        loop {
            interval.tick().await;
            let user_chats = self.user_chats.read().await;

            for (&user_id, &chat_id) in user_chats.iter() {
                if let Ok(feedings) = self.db.get_tarantulas_due_feeding(user_id).await {
                    if !feedings.is_empty() {
                        message.clear();
                        message.push_str("🍽 *Feeding Due*\n\n");

                        for t in &feedings {
                            use std::fmt::Write;
                            let _ = writeln!(message, "• {} - {} days since last feeding",
                                             t.name,
                                             t.days_since_feeding.unwrap_or(0.0)
                            );
                        }

                        let _ = self.bot.send_message(chat_id, &message)
                            .parse_mode(ParseMode::Html)
                            .await;
                    }
                }
            }
        }
    }

    async fn run_health_checks(self) {
        log::debug!("Starting health checks");
        let mut interval = time::interval(Duration::from_secs(3600));
        let mut message = String::with_capacity(1024);

        loop {
            interval.tick().await;
            let user_chats = self.user_chats.read().await;

            for (&user_id, &chat_id) in user_chats.iter() {
                if let Ok(alerts) = self.db.get_health_alerts(user_id).await {
                    let critical = alerts.iter()
                        .filter(|a| a.alert_type == "Critical");

                    message.clear();
                    let mut has_alerts = false;
                    message.push_str("🚨 *Critical Health Alerts*\n\n");

                    for alert in critical {
                        has_alerts = true;
                        use std::fmt::Write;
                        let _ = writeln!(message, "• {} - {}", alert.name, alert.alert_type);
                    }

                    if has_alerts {
                        let _ = self.bot.send_message(chat_id, &message)
                            .parse_mode(ParseMode::Html)
                            .await;
                    }
                }
            }
        }
    }

    async fn run_colony_checks(self) {
        log::debug!("Starting colony checks");
        let mut interval = time::interval(Duration::from_secs(86400));
        let mut message = String::with_capacity(1024);

        loop {
            interval.tick().await;
            let user_chats = self.user_chats.read().await;

            for (&user_id, &chat_id) in user_chats.iter() {
                if let Ok(colonies) = self.db.get_colony_status(user_id).await {
                    let low_colonies = colonies.iter()
                        .filter(|c| c.weeks_remaining.unwrap_or(0.0) < 2.0);

                    message.clear();
                    let mut has_alerts = false;
                    message.push_str("🦗 *Low Cricket Colony Alert*\n\n");

                    for colony in low_colonies {
                        has_alerts = true;
                        use std::fmt::Write;
                        let _ = writeln!(message, "• {} - {:.1} weeks remaining",
                                         colony.colony_name,
                                         colony.weeks_remaining.unwrap_or(0.0)
                        );
                    }

                    if has_alerts {
                        let _ = self.bot.send_message(chat_id, &message)
                            .parse_mode(ParseMode::Html)
                            .await;
                    }
                }
            }
        }
    }
}