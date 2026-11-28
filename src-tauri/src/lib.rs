pub mod collector;
pub mod models;

use collector::CollectorHub;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};

#[tauri::command]
fn open_dashboard_window(app: tauri::AppHandle) {
    if let Some(dashboard_win) = app.get_webview_window("dashboard") {
        let _ = dashboard_win.show();
        let _ = dashboard_win.set_focus();
    } else {
        let _ = tauri::WebviewWindowBuilder::new(
            &app,
            "dashboard",
            tauri::WebviewUrl::App("/dashboard".into()),
        )
        .title("MoniQA — System Monitor")
        .inner_size(1040.0, 680.0)
        .min_inner_size(760.0, 520.0)
        .decorations(true)
        .resizable(true)
        .build();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_dashboard_window])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let handle = app.handle().clone();

            // 1. Build Menu for Tray Icon (Right-click menu)
            let open_dash = MenuItem::with_id(app, "open_dash", "Open Dashboard", true, None::<&str>)?;
            let separator = MenuItem::with_id(app, "sep", "---", false, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit MoniQA", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&open_dash, &separator, &quit_item])?;

            // 2. Build Tray Icon
            let icon = app.default_window_icon().cloned().unwrap();
            let tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .title("MoniQA")
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "open_dash" => {
                        open_dashboard_window(app.clone());
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            let tray_ref = Arc::new(Mutex::new(tray));

            // 3. Auto-hide Popover Window on Focus Loss
            if let Some(main_window) = app.get_webview_window("main") {
                let win_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let WindowEvent::Focused(false) = event {
                        let _ = win_clone.hide();
                    }
                });
            }

            // 4. Hide dashboard window instead of destroying on close
            if let Some(dashboard_win) = app.get_webview_window("dashboard") {
                let dash_clone = dashboard_win.clone();
                dashboard_win.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = dash_clone.hide();
                    }
                });
            }

            // 4. Background Hardware Telemetry Polling Loop (1 second)
            let emit_handle = handle.clone();
            let tray_for_telemetry = tray_ref.clone();

            std::thread::spawn(move || {
                let mut hub = CollectorHub::new();

                loop {
                    let snapshot = hub.collect_snapshot();

                    // Emit event to all windows
                    let _ = emit_handle.emit("system-stats", &snapshot);

                    // Update menubar tray title with CPU% and Temp
                    let temp_display = if snapshot.thermal.cpu_temp_c > 0.0 {
                        format!(" {:.0}°C", snapshot.thermal.cpu_temp_c)
                    } else {
                        "".to_string()
                    };

                    let tray_title = format!("{:.0}%{}", snapshot.cpu.global_usage, temp_display);

                    if let Ok(tray_lock) = tray_for_telemetry.lock() {
                        let _ = tray_lock.set_title(Some(&tray_title));
                    }

                    std::thread::sleep(Duration::from_millis(3000));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_hub_snapshot() {
        let mut hub = CollectorHub::new();
        let snapshot = hub.collect_snapshot();

        assert!(snapshot.cpu.core_count > 0, "CPU core count should be > 0");
        assert!(snapshot.memory.total_bytes > 0, "Memory total should be > 0");
        assert!(!snapshot.system_info.os_name.is_empty(), "OS name should not be empty");
        println!("[Test Verified] MoniQA snapshot collected: CPU {} cores, RAM {} GB, Chip: {}", 
            snapshot.cpu.core_count,
            snapshot.memory.total_bytes / (1024 * 1024 * 1024),
            snapshot.system_info.chip_name
        );
    }
}

