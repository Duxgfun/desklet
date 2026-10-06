mod sys;
use tauri::{Manager, Window};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::sync::Mutex;
use sysinfo::System;

struct AppState {
    sys: Mutex<System>,
}

#[tauri::command]
fn pin_to_desktop(window: Window) {
    if let Ok(hwnd) = window.hwnd() {
        // hwnd is an `isize` or `*mut c_void` depending on the platform, we cast it
        #[cfg(target_os = "windows")]
        sys::attach_to_desktop(hwnd as *mut std::ffi::c_void);
    }
}

#[tauri::command]
fn get_sys_stats(state: tauri::State<AppState>) -> (f32, f32) {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    
    let cpu = sys.global_cpu_info().cpu_usage();
    let mem_total = sys.total_memory() as f32;
    let mem_used = sys.used_memory() as f32;
    let ram = if mem_total > 0.0 { (mem_used / mem_total) * 100.0 } else { 0.0 };
    
    (cpu, ram)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(System::new_all()),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, pin_to_desktop, get_sys_stats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
