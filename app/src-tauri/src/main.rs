#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

use std::time;
use std::{process, thread};

use crate::tray::create_tray;
use tauri::api::path::{app_dir, desktop_dir, resolve_path};
use tauri::utils::Error;
use tauri::{command, plugin, AppHandle, EventLoopMessage, Manager, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

use tauri::SystemTray;
use tauri_runtime_wry::{Plugin, PluginBuilder};
use webview2_com::Microsoft::Web::WebView2::Win32::{
  COREWEBVIEW2_PERMISSION_KIND_CLIPBOARD_READ, COREWEBVIEW2_PERMISSION_KIND_GEOLOCATION,
  COREWEBVIEW2_PERMISSION_KIND_UNKNOWN_PERMISSION, COREWEBVIEW2_PERMISSION_STATE_ALLOW,
};
use webview2_com::PermissionRequestedEventHandler;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, WPARAM};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
use windows::Win32::System::WinRT::EventRegistrationToken;
use windows::Win32::UI::Shell::{DesktopWallpaper, IDesktopWallpaper};
use windows::Win32::UI::WindowsAndMessaging::*;

struct DeskbtmWindowManager {
  target: HWND,
}

#[derive(Debug, Clone)]
pub struct PrepareWindowsPlatform {
  shell_window: HWND,
  def_view: HWND,
  folder_view: HWND,
}

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

fn set_sys_wallpaper() -> ::windows::core::Result<()> {
  let wallpaper: IDesktopWallpaper;
  unsafe {
    wallpaper = CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;
  };

  Ok(())
}

#[command]
fn exec_planet(app_handle: AppHandle) {
  let mut desktop = desktop_dir().unwrap();

  desktop.push("875477924/Planet.exe");

  if let Some(path) = desktop.to_str() {
    let child = process::Command::new(path).spawn().unwrap();
    dbg!(child.id());
    let ten_millis = time::Duration::from_secs(2);
    let now = time::Instant::now();
    thread::sleep(ten_millis);

    let (target_main_window, handles) = get_all_window_from_pid(child.id());

    dbg!(&target_main_window, &handles);

    unsafe {
      let progman_window: HWND = FindWindowW("Progman\0", PCWSTR::default());

      if let Some(target_main_window) = target_main_window {
        thread::sleep(time::Duration::from_secs(3));
        dbg!(IsWindowVisible(target_main_window).as_bool());
        remove_window_edge(target_main_window);

        maximize(target_main_window);

        split_window_workw(progman_window);
        enum_window();

        set_deskbtm(target_main_window);
      }
    }
    // for win in &handles {
    //   remove_window_edge(*win);
    //   dbg!(win, "==========");
    // }
  }
}

fn is_main_window(handle: HWND) -> bool {
  unsafe { GetWindow(handle, GW_OWNER) == HWND(0) }
}

fn maximize(window: HWND) {
  let flags = SWP_NOZORDER | SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED;

  unsafe {
    SetWindowPos(
      window,
      HWND::default(),
      0,
      0,
      GetSystemMetrics(SM_CXSCREEN),
      GetSystemMetrics(SM_CXSCREEN),
      flags,
    );
    SendMessageW(
      window,
      WM_SYSCOMMAND,
      WPARAM(SC_MAXIMIZE as usize),
      LPARAM(0),
    );
  }
}

fn get_main_window_from_pid() {
  // GetWindow(handle, GW_OWNER) == HWND(0) && IsWindowVisible(handle);
}

fn get_all_window_from_pid(pid: u32) -> (Option<HWND>, Vec<HWND>) {
  let mut handles: Vec<HWND> = Vec::new();
  let mut main_window: Option<HWND> = None;
  let mut window = HWND(0);
  let mut t_pid: u32 = 0;

  unsafe {
    loop {
      window = FindWindowExW(HWND(0), window, PCWSTR::default(), PCWSTR::default());

      GetWindowThreadProcessId(window, &mut t_pid);

      if t_pid == pid {
        handles.push(window);

        if is_main_window(window) {
          main_window = Some(window);
        }
      }

      if window == HWND(0) {
        break;
      }
    }

    (main_window, handles)
  }
}

#[command]
fn my_custom_command(app_handle: AppHandle) -> isize {
  // app_handle.clipboard_manager()

  // app_handle.

  let main_window = app_handle.get_window("main").unwrap();

  unsafe {
    let progman_window: HWND = FindWindowW("Progman\0", PCWSTR::default());
    let main_window_hwnd = main_window.hwnd().unwrap();
    split_window_workw(progman_window);
    enum_window();

    set_deskbtm(HWND(main_window_hwnd.0));

    dbg!(deepest_point, shell_window, sys_list_window);
  }

  main_window.hwnd().unwrap().0
}

#[command]
fn plugin_case(app_handle: AppHandle) -> String {
  String::from("==========")
}

struct RequestDefender {}

static mut ddd: i32 = 10;

struct Demo;

// impl PluginBuilder<EventLoopMessage> for Demo {
// type Plugin = Plugin<EventLoopMessage>;
// fn build(self, context: tauri_runtime_wry::Context<EventLoopMessage>) -> Self::Plugin {
//   context.
// }
// }

fn remove_window_edge(handle: HWND) {
  unsafe {
    dbg!(handle);
    let win = GetWindowLongW(handle, GWL_STYLE) as u32;
    let (mut style, mut ex_style) = (WINDOW_STYLE(win), WINDOW_EX_STYLE(win));

    style &= !(WS_CAPTION | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU);

    // ex_style &= !(WS_EX_DLGMODALFRAME | WS_EX_CLIENTEDGE | WS_EX_STATICEDGE | WS_EX_ACCEPTFILES);

    SetWindowLongW(handle, GWL_STYLE, style.0 as i32);
    // SetWindowLongW(handle, GWL_EXSTYLE, ex_style.0 as i32);
  }
}

fn main() {
  let tray = create_tray();

  tauri::Builder::default()
    .system_tray(tray)
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      let handle = app.handle();

      // app.wry_plugin();

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
