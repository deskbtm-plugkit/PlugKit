#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

use std::thread;

use crate::tray::create_tray;
use abyss_addon_log::fern::colors::{Color, ColoredLevelConfig};
use abyss_addon_log::{LogTarget, LoggerBuilder, RotationStrategy};

use abyss_core::webview2_com::Microsoft::Web::WebView2::Win32::*;
use abyss_core::webview2_com::PermissionRequestedEventHandler;
use abyss_core::windows::prepared_deskbtm;
use tauri::{command, AppHandle, Manager, SystemTrayEvent};

use tauri_runtime_wry::PluginBuilder;

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::WinRT::EventRegistrationToken;
use windows::Win32::UI::Shell::SetWindowSubclass;
use windows::Win32::UI::WindowsAndMessaging::{
  FindWindowW, GetDesktopWindow, SendMessageW, SetParent,
};
use windows::{
  core::PCWSTR,
  w,
  Win32::{
    Foundation::{BOOL, HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::*,
  },
};

#[command]
fn cmd1() {
  let instance = prepared_deskbtm();
  dbg!(instance);
}

/// This window will be at the bottom of deskbtm (index=0)
struct LauncherWindow {}

#[command]
fn cmd2() {
  unsafe {
    let deskbtm = prepared_deskbtm();
    let a = GetDesktopWindow();
    let b = FindWindowW(w!("Progman\0"), PCWSTR::null());
    dbg!(deskbtm);
    dbg!("===========");
    // let r = SendMessageW(deskbtm.view, WM_CLOSE, WPARAM(0), LPARAM(0));
    // CloseWindow(deskbtm.view);
    DestroyWindow(deskbtm.view);
    // UpdateWindow(b);;
    // UpdateWindow(a);
    // dbg!(r);
  }
  // let handle = thread::spawn(move || {
  //   dbg!("======================================");

  //   // let event_loop: EventLoop<()> = EventLoopExtWindows::new_any_thread();
  //   // let window = NativeWindowBuilder::new()
  //   //   .with_title("Fuck")
  //   //   .build(&event_loop)
  //   //   .unwrap();

  //   // let webview = WebViewBuilder::new(window)
  //   //   .unwrap()
  //   //   .with_url("https://baidu.com")
  //   //   .unwrap();

  //   // webview.build().unwrap();

  //   // event_loop.run(move |event, _, control_flow| {
  //   //   dbg!(event);
  //   // });
  // });

  // handle.join().unwrap();
}

#[command]
fn cmd3(app: AppHandle) {
  let main_window = app.get_window("main").unwrap();
  let deskbtm = prepared_deskbtm();
  dbg!(deskbtm);
  unsafe {
    SetParent(main_window.hwnd().ok(), deskbtm.view);
  }
}

#[command]
fn cmd4(app: AppHandle) {
  let main_window = app.get_window("main").unwrap();
  let deskbtm = prepared_deskbtm();
  unsafe {
    SetParent(deskbtm.view, main_window.hwnd().ok());
  }
}

unsafe extern "system" fn public_window_callback(
  window: HWND,
  msg: u32,
  wparam: WPARAM,
  lparam: LPARAM,
  uidsubclass: usize,
  subclass_input_ptr: usize,
) -> LRESULT {
  LRESULT(1)
}

#[command]
fn cmd5(app: AppHandle) {
  let main_window = app.get_window("main").unwrap();
  let deskbtm = prepared_deskbtm();
  unsafe {
    SetWindowSubclass(main_window.hwnd().ok(), Some(public_window_callback), 0, 0);
  }
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
  // let event_loop = EventLoop::new();

  // event_loop.run(move |event, _, control_flow| match event {
  //   Event::DeviceEvent { event, .. } => {
  //     dbg!(event);
  //   }
  //   _ => (),
  // });

  let builder = tauri::Builder::default()
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
      //   .on_web_resource_request(|request, response| {});
      Ok(())
    })
    // .on_window_event(|event| match event.event() {
    //   WindowEvent::Focused(focused) => {
    //     if !focused {
    //       event.window().hide().unwrap();
    //     }
    //   }
    //   _ => {}
    // })
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        let _item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "show" => {
            let window = app.get_window("main").unwrap();

            window.show().unwrap();
            // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
            // item_handle.set_title("Show").unwrap();
          }
          "take_out" => {
            let main_window = app.get_window("main").unwrap();
            let deskbtm = prepared_deskbtm();
            unsafe {
              SetParent(main_window.hwnd().ok(), GetDesktopWindow());
            }
          }
          _ => {}
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      create_demo_window,
      cmd1,
      cmd2,
      cmd3,
      cmd4
    ])
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  builder.run(move |_app_handle, _e| {})
}
