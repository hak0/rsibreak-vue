// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppSettings {
    work_duration: u32,
    break_duration: u32,
    start_time: String,
    end_time: String,
    active_days: Vec<u8>,
}

struct AppState {
    settings: Mutex<AppSettings>,
}

#[tauri::command]
async fn save_settings(state: State<'_, AppState>, settings: AppSettings) -> Result<(), String> {
    // 验证时间格式
    let start = chrono::NaiveTime::parse_from_str(&settings.start_time, "%H:%M")
        .map_err(|_| "无效的开始时间格式")?;
    let end = chrono::NaiveTime::parse_from_str(&settings.end_time, "%H:%M")
        .map_err(|_| "无效的结束时间格式")?;

    if start > end {
        return Err("结束时间不能早于开始时间".into());
    }

    if settings.active_days.is_empty() {
        return Err("请至少选择一个生效日期".into());
    }

    *state.settings.lock().unwrap() = settings;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            settings: Mutex::new(AppSettings {
                work_duration: 45,
                break_duration: 5,
                start_time: "09:00".into(),
                end_time: "18:00".into(),
                active_days: vec![1, 2, 3, 4, 5],
            }),
        })
        .invoke_handler(tauri::generate_handler![save_settings])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用失败");
}
