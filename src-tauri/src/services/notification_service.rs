use chrono::Timelike;
use std::time::Duration;
use tauri_plugin_notification::NotificationExt;
use tokio::time::sleep;
use tauri::{AppHandle, Manager};

use crate::Settings;

use super::settings_service;

pub async fn init(app: AppHandle) {
    tokio::spawn(async move {
        loop {
            let now = chrono::Local::now();
            let next_hour = (now + chrono::Duration::hours(1))
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap();

            let dur: Duration = (next_hour - now).to_std().unwrap();
            sleep(dur).await;

            let enabled = &settings_service::get_settings(app.state())[&Settings::Reminders.to_string()];
            if enabled.is_boolean() && enabled == false { continue };

            app.notification()
                .builder()
                .title("Reminder")
                .body("You still have tasks to do!")
                .show()
                .unwrap();
        }
    });
}
