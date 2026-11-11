#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scripts;

use directories::ProjectDirs;
use std::path::PathBuf;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Theme, WindowBuilder},
};
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
use wry::{http::Request, WebContext, WebViewBuilder};

const APP_NAME: &str = "Aether";

#[derive(Debug, Clone)]
enum UserEvent {
    GoHome,
    Navigate(String),
    SaveCards(String),
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
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // 1. Создаем нативное окно через Tao с глубокой тёмной темой
    let window = WindowBuilder::new()
        .with_title(format!("{} v0.6.2 — Легкий медиа-браузер", APP_NAME))
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

    println!("[Aether] Каталог профиля: {:?}", profile_dir);

    // 2.1 Загружаем сохраненные пользовательские карточки (по умолчанию пустой список)
    let cards_json_path = profile_dir.join("cards.json");
    let saved_cards = if cards_json_path.exists() {
        std::fs::read_to_string(&cards_json_path).unwrap_or_else(|_| "[]".to_string())
    } else {
        "[]".to_string()
    };

    // 2.2 Подготавливаем кастомную фиолетовую стартовую страницу Aether
    let start_html_path = profile_dir.join("start.html");
    std::fs::write(&start_html_path, scripts::START_PAGE_HTML).ok();
    let start_url = format!("file:///{}", start_html_path.to_string_lossy().replace('\\', "/"));

    let mut web_context = WebContext::new(Some(profile_dir));
    let core_script = scripts::get_core_script(&start_url, &saved_cards);

    // 3. Инициализируем рендерер Wry с переданным контекстом и IPC-обработчиком
    let ipc_proxy = proxy.clone();
    let cards_file_ipc = cards_json_path.clone();

    let webview = WebViewBuilder::with_web_context(&mut web_context)
        .with_clipboard(true)
        .with_devtools(true)
        .with_initialization_script(&core_script)
        .with_ipc_handler(move |req: Request<String>| {
            let body = req.body().trim();
            if body == "go_home" {
                ipc_proxy.send_event(UserEvent::GoHome).ok();
            } else if let Some(url) = body.strip_prefix("navigate:") {
                ipc_proxy.send_event(UserEvent::Navigate(url.to_string())).ok();
            } else if let Some(json) = body.strip_prefix("save_cards:") {
                let _ = std::fs::write(&cards_file_ipc, json);
                ipc_proxy.send_event(UserEvent::SaveCards(json.to_string())).ok();
            }
        })
        .with_url(&start_url)
        .build(&window)?;

    // 4. Главный цикл событий
    let start_url_clone = start_url.clone();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("[Aether v0.6.2] Запущен успешно в стиле Arc!");
                println!("  Стартовая страница: {}", start_url_clone);
                println!("  Ctrl+B:      свернуть/развернуть боковой сайдбар");
                println!("  Ctrl+T:      новая вкладка");
                println!("  Ctrl+W:      закрыть вкладку");
                println!("  F6 / Ctrl+L: поиск / омнибокс");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(user_event) => {
                match user_event {
                    UserEvent::GoHome => {
                        webview.load_url(&start_url_clone).ok();
                    }
                    UserEvent::Navigate(target_url) => {
                        webview.load_url(&target_url).ok();
                    }
                    UserEvent::SaveCards(_) => {}
                }
            }
            _ => (),
        }
    });
}

fn main() {
    // Устанавливаем перехватчик паник для записи логов и показа нативного окна ошибки
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("Критическая ошибка Aether Browser:\n{}\n", info);
        eprintln!("{}", msg);
        if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
            let crash_file = proj_dirs.data_dir().join("crash.log");
            let _ = std::fs::write(&crash_file, &msg);
        }
        show_error_dialog("Aether Browser — Ошибка запуска", &msg);
    }));

    if let Err(e) = run_app() {
        let msg = format!("Ошибка работы браузера: {:?}\n", e);
        eprintln!("{}", msg);
        if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
            let crash_file = proj_dirs.data_dir().join("crash.log");
            let _ = std::fs::write(&crash_file, &msg);
        }
        show_error_dialog("Aether Browser — Ошибка", &msg);
    }
}
