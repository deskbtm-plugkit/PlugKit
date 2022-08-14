#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{command, AppHandle, Manager};
use windows::core::PCWSTR;
use windows::w;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

fn split_worker(hwnd: HWND) {
    unsafe {
        SendMessageW(hwnd, 0x052c, WPARAM(0), LPARAM(0));
    }
}

fn enum_window() {
    unsafe {
        EnumWindows(Some(enum_window_proc), LPARAM(0));
    }
}

extern "system" fn enum_window_proc(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let shell_window = FindWindowExW(window, HWND(0), w!("SHELLDLL_DefView\0"), PCWSTR::null());
        let sys_list_window =
            FindWindowExW(shell_window, HWND(0), w!("SysListView32\0"), PCWSTR::null());

        let shell_window = FindWindowExW(HWND(0), window, w!("WorkerW\0"), PCWSTR::null());

        BOOL(1)
    }
}

#[command]
fn my_custom_command(app_handle: AppHandle) -> isize {
    let main_window = app_handle.get_window("main").unwrap();

    unsafe {
        // CreateWindowExA();
        let progman_window: HWND = FindWindowExW(HWND(0), HWND(0), w!("Progman\0"), PCWSTR::null());
        // let progman_window: HWND = FindWindowW(w!("Progman\0"), PCWSTR::null());
        println!("{:?}", progman_window);
    }

    main_window.hwnd().unwrap().0
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
