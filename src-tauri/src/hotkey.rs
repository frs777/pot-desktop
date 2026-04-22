use crate::config::{get, set};
use crate::window::{input_translate, ocr_recognize, ocr_translate, selection_translate};
use crate::APP;
use log::{info, warn};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, Modifiers, Code, ShortcutState};
use std::str::FromStr;

/// Parse a shortcut string like "Ctrl+Shift+T" into a Shortcut object
pub fn parse_shortcut_str(input: &str) -> Result<Shortcut, String> {
    let parts: Vec<&str> = input.split('+').map(|s| s.trim()).collect();
    if parts.is_empty() {
        return Err("Empty shortcut".into());
    }

    let mut modifiers = Modifiers::empty();
    let (modifier_parts, key_parts) = parts.split_at(parts.len() - 1);

    for part in modifier_parts {
        match part.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "shift" => modifiers |= Modifiers::SHIFT,
            "alt" | "option" => modifiers |= Modifiers::ALT,
            "meta" | "super" | "cmd" | "command" | "win" => modifiers |= Modifiers::META,
            "cmdorcontrol" | "cmd_or_control" | "cmdorctrl" => {
                // Cross-platform: both CONTROL and META
                modifiers |= Modifiers::CONTROL;
                modifiers |= Modifiers::META;
            }
            _ => return Err(format!("Unknown modifier: `{}`", part)),
        }
    }

    let key_str = key_parts.first().ok_or("No target key")?;
    
    let code = Code::from_str(key_str)
        .map_err(|_| format!("Unknown key: `{}`", key_str))?;

    Ok(Shortcut::new(Some(modifiers), code))
}

fn register_shortcut_for(app_handle: &AppHandle, name: &str, handler: impl Fn() + Send + Sync + 'static, key: &str) -> Result<(), String> {
    let hotkey = {
        if key.is_empty() {
            match get(name) {
                Some(v) => v.as_str().unwrap().to_string(),
                None => {
                    set(name, "");
                    String::new()
                }
            }
        } else {
            key.to_string()
        }
    };

    if !hotkey.is_empty() {
        info!("Registering global shortcut: {} for {}", hotkey, name);
        
        match parse_shortcut_str(&hotkey) {
            Ok(shortcut) => {
                let handler_arc = std::sync::Arc::new(handler);
                let name_clone = name.to_string();
                
                // Register with the global shortcut plugin
                let result = app_handle.global_shortcut().on_shortcut(shortcut, move |_app, _sc, event| {
                    if matches!(event.state(), ShortcutState::Pressed) {
                        info!("Shortcut triggered: {}", name_clone);
                        let h = std::sync::Arc::clone(&handler_arc);
                        h();
                    }
                });

                if let Err(e) = result {
                    warn!("Failed to register shortcut {}: {}", name, e);
                    return Err(format!("Failed to register shortcut: {}", e));
                }
            }
            Err(e) => {
                warn!("Failed to parse shortcut {}: {}", hotkey, e);
                return Err(format!("Failed to parse shortcut: {}", e));
            }
        }
    }
    Ok(())
}

pub fn register_shortcut(shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match shortcut {
        "hotkey_selection_translate" => register_shortcut_for(
            app_handle,
            "hotkey_selection_translate",
            selection_translate,
            "",
        )?,
        "hotkey_input_translate" => {
            register_shortcut_for(app_handle, "hotkey_input_translate", input_translate, "")?
        }
        "hotkey_ocr_recognize" => register_shortcut_for(app_handle, "hotkey_ocr_recognize", ocr_recognize, "")?,
        "hotkey_ocr_translate" => register_shortcut_for(app_handle, "hotkey_ocr_translate", ocr_translate, "")?,
        "all" => {
            register_shortcut_for(
                app_handle,
                "hotkey_selection_translate",
                selection_translate,
                "",
            )?;
            register_shortcut_for(app_handle, "hotkey_input_translate", input_translate, "")?;
            register_shortcut_for(app_handle, "hotkey_ocr_recognize", ocr_recognize, "")?;
            register_shortcut_for(app_handle, "hotkey_ocr_translate", ocr_translate, "")?;
        }
        _ => {}
    }
    Ok(())
}

#[tauri::command]
pub fn register_shortcut_by_frontend(name: &str, shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match name {
        "hotkey_selection_translate" => register_shortcut_for(
            app_handle,
            "hotkey_selection_translate",
            selection_translate,
            shortcut,
        )?,
        "hotkey_input_translate" => register_shortcut_for(
            app_handle,
            "hotkey_input_translate",
            input_translate,
            shortcut,
        )?,
        "hotkey_ocr_recognize" => {
            register_shortcut_for(app_handle, "hotkey_ocr_recognize", ocr_recognize, shortcut)?
        }
        "hotkey_ocr_translate" => {
            register_shortcut_for(app_handle, "hotkey_ocr_translate", ocr_translate, shortcut)?
        }
        _ => {}
    }
    Ok(())
}
