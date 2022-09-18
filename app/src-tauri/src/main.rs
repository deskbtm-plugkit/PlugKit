#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

use crate::tray::create_tray;
use abyss_addon_log::fern::colors::{Color, ColoredLevelConfig};
use abyss_addon_log::{LogTarget, LoggerBuilder, RotationStrategy};
use abyss_core::windows::prepared_deskbtm;

use tauri::{command, AppHandle, Manager, SystemTrayEvent};

use tauri_runtime_wry::PluginBuilder;
use webview2_com::Microsoft::Web::WebView2::Win32::*;
use webview2_com::PermissionRequestedEventHandler;

use windows::Win32::System::WinRT::EventRegistrationToken;

#[command]
fn cmd1() {
  let _instance = prepared_deskbtm();
}

struct RequestDefender {}

#[command]
async fn create_demo_window(app: AppHandle) {
  let _window = tauri::WindowBuilder::new(
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

      let _handle = app.handle();

      app.listen_global("invoke-demo", |_event| {});

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
        let _item_handle = app.tray_handle().get_item(&id);
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
