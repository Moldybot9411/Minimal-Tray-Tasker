use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_positioner::{ Position, WindowExt };

use crate::{services::settings_service::get_settings, Settings};

pub async fn init(app: AppHandle) {
    #[cfg(desktop)]
    {
        let open_i = MenuItem::with_id(&app, "open", "Open", true, None::<&str>).unwrap();
        let quit_i = MenuItem::with_id(&app, "quit", "Quit", true, None::<&str>).unwrap();
        let menu = Menu::with_id_and_items(&app, "menu", &[&open_i, &quit_i]).unwrap();

        let _ = TrayIconBuilder::new()
            .tooltip("Minimal Tracker App")
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu)
            .show_menu_on_left_click(false)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "open" => {
                    if let Some(window) = app.get_webview_window("main") {
                        open_existing_window(window);
                    } else {
                        create_window(&app);
                    }
                }

                "quit" => {
                    app.exit(0);
                }

                _ => {
                    println!("Menu item {:?} not handled", event.id);
                }
            })
            .on_tray_icon_event(|tray, event| match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    let app = tray.app_handle();
                    tauri_plugin_positioner::on_tray_event(&app, &event);

                    if let Some(window) = app.get_webview_window("main") {
                        open_existing_window(window);
                    } else {
                        create_window(&app);
                    }
                }

                _ => {}
            })
            .build(&app);
    }
}

fn open_existing_window(window: WebviewWindow) {
    let _ = window.unminimize();
    let _ = window.show();

    move_window(&window);

    let _ = window.set_focus();
}

fn create_window(app: &AppHandle) {
    let window = WebviewWindow::builder(
        app,
        "main",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("Minimal Tracker")
    .decorations(false)
    .inner_size(400.0, 500.0)
    .visible(true)
    .always_on_top(true)
    .build()
    .unwrap();

    move_window(&window);

    let _ = window.set_focus();

    window
        .clone()
        .on_window_event(move |event| match event {
            tauri::WindowEvent::Focused(focus) if !focus => {
                let ramsaver = &get_settings(window.state())[&Settings::RamSaver.to_string()];

                if ramsaver.is_boolean() && ramsaver == true {
                    let _ = window.close();
                } else {
                    let _ = window.hide();
                }
            }
            _ => {}
        });
}

fn move_window(window: &WebviewWindow) {
    #[cfg(target_os = "linux")]
    let _ = window.move_window_constrained(Position::Center);

    #[cfg(not(target_os = "linux"))]
    let _ = window.move_window_constrained(Position::TrayCenter);
}
