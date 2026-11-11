#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scripts;

use directories::{ProjectDirs, UserDirs};
use std::path::PathBuf;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Fullscreen, Icon, Theme, WindowBuilder},
};
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
use wry::{http::Request, NewWindowResponse, WebContext, WebViewBuilder};

const APP_NAME: &str = "Aether";
const START_URL: &str = "http://aether.localhost/";
const ICON_RGBA: &[u8] = include_bytes!("../assets/icon_256.rgba");

#[derive(Debug, Clone)]
enum UserEvent {
    GoHome,
    Navigate(String),
    SaveCards(String),
    SetFullscreen(bool),
    OpenNewTab(String),
}

fn get_desktop_log_path() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(desktop) = user_dirs.desktop_dir() {
            return desktop.join("aether_debug.log");
        }
    }
    PathBuf::from("aether_debug.log")
}

pub fn log_desktop(msg: &str) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let line = format!("[time: {}] {}\r\n", now, msg);
    eprintln!("{}", line.trim());
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(get_desktop_log_path()) {
        use std::io::Write;
        let _ = file.write_all(line.as_bytes());
    }
}

fn show_error_dialog(title: &str, message: &str) {
    #[cfg(target_os = "windows")]
    unsafe {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        let wide_msg: Vec<u16> = OsStr::new(message).encode_wide().chain(std::iter::once(0)).collect();
        let wide_title: Vec<u16> = OsStr::new(title).encode_wide().chain(std::iter::once(0)).collect();
        extern "system" {
            fn MessageBoxW(hwnd: isize, text: *const u16, caption: *const u16, utype: u32) -> i32;
        }
        MessageBoxW(0, wide_msg.as_ptr(), wide_title.as_ptr(), 0x10 /* MB_ICONERROR */);
    }
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    log_desktop("=== Aether Browser v0.6.6: Запуск приложения ===");

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let new_window_proxy = proxy.clone();

    // 1. Создаем нативное окно через Tao с глубокой тёмной темой и качественной иконкой
    let window_icon = Icon::from_rgba(ICON_RGBA.to_vec(), 256, 256).ok();

    let window = WindowBuilder::new()
        .with_title(format!("{} v0.6.6 — Легкий медиа-браузер", APP_NAME))
        .with_window_icon(window_icon)
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 900.0))
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)?;

    // 1.1 Тёмная полоса заголовка Windows DWM
    #[cfg(target_os = "windows")]
    unsafe {
        extern "system" {
            fn LoadLibraryA(lpLibFileName: *const u8) -> *mut std::ffi::c_void;
            fn GetProcAddress(hModule: *mut std::ffi::c_void, lpProcName: *const u8) -> *mut std::ffi::c_void;
        }
        let dwmapi = LoadLibraryA(b"dwmapi.dll\0".as_ptr());
        if !dwmapi.is_null() {
            let func = GetProcAddress(dwmapi, b"DwmSetWindowAttribute\0".as_ptr());
            if !func.is_null() {
                let set_attr: extern "system" fn(isize, u32, *const i32, u32) -> i32 = std::mem::transmute(func);
                let dark: i32 = 1;
                let hwnd = window.hwnd() as isize;
                set_attr(hwnd, 20, &dark, 4); // DWMWA_USE_IMMERSIVE_DARK_MODE (Windows 11 / Windows 10 2004+)
                set_attr(hwnd, 19, &dark, 4); // DWMWA_USE_IMMERSIVE_DARK_MODE (Windows 10 1903/1909)
                let caption_color: u32 = 0x0014070B; // Тёмно-фиолетовый акцент заголовка на Windows 11
                set_attr(hwnd, 35, &caption_color as *const _ as *const i32, 4);
            }
        }
    }

    // 2. Каталог профиля для сохранения всех сессий, куки и истории
    let profile_dir = if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
        let dir = proj_dirs.data_dir().to_path_buf();
        std::fs::create_dir_all(&dir).ok();
        dir
    } else {
        let dir = PathBuf::from("./aether_profile");
        std::fs::create_dir_all(&dir).ok();
        dir
    };

    log_desktop(&format!("Profile dir: {:?}", profile_dir));

    // 2.1 Загружаем сохраненные пользовательские карточки
    let cards_json_path = profile_dir.join("cards.json");
    let saved_cards = if cards_json_path.exists() {
        std::fs::read_to_string(&cards_json_path).unwrap_or_else(|_| "[]".to_string())
    } else {
        "[]".to_string()
    };

    let mut web_context = WebContext::new(Some(profile_dir));
    let core_script = scripts::get_core_script(START_URL, &saved_cards);

    // 3. Инициализируем рендерер Wry с протоколом aether и IPC
    let ipc_proxy = proxy.clone();
    let cards_file_ipc = cards_json_path.clone();

    let webview = WebViewBuilder::with_web_context(&mut web_context)
        .with_clipboard(true)
        .with_devtools(true)
        .with_custom_protocol("aether".into(), move |_id, _request| {
            let body = scripts::START_PAGE_HTML.as_bytes().to_vec();
            wry::http::Response::builder()
                .header(wry::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
                .header("Access-Control-Allow-Origin", "*")
                .body(std::borrow::Cow::Owned(body))
                .unwrap()
        })
        .with_initialization_script(&core_script)
        .with_new_window_req_handler(move |url, _features| {
            log_desktop(&format!("with_new_window_req_handler: перехвачен запрос нового окна: {}", url));
            let _ = new_window_proxy.send_event(UserEvent::OpenNewTab(url));
            NewWindowResponse::Deny
        })
        .with_ipc_handler(move |req: Request<String>| {
            let body = req.body().trim();
            if body == "go_home" {
                log_desktop("IPC handler: go_home");
                ipc_proxy.send_event(UserEvent::GoHome).ok();
            } else if let Some(url) = body.strip_prefix("navigate:") {
                log_desktop(&format!("IPC handler: navigate to {}", url));
                ipc_proxy.send_event(UserEvent::Navigate(url.to_string())).ok();
            } else if let Some(json) = body.strip_prefix("save_cards:") {
                log_desktop("IPC handler: save_cards");
                let _ = std::fs::write(&cards_file_ipc, json);
                ipc_proxy.send_event(UserEvent::SaveCards(json.to_string())).ok();
            } else if let Some(fs_val) = body.strip_prefix("set_fullscreen:") {
                let enabled = fs_val.trim() == "true";
                log_desktop(&format!("IPC handler: set_fullscreen({})", enabled));
                ipc_proxy.send_event(UserEvent::SetFullscreen(enabled)).ok();
            } else if body == "open_log_in_explorer" {
                let log_path = get_desktop_log_path();
                log_desktop(&format!("IPC handler: открытие проводника с выделением файла {:?}", log_path));
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("explorer")
                        .arg(format!("/select,{}", log_path.display()))
                        .spawn();
                }
            } else if let Some(log_msg) = body.strip_prefix("log:") {
                log_desktop(&format!("[JS] {}", log_msg));
            }
        })
        .with_url(START_URL)
        .build(&window)?;

    log_desktop(&format!("WebView2 инициализирован. Загружен START_URL: {}", START_URL));

    // 4. Главный цикл событий
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                log_desktop("[Aether v0.6.6] Готов к работе!");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log_desktop("Окно закрыто пользователем. Завершение работы.");
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(user_event) => {
                match user_event {
                    UserEvent::GoHome => {
                        log_desktop("UserEvent::GoHome -> webview.load_url(http://aether.localhost/)");
                        let res = webview.load_url(START_URL);
                        log_desktop(&format!("load_url результат: {:?}", res));
                    }
                    UserEvent::Navigate(target_url) => {
                        let valid_url = if target_url.starts_with("http://") || target_url.starts_with("https://") || target_url.starts_with("aether://") {
                            target_url
                        } else {
                            format!("https://{}", target_url)
                        };
                        log_desktop(&format!("UserEvent::Navigate -> webview.load_url({})", valid_url));
                        let res = webview.load_url(&valid_url);
                        log_desktop(&format!("load_url результат: {:?}", res));
                    }
                    UserEvent::SetFullscreen(enabled) => {
                        log_desktop(&format!("UserEvent::SetFullscreen({})", enabled));
                        if enabled {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        } else {
                            window.set_fullscreen(None);
                        }
                    }
                    UserEvent::OpenNewTab(url) => {
                        log_desktop(&format!("UserEvent::OpenNewTab({})", url));
                        let script = format!("if (typeof createNewTab === 'function') {{ createNewTab({:?}); }} else {{ window.location.href = {:?}; }}", url, url);
                        let _ = webview.evaluate_script(&script);
                    }
                    UserEvent::SaveCards(_) => {}
                }
            }
            _ => (),
        }
    });
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("Критическая ошибка Aether Browser:\n{}\n", info);
        eprintln!("{}", msg);
        log_desktop(&format!("CRITICAL PANIC:\n{}", info));
        if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
            let crash_file = proj_dirs.data_dir().join("crash.log");
            let _ = std::fs::write(&crash_file, &msg);
        }
        show_error_dialog("Aether Browser — Ошибка запуска", &msg);
    }));

    if let Err(e) = run_app() {
        let msg = format!("Ошибка работы браузера: {:?}\n", e);
        eprintln!("{}", msg);
        log_desktop(&format!("App error: {:?}", e));
        show_error_dialog("Aether Browser — Ошибка", &msg);
    }
}
