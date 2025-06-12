// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{Position, WindowExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .on_window_event(on_window_event)
        .system_tray(system_tray())
        .on_system_tray_event(on_system_tray_event)
        .enable_macos_default_menu(false)
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn on_window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::Focused(focus) => {
            if !focus {
                event.window().hide().unwrap();
            }
        }
        tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}

fn system_tray() -> SystemTray {
    SystemTray::new()
        .with_menu(system_tray_menu())
}

fn system_tray_menu() -> SystemTrayMenu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    SystemTrayMenu::new()
        .add_item(quit)
}

fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            if !window.is_visible().unwrap() {
                window.show().unwrap();
                window.set_focus().unwrap();
                window.move_window(Position::TrayCenter).unwrap();
            } else {
                window.hide().unwrap();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
