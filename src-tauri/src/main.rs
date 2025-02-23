use chrono::{Datelike, Local, NaiveTime};
use serde::{Deserialize, Serialize};
use tauri_plugin_notification::NotificationExt;
use std::sync::Mutex;
use tauri::{Manager, State};
use std::time::Duration as StdDuration;


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
    let start = NaiveTime::parse_from_str(&settings.start_time, "%H:%M")
        .map_err(|_| "无效的开始时间格式")?;
    let end =
        NaiveTime::parse_from_str(&settings.end_time, "%H:%M").map_err(|_| "无效的结束时间格式")?;

    if start > end {
        return Err("结束时间不能早于开始时间".into());
    }

    if settings.active_days.is_empty() {
        return Err("请至少选择一个生效日期".into());
    }

    *state.settings.lock().unwrap() = settings;
    Ok(())
}

async fn check_time_and_notify(app_handle: tauri::AppHandle) {
    println!("进入check_time_and_notify");
    loop {
        println!("进入循环");
        // 获取应用设置
        let app_settings = {
            let state = app_handle.state::<AppState>();
            let settings = state.settings.lock().unwrap();
            settings.clone()
        };

        // 解析开始和结束时间
        let (start_time, end_time) = match (
            NaiveTime::parse_from_str(&app_settings.start_time, "%H:%M"),
            NaiveTime::parse_from_str(&app_settings.end_time, "%H:%M"),
        ) {
            (Ok(s), Ok(e)) => (s, e),
            _ => {
                tokio::time::sleep(StdDuration::from_secs(60)).await;
                continue;
            }
        };

        let now = Local::now().naive_local();
        let current_time = now.time();
        let current_weekday = now.weekday().number_from_monday() as u8;

        // 检查生效日
        println!("Active days: {:?}", app_settings.active_days);
        println!("Current_Weekday: {}", current_weekday);
        if !app_settings.active_days.contains(&current_weekday) {
            tokio::time::sleep(StdDuration::from_secs(60)).await;
            continue;
        }

        // 检查时间窗口
        if !is_time_in_window(current_time, start_time, end_time) {
            tokio::time::sleep(StdDuration::from_secs(60)).await;
            continue;
        }

        // 计算基准时间点
        let base_time = calculate_base_time(now, start_time, end_time);
        let delta_minutes = (now - base_time).num_minutes() as u32;

        // 计算周期参数
        let total_cycle = app_settings.work_duration + app_settings.break_duration;
        if total_cycle == 0 {
            tokio::time::sleep(StdDuration::from_secs(60)).await;
            continue;
        }

        // 判断通知类型
        match delta_minutes % total_cycle {
            0 => send_notification(&app_handle, "该工作了！"),
            x if x == app_settings.work_duration => send_notification(&app_handle, "该休息了！"),
            _ => (),
        }

        tokio::time::sleep(StdDuration::from_secs(60)).await;
    }
}

// 时间窗口判断函数
fn is_time_in_window(current: NaiveTime, start: NaiveTime, end: NaiveTime) -> bool {
    if start <= end {
        current >= start && current <= end
    } else {
        current >= start || current <= end
    }
}

// 计算基准时间函数
fn calculate_base_time(now: chrono::NaiveDateTime, start: NaiveTime, end: NaiveTime) -> chrono::NaiveDateTime {
    if start <= end {
        now.date().and_time(start)
    } else {
        if now.time() >= start {
            now.date().and_time(start)
        } else {
            (now.date() - chrono::TimeDelta::days(1)).and_time(start)
        }
    }
}

// 发送通知函数
fn send_notification(app_handle: &tauri::AppHandle, message: &str) {
    app_handle.notification()
        .builder()
        .title("时间到！")
        .body(message)
        .show()
        .unwrap();
}
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(1)
    .enable_time()
    .build()
    .unwrap();
    // let runtime = tokio::runtime::Runtime::new().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
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
        .setup(move |app| {
            println!("开始setup");
            let app_handle = app.handle().clone();
            println!("开始spawn");
            runtime.spawn(check_time_and_notify(app_handle));
            println!("结束spawn");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用失败");
}
