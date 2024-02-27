#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod google_gemini;

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager, WindowEvent};
use crate::google_gemini::generate_content;

#[tauri::command]
async fn get_ai_response(prompt: String) -> Result<String, String> {
    generate_content(prompt).await.map_err(|e| e.to_string())
}

fn main() {
    let context = tauri::generate_context!();

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("open", "Open"))
        .add_item(CustomMenuItem::new("exit", "Exit"));

    let system_tray = SystemTray::new().with_menu(tray_menu).with_tooltip("My Assistant");

    tauri::Builder::default()
        .system_tray(system_tray)
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| match event {
                    WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        window_clone.hide().unwrap();
                    }
                    _ => {}
                });
            }
            Ok(())
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if let Some(window) = app.get_window("main") {
                    match id.as_str() {
                        "open" => {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        },
                        "exit" => {
                            std::process::exit(0);
                        },
                        _ => {}
                    }
                }
            },
            SystemTrayEvent::LeftClick { .. } => {
                if let Some(window) = app.get_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_ai_response])
        .run(context)
        .expect("error while running tauri application");
}
