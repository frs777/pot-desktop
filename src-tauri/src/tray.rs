use tauri::{
    menu::{Menu, MenuItem, MenuEvent, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconId},
    AppHandle, Manager, Emitter,
};
use log::info;

const TRAY_ID: &str = "main_tray";

// Forward to clipboard module
fn toggle_clipboard_monitor(app: &AppHandle) {
    crate::clipboard::toggle_clipboard_monitor(app.clone());
}

pub fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "show" => {
            info!("Show window requested from tray");
            if let Some(window) = app.get_webview_window("translate") {
                let _ = window.show();
                let _ = window.set_focus();
            } else if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            } else {
                info!("No window found, creating new translate window");
                crate::window::text_translate("".to_string());
            }
        }
        "settings" => {
            info!("Opening settings from tray");
            crate::window::config_window();
        }
        "clipboard_toggle" => {
            info!("Toggle clipboard monitor from tray");
            toggle_clipboard_monitor(app);
        }
        "quit" => {
            info!("Quit requested from tray");
            // Close all windows first
            for window in app.webview_windows().values() {
                let _ = window.close();
            }
            // Then exit the application
            std::process::exit(0);
        }
        _ => {
            info!("Unknown menu event: {}", event.id.as_ref());
        }
    }
}

/// Build and register the system tray menu
pub fn build_tray(app: &AppHandle, language: &str, copy_mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Building tray menu (language={}, copy_mode={})", language, copy_mode);

    // Tray menu items
    let show_item = MenuItem::with_id(
        app,
        "show",
        "Pokaż okno",
        true,
        None::<&str>,
    )?;

    let settings_item = MenuItem::with_id(
        app,
        "settings",
        "Ustawienia",
        true,
        None::<&str>,
    )?;

    // Dynamic label based on copy_mode
    let clipboard_label = if copy_mode == "true" || copy_mode == "clipboard" {
        "Schowek: WŁĄCZONY"
    } else {
        "Schowek: WYŁĄCZONY"
    };
    let clipboard_item = MenuItem::with_id(
        app,
        "clipboard_toggle",
        clipboard_label,
        true,
        None::<&str>,
    )?;

    let separator = PredefinedMenuItem::separator(app)?;

    let quit_item = MenuItem::with_id(
        app,
        "quit",
        "Wyjdź",
        true,
        None::<&str>,
    )?;

    // Build the menu
    let tray_menu = Menu::with_items(
        app,
        &[&show_item, &settings_item, &clipboard_item, &separator, &quit_item],
    )?;

    // Check if tray already exists and update menu instead of creating new one
    let tray = app.tray_by_id(TRAY_ID);
    match tray {
        Some(existing_tray) => {
            info!("Updating existing tray menu");
            existing_tray.set_menu(Some(tray_menu))?;
        }
        None => {
            info!("Creating new tray icon");
            let _tray = TrayIconBuilder::with_id(TrayIconId::new(TRAY_ID))
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .tooltip("Pot App - Translation & OCR")
                .show_menu_on_left_click(true)
                .on_tray_icon_event(|tray, event| {
                    use tauri::tray::TrayIconEvent;
                    if let TrayIconEvent::Click { .. } = event {
                        // Show main window on tray icon click
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("translate") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        } else if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
        }
    }

    info!("Tray menu ready");
    Ok(())
}

#[tauri::command]
pub fn update_tray(app_handle: AppHandle, language: String, copy_mode: String) {
    info!("Update tray called (language={}, copy_mode={})", language, copy_mode);
    // Update tray menu with new values (doesn't create new tray)
    if let Err(e) = build_tray(&app_handle, &language, &copy_mode) {
        log::error!("Failed to update tray: {}", e);
    }
}
