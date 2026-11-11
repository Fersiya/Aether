#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scripts;

use directories::ProjectDirs;
use std::path::PathBuf;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Theme, WindowBuilder},
};
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
use wry::{WebContext, WebViewBuilder};

const APP_NAME: &str = "Aether";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();

    // 1. Создаем нативное окно через Tao с глубокой тёмной темой
    let window = WindowBuilder::new()
        .with_title(format!("{} v0.5.7 — Легкий медиа-браузер", APP_NAME))
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 900.0))
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)?;

    // 1.1 Принудительное включение тёмного заголовка DWM на Windows 10/11
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

    // 2. Настраиваем хранилище профиля для сохранения всех сессий, куки и стартовой страницы
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

    // 2.1 Подготавливаем кастомную фиолетовую стартовую страницу Aether
    let start_html_path = profile_dir.join("start.html");
    std::fs::write(&start_html_path, scripts::START_PAGE_HTML).ok();
    let start_url = format!("file:///{}", start_html_path.to_string_lossy().replace('\\', "/"));

    let mut web_context = WebContext::new(Some(profile_dir));

    let core_script = scripts::get_core_script(&start_url);

    // 3. Инициализируем рендерер Wry с переданным контекстом
    let _webview = WebViewBuilder::with_web_context(&mut web_context)
        .with_clipboard(true)
        .with_devtools(true)
        .with_initialization_script(&core_script)
        .with_url(&start_url)
        .build(&window)?;

    // 4. Главный цикл событий
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("[Aether v0.5.7] Запущен успешно!");
                println!("  Стартовая страница: {}", start_url);
                println!("  F6 / Ctrl+L: поиск / омнибокс");
                println!("  Ctrl+T:      новая вкладка");
                println!("  Ctrl+W:      закрыть вкладку");
                println!("  Ctrl+Tab:    переключение вкладок");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
