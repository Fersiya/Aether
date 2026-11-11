mod scripts;

use directories::ProjectDirs;
use std::path::PathBuf;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebContext, WebViewBuilder};

const APP_NAME: &str = "Aether";
const DEFAULT_HOME_URL: &str = "https://www.google.com";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();

    // 1. Создаем нативное окно через Tao (чистый кроссплатформенный интерфейс)
    let window = WindowBuilder::new()
        .with_title(format!("{} v0.5.0 — Легкий медиа-браузер", APP_NAME))
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 900.0))
        .build(&event_loop)?;

    // 2. Настраиваем хранилище профиля для сохранения всех сессий и куки
    let profile_dir = if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
        let dir = proj_dirs.data_dir().to_path_buf();
        std::fs::create_dir_all(&dir).ok();
        dir
    } else {
        PathBuf::from("./aether_profile")
    };

    println!("[Aether] Каталог профиля: {:?}", profile_dir);
    let mut web_context = WebContext::new(Some(profile_dir));

    // 3. Инициализируем рендерер Wry с переданным контекстом
    let _webview = WebViewBuilder::with_web_context(&mut web_context)
        .with_clipboard(true)
        .with_devtools(true)
        .with_initialization_script(scripts::AETHER_CORE_SCRIPT)
        .with_url(DEFAULT_HOME_URL)
        .build(&window)?;

    // 4. Главный цикл событий
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("[Aether v0.5.0] Запущен успешно!");
                println!("  F6 / Ctrl+L: поиск в Google / ввод адреса");
                println!("  Ctrl+Space:  пауза/плей (YouTube / ВК)");
                println!("  Ctrl+P:      режим Picture-in-Picture");
                println!("  Ctrl+M:      режим 'Только аудио'");
                println!("  Ctrl+D:      скачать медиа / yt-dlp");
                println!("  Ctrl+1..4:   быстрые сервисы (Google, YouTube, VK, Telegram)");
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
