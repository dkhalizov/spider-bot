use crate::bot::bot::TarantulaBot;
use crate::bot::callbacks::BotCallback;
use crate::bot::callbacks::BotCallback::{Colonies, ColonyMaintenance, FeedingSchedule, HealthAlerts, ListTarantulas, MainMenu, Maintenance, MoltHistory, RecordFeeding, RecordHealthCheck, RecordMolt, StatusOverview, ViewRecords};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::models::cricket::ColonyStatus;

pub(crate) fn welcome_keyboard()-> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("🕷 List Tarantulas", ListTarantulas.to_string()),
            InlineKeyboardButton::callback("📊 Status Overview", StatusOverview.to_string()),
        ],
        vec![
            InlineKeyboardButton::callback("🍽 Due Feedings", FeedingSchedule.to_string()),
            InlineKeyboardButton::callback("📝 Record Feeding", RecordFeeding.to_string()),
        ],
        vec![
            InlineKeyboardButton::callback("🏥 Health Alerts", HealthAlerts.to_string()),
            InlineKeyboardButton::callback(
                "🔍 Record Health Check",
                RecordHealthCheck.to_string(),
            ),
        ],
        vec![
            InlineKeyboardButton::callback("🐾 Recent Molts", MoltHistory.to_string()),
            InlineKeyboardButton::callback("📝 Record Molt", RecordMolt.to_string()),
        ],
        vec![
            InlineKeyboardButton::callback("🦗 Colony Status", Colonies.to_string()),
            InlineKeyboardButton::callback(
                "🧰 Colony Maintenance",
                ColonyMaintenance.to_string(),
            ),
        ],
        vec![
            InlineKeyboardButton::callback("🧹 Maintenance Tasks", Maintenance.to_string()),
            InlineKeyboardButton::callback("📋 View Records", ViewRecords.to_string()),
        ],
    ])
}

pub(crate) fn back_to_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        "« Back to Menu",
        MainMenu.to_string(),
    )]])
}

pub(crate) fn feed_count_selection_keyboard(
    tarantula_id: i64,
    colony_id: i64,
) -> InlineKeyboardMarkup {
     InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(
                "1 cricket",
                BotCallback::FeedConfirm(tarantula_id, colony_id, 1).to_string(),
            ),
            InlineKeyboardButton::callback(
                "2 crickets",
                BotCallback::FeedConfirm(tarantula_id, colony_id, 2).to_string(),
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                "3 crickets",
                BotCallback::FeedConfirm(tarantula_id, colony_id, 3).to_string(),
            ),
            InlineKeyboardButton::callback(
                "5 crickets",
                BotCallback::FeedConfirm(tarantula_id, colony_id, 5).to_string(),
            ),
        ],
        vec![InlineKeyboardButton::callback(
            "« Cancel",
            MainMenu.to_string(),
        )],
    ])
}

pub(crate) fn feed_command_keyboard(tarantula_id: i64, colonies: Vec<ColonyStatus>) -> Vec<Vec<InlineKeyboardButton>> {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = colonies
        .chunks(2)
        .map(|chunk| {
            chunk
                .iter()
                .map(|colony| {
                    InlineKeyboardButton::callback(
                        format!("{} ({})", colony.colony_name, colony.size_type.to_db_name()),
                        BotCallback::FeedSelectColony(tarantula_id, colony.id).to_string(),
                    )
                })
                .collect()
        })
        .collect();

    keyboard.push(vec![InlineKeyboardButton::callback(
        "« Cancel",
        MainMenu.to_string(),
    )]);
    keyboard
}
