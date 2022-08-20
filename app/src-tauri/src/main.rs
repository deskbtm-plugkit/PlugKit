#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::mem;

use tauri::{command, AppHandle, Manager};
use windows::core::PCWSTR;
use windows::w;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

fn split_window_workw(hwnd: HWND) {
    unsafe {
        let a = SendMessageW(hwnd, 0x052c, WPARAM(0), LPARAM(0));
        dbg!(a);
    }
}

fn enum_window() {
    unsafe {
        EnumWindows(Some(enum_window_proc), LPARAM(0)).unwrap();
    }
}

fn set_deskbtm(target: HWND) {
    unsafe {
        SetParent(target, deepest_point);
    }
}

static mut shell_window: HWND = HWND(0);
static mut sys_list_window: HWND = HWND(0);
// This name is from the made in abyss see the deepest point (奈落之底)
static mut deepest_point: HWND = HWND(0);

extern "system" fn enum_window_proc(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let tmp_shell_window =
            FindWindowExW(window, HWND(0), w!("SHELLDLL_DefView\0"), PCWSTR::null());
        let tmp_sys_list_window = FindWindowExW(
            tmp_shell_window,
            HWND(0),
            w!("SysListView32\0"),
            PCWSTR::null(),
        );

        if HWND::default() != tmp_shell_window {
            let tmp_deepest_point = FindWindowExW(HWND(0), window, w!("WorkerW\0"), PCWSTR::null());
            shell_window = tmp_shell_window;
            sys_list_window = tmp_sys_list_window;
            if tmp_deepest_point != HWND::default() {
                deepest_point = tmp_deepest_point;
            }
        }

        BOOL(1)
    }
}

extern "system" fn mouse_proc(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let tmp_shell_window =
            FindWindowExW(window, HWND(0), w!("SHELLDLL_DefView\0"), PCWSTR::null());
        let tmp_sys_list_window = FindWindowExW(
            tmp_shell_window,
            HWND(0),
            w!("SysListView32\0"),
            PCWSTR::null(),
        );

        if HWND::default() != tmp_shell_window {
            let tmp_deepest_point = FindWindowExW(HWND(0), window, w!("WorkerW\0"), PCWSTR::null());
            shell_window = tmp_shell_window;
            sys_list_window = tmp_sys_list_window;
            if tmp_deepest_point != HWND::default() {
                deepest_point = tmp_deepest_point;
            }
        }

        BOOL(1)
    }
}

#[command]
fn my_custom_command(app_handle: AppHandle) -> isize {
    let main_window = app_handle.get_window("main").unwrap();

    unsafe {
        // CreateWindowExA();
        // let progman_window: HWND = FindWindowExW(HWND(0), HWND(0), w!("Progman\0"), PCWSTR::null());
        let progman_window: HWND = FindWindowW(w!("Progman\0"), PCWSTR::null());
        split_window_workw(progman_window);

        enum_window();

        let main_window_int = main_window.hwnd().unwrap();

        set_deskbtm(HWND(main_window_int.0));

        dbg!(deepest_point, shell_window, sys_list_window);

        println!("{:?}", progman_window);
    }

    main_window.hwnd().unwrap().0
}

#[command]
fn plugin_case(app_handle: AppHandle) -> String {
    String::from("==========")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command, plugin_case])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
