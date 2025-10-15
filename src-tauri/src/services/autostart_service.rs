use std::sync::Arc;
use tauri::{AppHandle, Manager};

use crate::{services::settings_service::get_settings, Settings};

pub fn init(app: Arc<AppHandle>) {
    use tauri_plugin_autostart::MacosLauncher;
    use tauri_plugin_autostart::ManagerExt;

    let _ = app.app_handle().plugin(tauri_plugin_autostart::init(
        MacosLauncher::LaunchAgent,
        Some(vec![]),
    ));

    println!("Updating autostart");
    let enabled = &get_settings(app.state())[&Settings::Autostart.to_string()];

    let autostart_manager = app.autolaunch();
    let _ = if enabled == true {
        autostart_manager.enable()
    } else {
        autostart_manager.disable()
    };
}
