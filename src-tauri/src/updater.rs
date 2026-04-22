use crate::config::{get, set};
use log::{info, warn, error};
use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

pub fn check_update(app_handle: tauri::AppHandle) {
    let enable = match get("check_update") {
        Some(v) => v.as_bool().unwrap(),
        None => {
            set("check_update", true);
            true
        }
    };
    
    if enable {
        info!("Update check enabled, checking for updates...");
        
        tauri::async_runtime::spawn(async move {
            match app_handle.updater() {
                Ok(updater) => {
                    match updater.check().await {
                        Ok(Some(update)) => {
                            info!("Update available: {}", update.version);
                            
                            // Download and install the update
                            // download() requires two callbacks: progress and download complete
                            match update.download_and_install(|_, _| {}, || {}).await {
                                Ok(_) => {
                                    info!("Update downloaded and installed successfully");
                                }
                                Err(e) => {
                                    error!("Failed to download/install update: {}", e);
                                }
                            }
                        }
                        Ok(None) => {
                            info!("No updates available, app is up to date");
                        }
                        Err(e) => {
                            error!("Failed to check for updates: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Updater not available: {}", e);
                }
            }
        });
    } else {
        info!("Update check disabled");
    }
}
