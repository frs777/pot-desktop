use crate::config::{get, set};
use crate::window::text_translate;
use log::info;
use serde_json;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;

pub struct ClipboardMonitorEnableWrapper(pub Mutex<String>);

pub fn start_clipboard_monitor(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut pre_text = "".to_string();
        loop {
            let handle = app_handle.app_handle();
            let state = handle.state::<ClipboardMonitorEnableWrapper>();
            if let Ok(clipboard_monitor) = state.0.try_lock() {
                if clipboard_monitor.contains("true") {
                    if let Ok(result) = app_handle.clipboard().read_text() {
                        if !result.is_empty() && result != pre_text {
                            text_translate(result.clone());
                            pre_text = result;
                        }
                    }
                } else {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });
}

#[tauri::command]
pub fn toggle_clipboard_monitor(app: tauri::AppHandle) {
    let state = app.state::<ClipboardMonitorEnableWrapper>();
    let (was_enabled, new_value) = {
        if let Ok(mut monitor) = state.0.try_lock() {
            let current = monitor.contains("true");
            let new_value = if current { "false" } else { "true" }.to_string();
            *monitor = new_value.clone();
            (current, new_value)
        } else {
            return;
        }
    };

    // If enabling clipboard monitor, restart the monitoring loop
    if !was_enabled {
        info!("Clipboard monitor enabled, starting monitoring loop");
        start_clipboard_monitor(app.clone());
    } else {
        info!("Clipboard monitor disabled");
    }

    // Also persist to config
    set("clipboard_monitor", serde_json::Value::Bool(!was_enabled));
    // Rebuild tray to update the label
    let language = get("app_language").unwrap_or(serde_json::Value::String("pl".into())).as_str().unwrap_or("pl").to_string();
    let _ = crate::tray::update_tray(app.clone(), language, new_value);
}
