use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_store::StoreExt;
use serde_json::{json, Value};
use std::sync::Mutex;

use crate::{AppData, Settings};

pub fn init(app: AppHandle, state: State<'_, Mutex<AppData>>) {
    let store = app.app_handle().store("usersettings.json").unwrap();
    let state = state.lock().unwrap();

    for (field_name, field_value) in &state.user_settings {
        match store.get(field_name.to_string()) {
            Some(val) => println!("{}", val),
            None => {
                store.set(field_name.to_string(), json!(field_value));
            },
        }
    }
}

#[tauri::command]
pub fn get_settings(state: State<'_, Mutex<AppData>>) -> serde_json::Map<String, Value> {
    let state = state.lock().unwrap();

    let mut map = serde_json::Map::new();

    for (field_name, field_value) in &state.user_settings {
        map.insert(field_name.to_string(), field_value.clone());
    }

    map
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, state: State<'_, Mutex<AppData>>, value: bool) {
    let store = app.app_handle().store("usersettings.json").unwrap();
    let mut state = state.lock().unwrap();

    state.user_settings.insert(Settings::Autostart, json!(value));

    store.set(Settings::Autostart.to_string(), value);
    app.emit("settings_changed", json!(null)).unwrap();
}

#[tauri::command]
pub fn set_reminders(app: AppHandle, state: State<'_, Mutex<AppData>>, value: bool) {
    let store = app.app_handle().store("usersettings.json").unwrap();
    let mut state = state.lock().unwrap();

    state.user_settings.insert(Settings::Reminders, json!(value));

    store.set(Settings::Reminders.to_string(), value);
    app.emit("settings_changed", json!(null)).unwrap();
}