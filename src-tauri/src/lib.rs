mod migration_manager;
mod services {
    pub mod autostart_service;
    pub mod notification_service;
    pub mod refresh_dailies_service;
    pub mod settings_service;
    pub mod tray_service;
}
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use services::{
    autostart_service, notification_service, refresh_dailies_service, settings_service,
    tray_service,
};
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};
use tauri::{Listener, Manager};

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
enum Settings {
    Autostart,
    Reminders,
    RamSaver,
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Settings::Autostart => write!(f, "autostart"),
            Settings::Reminders => write!(f, "reminders"),
            Settings::RamSaver => write!(f, "ramsaver"),
        }
    }
}

struct AppData {
    user_settings: HashMap<Settings, serde_json::Value>,
}

impl Default for AppData {
    fn default() -> Self {
        let mut settings: HashMap<Settings, serde_json::Value> = HashMap::new();
        settings.insert(Settings::Autostart, json!(true));
        settings.insert(Settings::Reminders, json!(true));
        settings.insert(Settings::RamSaver, json!(false));

        Self {
            user_settings: settings,
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:appdb.sqlite", migration_manager::get_migrations())
                .build(),
        )
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            app.manage(Mutex::new(AppData::default()));

            let handle = app.handle();

            // Start System Tray
            tauri::async_runtime::spawn(tray_service::init(handle.clone()));

            // Start User Settings Service
            settings_service::init(handle.clone(), handle.state());

            // Start the hourly Notification Service
            tauri::async_runtime::spawn(notification_service::init(handle.clone()));

            // Start Daily Task service
            tauri::async_runtime::spawn(refresh_dailies_service::init(handle.clone()));

            #[cfg(desktop)]
            {
                let handle = Arc::new(app.handle().clone());

                //Start Autostart Service and attach to settings changes
                autostart_service::init(handle.clone());

                app.listen_any("settings_changed", move |_event| {
                    let handle = handle.clone();
                    tauri::async_runtime::spawn(
                        async move { autostart_service::init(handle.clone()) },
                    );
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            services::settings_service::get_settings,
            services::settings_service::set_autostart,
            services::settings_service::set_reminders,
            services::settings_service::set_ram_saver
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, code, .. } => {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
            _ => {}
        });
}
