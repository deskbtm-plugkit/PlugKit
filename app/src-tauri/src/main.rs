#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

use std::time;
use std::{process, thread};

use crate::tray::create_tray;
use abyss_addon_log::fern::colors::{Color, ColoredLevelConfig};
use abyss_addon_log::{LogTarget, LoggerBuilder, RotationStrategy};
use abyss_core::windows::{prepared_deskbtm, PreparedDeskbtm};
use log::info;
use tauri::api::path::{app_dir, desktop_dir, resolve_path};
use tauri::utils::Error;
use tauri::{
  command, plugin, AppHandle, EventLoopMessage, Manager, SystemTrayEvent, WindowBuilder,
};

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

#[command]
fn cmd1() {
  let instance = prepared_deskbtm();

  // Log::info(&"demo");
  // Log::error(&1);

  // let prepared_deskbtm = PreparedDeskbtm::new();
}

struct RequestDefender {}

#[command]
async fn create_demo_window(app: AppHandle) {
  let window = tauri::WindowBuilder::new(
    &app,
    "label",
    tauri::WindowUrl::App("app/src/setting/index.html".into()),
  )
  .build()
  .unwrap();
}

fn main() {
  // prepared_deskbtm

  let tray = create_tray();
  let colors = ColoredLevelConfig::new()
    .info(Color::Green)
    .debug(Color::Magenta);
  let mut logger_option = LoggerBuilder::default()
    .targets([LogTarget::LogDir, LogTarget::Stdout])
    .with_colors(colors)
    .rotation_strategy(RotationStrategy::KeepAll);

  if cfg!(debug_assertions) {
    logger_option = logger_option.with_colors(colors);
  }

  let logger_plugin = logger_option.build();

  tauri::Builder::default()
    .system_tray(tray)
    .plugin(logger_plugin)
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      // main_window.

      let handle = app.handle();

      // main_window.

      // app.wry_plugin();

      #[allow(unused_must_use)]
      {
        main_window.with_webview(|webview| unsafe {
          let webview = webview.controller().CoreWebView2().unwrap();
          let mut token = EventRegistrationToken::default();

          webview.add_PermissionRequested(
            &PermissionRequestedEventHandler::create(Box::new(|_, args| {
              if let Some(args) = args {
                let mut kind = COREWEBVIEW2_PERMISSION_KIND_UNKNOWN_PERMISSION;
                args.PermissionKind(&mut kind)?;
                dbg!(kind);
                if kind == COREWEBVIEW2_PERMISSION_KIND_CLIPBOARD_READ {
                  args.SetState(COREWEBVIEW2_PERMISSION_STATE_ALLOW)?;
                }
              }
              Ok(())
            })),
            &mut token,
          );
        });
      }

      // main_window.config();
      // WindowBuilder::new(app, "core", WindowUrl::App("index.html".into()))
      //   .on_web_resource_request(|request, response| {});
      Ok(())
    })
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
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
    .invoke_handler(tauri::generate_handler![create_demo_window, cmd1])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
