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
    chat_ids: Arc<RwLock<Vec<ChatId>>>,
}

impl NotificationSystem {
    pub fn new(bot: Bot, db: Arc<TarantulaDB>) -> Self {
        Self {
            bot,
            db,
            chat_ids: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub async fn register_chat(&self, chat_id: ChatId) {
        let mut chat_ids = self.chat_ids.write().await;
        if !chat_ids.contains(&chat_id) {
            log::debug!("Registering chat_id: {}", chat_id);
            chat_ids.push(chat_id);
        }
    }

    pub async fn start(self) {
        log::debug!("Starting notification system");
        let feeding_task = self.clone();
        let health_task = self.clone();
        let colony_task = self.clone();
        let molt_task = self.clone();

        tokio::spawn(async move { feeding_task.run_feeding_checks().await });
        tokio::spawn(async move { health_task.run_health_checks().await });
        tokio::spawn(async move { colony_task.run_colony_checks().await });
        tokio::spawn(async move { molt_task.run_molt_checks().await });
    }

    async fn run_feeding_checks(self) {
        log::debug!("Starting feeding checks");
        let mut interval = time::interval(Duration::from_secs(86400));
        loop {
            interval.tick().await;
            if let Ok(due_feedings) = self.db.get_tarantulas_due_feeding().await {
                let chat_ids = self.chat_ids.read().await;
                for chat_id in chat_ids.iter() {
                    if !due_feedings.is_empty() {
                        let mut message = "🍽 *Feeding Due*\n\n".to_string();
                        for t in &due_feedings {
                            message.push_str(&format!(
                                "• {} - {} days since last feeding\n",
                                t.name,
                                t.days_since_feeding.unwrap_or(0.0)
                            ));
                        }
                        let _ = self.bot.send_message(*chat_id, message)
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
        loop {
            interval.tick().await;
            if let Ok(alerts) = self.db.get_health_alerts().await {
                let chat_ids = self.chat_ids.read().await;
                for chat_id in chat_ids.iter() {
                    let critical = alerts.iter()
                        .filter(|a| a.alert_type == "Critical")
                        .collect::<Vec<_>>();

                    if !critical.is_empty() {
                        let mut message = "🚨 *Critical Health Alerts*\n\n".to_string();
                        for alert in critical {
                            message.push_str(&format!(
                                "• {} - {}\n",
                                alert.name,
                                alert.alert_type
                            ));
                        }
                        let _ = self.bot.send_message(*chat_id, message)
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
        loop {
            interval.tick().await;
            if let Ok(colonies) = self.db.get_colony_status().await {
                let chat_ids = self.chat_ids.read().await;
                for chat_id in chat_ids.iter() {
                    let low_colonies = colonies.iter()
                        .filter(|c| c.weeks_remaining.unwrap_or(0.0) < 2.0)
                        .collect::<Vec<_>>();

                    if !low_colonies.is_empty() {
                        let mut message = "🦗 *Low Cricket Colony Alert*\n\n".to_string();
                        for colony in low_colonies {
                            message.push_str(&format!(
                                "• {} - {:.1} weeks remaining\n",
                                colony.colony_name,
                                colony.weeks_remaining.unwrap_or(0.0)
                            ));
                        }
                        let _ = self.bot.send_message(*chat_id, message)
                            .parse_mode(ParseMode::Html)
                            .await;
                    }
                }
            }
        }
    }

    async fn run_molt_checks(self) {
        log::debug!("Starting molt checks");
        let mut interval = time::interval(Duration::from_secs(86400));
        loop {
            interval.tick().await;
            if let Ok(alerts) = self.db.get_health_alerts().await {
                let chat_ids = self.chat_ids.read().await;
                for chat_id in chat_ids.iter() {
                    let molt_alerts = alerts.iter()
                        .filter(|a| a.alert_type == "Extended Pre-molt")
                        .collect::<Vec<_>>();

                    if !molt_alerts.is_empty() {
                        let mut message = "🐾 *Molt Status Alert*\n\n".to_string();
                        for alert in molt_alerts {
                            message.push_str(&format!(
                                "• {} - in pre-molt for {} days\n",
                                alert.name,
                                alert.days_in_state
                            ));
                        }
                        let _ = self.bot.send_message(*chat_id, message)
                            .parse_mode(ParseMode::Html)
                            .await;
                    }
                }
            }
        }
    }
}