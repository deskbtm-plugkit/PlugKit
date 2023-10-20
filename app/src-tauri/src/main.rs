#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

use crate::tray::create_tray;
use plugkit_addon_log::fern::colors::{Color, ColoredLevelConfig};
use plugkit_addon_log::{Builder as LoggerBuilder, LogTarget, RotationStrategy};

use plugkit_core::webview2_com::Microsoft::Web::WebView2::Win32::*;
use plugkit_core::webview2_com::PermissionRequestedEventHandler;
use plugkit_core::windows::deskbtm;
use tauri::{command, AppHandle, Manager, SystemTrayEvent};

use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
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
  let instance = deskbtm().lock().unwrap();
  dbg!(instance);
}

/// This window will be at the bottom of deskbtm (index=0)
struct LauncherWindow {}

#[command]
fn cmd2() {
  unsafe {
    let deskbtm = &deskbtm().lock().unwrap();
    let a = GetDesktopWindow();
    let b = FindWindowW(w!("Progman\0"), PCWSTR::null());
    dbg!(deskbtm);

    deskbtm.clear();
    dbg!(deskbtm);

    // let deskbtm1 = deskbtm().lock().unwrap();

    // dbg!(deskbtm1);
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
fn cmd6(app: AppHandle) {
  let main_window = app.get_window("label").unwrap();
  let deskbtm = deskbtm().lock().unwrap();
  unsafe {
    SetParent(main_window.hwnd().ok(), deskbtm.view);
  }
}

#[command]
fn cmd3(app: AppHandle) {
  let main_window = app.get_window("main").unwrap();
  let deskbtm = deskbtm().lock().unwrap();
  unsafe {
    SetParent(main_window.hwnd().ok(), deskbtm.view);
  }
}

#[command]
fn cmd4(app: AppHandle) {
  let main_window = app.get_window("main").unwrap();
  let deskbtm = deskbtm().lock().unwrap();
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
  let deskbtm = deskbtm().lock().unwrap();
  unsafe {
    SetWindowSubclass(main_window.hwnd().ok(), Some(public_window_callback), 0, 0);
  }
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
  // disable webview2 elastic overscroll.
  #[cfg(target_os = "windows")]
  std::env::set_var(
    "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
    "--disable-features=ElasticOverscroll",
  );
  let tray = create_tray();
  let mut logger_options = LoggerBuilder::default()
    .targets([LogTarget::LogDir, LogTarget::Stdout])
    .rotation_strategy(RotationStrategy::KeepAll);

  if cfg!(debug_assertions) {
    let colors = ColoredLevelConfig::new()
      .info(Color::Green)
      .debug(Color::Magenta);
    logger_options = logger_options.with_colors(colors);
  }

  let logger_plugin = logger_options.build();
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

      main_window.with_webview(|webview| unsafe {
        let webview2: ICoreWebView2 = webview.controller().CoreWebView2().unwrap();
        let mut token = EventRegistrationToken::default();

        webview2.add_PermissionRequested(
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
            let deskbtm = deskbtm().lock().unwrap();
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
      cmd4,
      cmd6
    ])
    .build(tauri::generate_context!())
    .expect("error while running tauri application");
  // unsafe {
  //   let hook = SetWindowsHookExW(
  //     WH_MOUSE_LL,
  //     Some(enum_window_proc),
  //     GetModuleHandleW(PCWSTR::null()).ok(),
  //     0,
  //   );

  //   unsafe extern "system" fn enum_window_proc(
  //     code: i32,
  //     wparam: WPARAM,
  //     lparam: LPARAM,
  //   ) -> LRESULT {
  //     println!("{} {:?} {:?}", code, wparam, lparam);

  //     let WPARAM(msg) = wparam;

  //     match msg as u32 {
  //       WM_RBUTTONUP => {
  //         dbg!("====================");
  //       }
  //       WM_LBUTTONDOWN => {
  //         dbg!("=============左边");
  //       }
  //       _ => (),
  //     }

  //     CallNextHookEx(HHOOK(0), code, wparam, lparam)
  //   }
  // }

  builder.run(move |_app_handle, _e| {})
}
