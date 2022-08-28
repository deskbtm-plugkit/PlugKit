#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process;

use tauri::api::path::desktop_dir;
use tauri::{command, plugin, AppHandle, Manager, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

use tauri::SystemTray;
use webview2_com::Microsoft::Web::WebView2::Win32::{
  COREWEBVIEW2_PERMISSION_KIND_CLIPBOARD_READ, COREWEBVIEW2_PERMISSION_KIND_GEOLOCATION,
  COREWEBVIEW2_PERMISSION_KIND_UNKNOWN_PERMISSION, COREWEBVIEW2_PERMISSION_STATE_ALLOW,
};
use webview2_com::PermissionRequestedEventHandler;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::System::WinRT::EventRegistrationToken;
use windows::Win32::UI::WindowsAndMessaging::*;

struct DeskbtmWindowManager {}

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

// This name is from the made in abyss, see the deepest point (奈落之底)
static mut deepest_point: HWND = HWND(0);

extern "system" fn enum_window_proc(window: HWND, _: LPARAM) -> BOOL {
  unsafe {
    let tmp_shell_window = FindWindowExW(window, HWND(0), "SHELLDLL_DefView\0", PCWSTR::default());
    let tmp_sys_list_window = FindWindowExW(
      tmp_shell_window,
      HWND(0),
      "SysListView32\0",
      PCWSTR::default(),
    );

    if HWND::default() != tmp_shell_window {
      let tmp_deepest_point = FindWindowExW(HWND(0), window, "WorkerW\0", PCWSTR::default());
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
    let tmp_shell_window = FindWindowExW(window, HWND(0), "SHELLDLL_DefView\0", PCWSTR::default());
    let tmp_sys_list_window = FindWindowExW(
      tmp_shell_window,
      HWND(0),
      "SysListView32\0",
      PCWSTR::default(),
    );

    if HWND::default() != tmp_shell_window {
      let tmp_deepest_point = FindWindowExW(HWND(0), window, "WorkerW\0", PCWSTR::default());
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
fn exec_planet(app_handle: AppHandle) {
  let mut desktop = desktop_dir().unwrap();

  desktop.push("875477924/Planet.exe");

  if let Some(path) = desktop.to_str() {
    let child = process::Command::new(path).spawn().unwrap();
    dbg!(child.id());
  }
}

#[command]
fn my_custom_command(app_handle: AppHandle) -> isize {
  // app_handle.clipboard_manager()

  // app_handle.

  let main_window = app_handle.get_window("main").unwrap();

  unsafe {
    // CreateWindowExA();
    // let progman_window: HWND = FindWindowExW(HWND(0), HWND(0), w!("Progman\0"), PCWSTR::null());
    let progman_window: HWND = FindWindowW("Progman\0", PCWSTR::default());
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

struct RequestDefender {}

static mut ddd: i32 = 10;

fn main() {
  let show = CustomMenuItem::new("show".to_string(), "Show");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(show);

  let tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .system_tray(tray)
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      #[allow(unused_must_use)]
      {
        main_window.with_webview(|webview| unsafe {
          let webview2 = webview.controller().CoreWebView2().unwrap();
          let mut token = EventRegistrationToken::default();

          webview2.add_PermissionRequested(
            PermissionRequestedEventHandler::create(Box::new(|_, args| {
              if let Some(args) = args {
                let mut kind = COREWEBVIEW2_PERMISSION_KIND_UNKNOWN_PERMISSION;
                args.PermissionKind(&mut kind)?;
                dbg!(kind);
                if kind == COREWEBVIEW2_PERMISSION_KIND_GEOLOCATION {
                  args.SetState(COREWEBVIEW2_PERMISSION_STATE_ALLOW)?;
                }
              }
              Ok(())
            })),
            &mut token,
          );
        });
      }

      dbg!("Hello World");

      // main_window.config();
      // WindowBuilder::new(app, "core", WindowUrl::App("index.html".into()))
      //   .on_web_resource_request(|request, response| {});
      Ok(())
    })
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        // get a handle to the clicked menu item
        // note that `tray_handle` can be called anywhere,
        // just get a `AppHandle` instance with `app.handle()` on the setup hook
        // and move it to another function or thread
        let item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "show" => {
            dbg!("=====================");
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
            // item_handle.set_title("Show").unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      my_custom_command,
      plugin_case,
      exec_planet
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
