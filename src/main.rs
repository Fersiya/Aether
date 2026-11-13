#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scripts;
mod session;

use directories::{ProjectDirs, UserDirs};
use serde_json::Value;
use session::{CloseResult, Session, START_URL};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    window::{Fullscreen, Icon, Theme, Window, WindowBuilder},
};
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;
use wry::{dpi, http::Request, Rect, WebContext, WebView, WebViewBuilder};
#[cfg(target_os = "windows")]
use wry::{MemoryUsageLevel, WebViewExtWindows};

const APP_NAME: &str = "Aether";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const ICON_RGBA: &[u8] = include_bytes!("../assets/icon_256.rgba");

/// Сколько вкладок одновременно держит «живыми» (с запущенной страницей).
/// Остальные выгружаются (адрес и заголовок сохраняются) и открываются заново при переходе на них.
/// Вкладки, в которых играет звук/видео, не выгружаются никогда.
const MAX_LIVE_TABS: usize = 10;
/// Через сколько после скрытия вкладке урезается память (WebView2 «Low memory target»).
/// Страница при этом не замораживается: звук, сообщения и таймеры работают, просто занимает меньше памяти.
/// Небольшая пауза нужна, чтобы быстрое листание вкладок не дёргало движок.
const TRIM_AFTER: Duration = Duration::from_secs(20);
/// Пауза перед записью session.json и обновлением сайдбара после частых событий (смена заголовка).
const FLUSH_DELAY: Duration = Duration::from_millis(600);

#[derive(Debug)]
enum UserEvent {
    /// Сообщение от страницы вкладки `tab` (`trusted` — сообщение пришло со стартовой страницы Aether).
    Ipc { tab: u32, trusted: bool, body: String },
    /// Страница запросила новое окно (target=_blank, window.open) — открываем вкладку.
    NewTab(String),
    Title(u32, String),
    Navigated(u32, String),
}

// ───────────────────────────── журнал ─────────────────────────────

fn get_desktop_log_path() -> PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(desktop) = user_dirs.desktop_dir() {
            return desktop.join("aether_debug.log");
        }
    }
    PathBuf::from("aether_debug.log")
}

/// Лог-файл открывается один раз за запуск. Пишем только служебные события и ошибки —
/// адреса посещённых сайтов в журнал не попадают.
static LOG_FILE: OnceLock<Mutex<Option<std::fs::File>>> = OnceLock::new();

pub fn log_desktop(msg: &str) {
    use std::io::Write;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let line = format!("[time: {}] {}\r\n", now, msg);
    #[cfg(debug_assertions)]
    eprintln!("{}", line.trim());
    let cell = LOG_FILE.get_or_init(|| {
        Mutex::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(get_desktop_log_path())
                .ok(),
        )
    });
    if let Ok(mut guard) = cell.lock() {
        if let Some(file) = guard.as_mut() {
            let _ = file.write_all(line.as_bytes());
        }
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
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (title, message);
    }
}

// ───────────────────────────── приложение ─────────────────────────────

/// Живая вкладка: отдельный WebView, который просто скрывается при переключении.
struct View {
    wv: WebView,
    last_used: u64,
    playing: bool,
    /// Когда вкладку скрыли (None — она сейчас видима).
    hidden_at: Option<Instant>,
    /// Памяти уже урезана (нужно вернуть Normal при показе).
    trimmed: bool,
}

/// Урезает (`low = true`) или возвращает (`false`) целевой объём памяти вкладки.
/// Нужен WebView2 Runtime 114+, на старых версиях ничего не делает.
#[cfg(target_os = "windows")]
fn set_memory_low(wv: &WebView, low: bool) {
    let _ = wv.set_memory_usage_level(if low { MemoryUsageLevel::Low } else { MemoryUsageLevel::Normal });
}

#[cfg(not(target_os = "windows"))]
fn set_memory_low(_wv: &WebView, _low: bool) {}

struct App {
    window: Window,
    web_context: WebContext,
    proxy: EventLoopProxy<UserEvent>,
    session: Session,
    views: HashMap<u32, View>,
    tick: u64,
    cards_path: PathBuf,
    session_path: PathBuf,
    last_saved: String,
    flush_at: Option<Instant>,
    sweep_at: Option<Instant>,
}

impl App {
    fn bounds(&self) -> Rect {
        let s = self.window.inner_size();
        Rect {
            position: dpi::PhysicalPosition::new(0, 0).into(),
            size: dpi::PhysicalSize::new(s.width, s.height).into(),
        }
    }

    /// Создаёт WebView для вкладки `id` и загружает `url`.
    fn create_view(&mut self, id: u32, url: &str) -> bool {
        let script = scripts::get_core_script(START_URL, &self.session.state_json());
        let bounds = self.bounds();
        let ipc_proxy = self.proxy.clone();
        let window_proxy = self.proxy.clone();
        let title_proxy = self.proxy.clone();
        let nav_proxy = self.proxy.clone();

        let built = WebViewBuilder::with_web_context(&mut self.web_context)
            .with_bounds(bounds)
            .with_clipboard(true)
            .with_devtools(true)
            .with_background_color((7, 4, 13, 255))
            .with_custom_protocol("aether".into(), move |_id, _request| {
                let body = scripts::START_PAGE_HTML.as_bytes().to_vec();
                wry::http::Response::builder()
                    .header(wry::http::header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(std::borrow::Cow::Owned(body))
                    .unwrap()
            })
            .with_initialization_script(&script)
            .with_new_window_req_handler(move |url| {
                let _ = window_proxy.send_event(UserEvent::NewTab(url));
                false
            })
            .with_document_title_changed_handler(move |title| {
                let _ = title_proxy.send_event(UserEvent::Title(id, title));
            })
            .with_navigation_handler(move |url| {
                let _ = nav_proxy.send_event(UserEvent::Navigated(id, url));
                true
            })
            .with_ipc_handler(move |req: Request<String>| {
                // uri запроса — адрес страницы-отправителя: так отличаем свою стартовую страницу от чужих сайтов
                let trusted = req.uri().host() == Some("aether.localhost");
                let _ = ipc_proxy.send_event(UserEvent::Ipc { tab: id, trusted, body: req.into_body() });
            })
            .with_url(url)
            .build_as_child(&self.window);

        match built {
            Ok(wv) => {
                self.tick += 1;
                self.views.insert(
                    id,
                    View { wv, last_used: self.tick, playing: false, hidden_at: None, trimmed: false },
                );
                true
            }
            Err(e) => {
                log_desktop(&format!("Не удалось создать WebView вкладки {}: {:?}", id, e));
                false
            }
        }
    }

    /// Подтягивает актуальный адрес из вебвью (SPA меняют URL без перезагрузки).
    fn refresh_url(&mut self, id: u32) {
        let current = self.views.get(&id).and_then(|v| v.wv.url().ok());
        if let Some(url) = current {
            self.session.sync_url(id, &url);
        }
    }

    /// Делает вкладку активной: прячет прежнюю (она остаётся живой), показывает выбранную;
    /// выгруженную вкладку создаёт заново.
    fn activate(&mut self, id: u32) {
        let Some(url) = self.session.url_of(id) else { return };

        let prev = self.session.active;
        if prev != id {
            self.refresh_url(prev);
            if let Some(v) = self.views.get_mut(&prev) {
                let _ = v.wv.set_visible(false);
                v.hidden_at = Some(Instant::now());
                v.trimmed = false;
                self.schedule_sweep();
            }
        }
        self.session.active = id;

        if !self.views.contains_key(&id) && !self.create_view(id, &url) {
            return;
        }

        self.tick += 1;
        let bounds = self.bounds();
        if let Some(v) = self.views.get_mut(&id) {
            v.last_used = self.tick;
            if v.trimmed {
                set_memory_low(&v.wv, false); // вернуть обычный режим до показа
                v.trimmed = false;
            }
            v.hidden_at = None;
            let _ = v.wv.set_bounds(bounds);
            let _ = v.wv.set_visible(true);
            let _ = v.wv.focus();
        }

        self.evict();
        self.push_state();
        self.save_now();
    }

    fn schedule_sweep(&mut self) {
        if self.sweep_at.is_none() {
            self.sweep_at = Some(Instant::now() + TRIM_AFTER);
        }
    }

    /// Урезает память вкладок, которые скрыты дольше TRIM_AFTER (кроме играющих медиа).
    /// Таймер включён только пока есть кого урезать — в остальное время браузер не просыпается.
    fn sweep(&mut self) {
        let now = Instant::now();
        let mut next: Option<Instant> = None;
        for v in self.views.values_mut() {
            let Some(hidden) = v.hidden_at else { continue };
            if v.trimmed || v.playing {
                continue;
            }
            let due = hidden + TRIM_AFTER;
            if now >= due {
                set_memory_low(&v.wv, true);
                v.trimmed = true;
            } else {
                next = Some(next.map_or(due, |n| n.min(due)));
            }
        }
        self.sweep_at = next;
    }

    /// Выгружает самые давние неактивные вкладки сверх лимита (кроме играющих медиа).
    fn evict(&mut self) {
        while self.views.len() > MAX_LIVE_TABS {
            let active = self.session.active;
            let victim = self
                .views
                .iter()
                .filter(|(id, v)| **id != active && !v.playing)
                .min_by_key(|(_, v)| v.last_used)
                .map(|(id, _)| *id);
            match victim {
                Some(id) => {
                    self.refresh_url(id);
                    self.views.remove(&id);
                }
                None => break,
            }
        }
    }

    fn new_tab(&mut self, url: &str) {
        let url = if url.trim().is_empty() {
            START_URL.to_string()
        } else {
            session::normalize_url(url)
        };
        let id = self.session.add_tab(&url);
        self.activate(id);
    }

    fn close_tab(&mut self, id: u32) {
        match self.session.close(id) {
            CloseResult::NotFound => {}
            CloseResult::ResetToStart => {
                match self.views.get(&id) {
                    Some(v) => {
                        let _ = v.wv.load_url(START_URL);
                    }
                    None => {
                        self.activate(id);
                        return;
                    }
                }
                self.push_state();
                self.save_now();
            }
            CloseResult::Removed { was_active } => {
                self.views.remove(&id);
                if was_active {
                    let next = self.session.active;
                    self.activate(next);
                } else {
                    self.push_state();
                    self.save_now();
                }
            }
        }
    }

    fn navigate(&mut self, id: u32, raw: &str) {
        let url = session::normalize_url(raw);
        if let Some(v) = self.views.get(&id) {
            self.session.begin_nav(id, &url);
            let _ = v.wv.load_url(&url);
            self.push_state();
            self.save_now();
        }
    }

    fn go_home(&mut self, sender: u32) {
        match self.session.start_tab() {
            Some(start) if start != sender => self.activate(start),
            Some(_) => {}
            None => self.navigate(sender, START_URL),
        }
    }

    /// Передаёт актуальное состояние активной странице (сайдбар перерисуется, только если он открыт).
    fn push_state(&self) {
        if let Some(v) = self.views.get(&self.session.active) {
            let js = format!(
                "window.__aetherApplyState&&window.__aetherApplyState({})",
                self.session.state_json()
            );
            let _ = v.wv.evaluate_script(&js);
        }
    }

    fn save_now(&mut self) {
        let json = self.session.to_json();
        if json.is_empty() || json == self.last_saved {
            return;
        }
        let tmp = self.session_path.with_extension("json.tmp");
        if std::fs::write(&tmp, &json).is_ok() && std::fs::rename(&tmp, &self.session_path).is_ok() {
            self.last_saved = json;
        }
    }

    fn schedule_flush(&mut self) {
        if self.flush_at.is_none() {
            self.flush_at = Some(Instant::now() + FLUSH_DELAY);
        }
    }

    fn flush(&mut self) {
        self.flush_at = None;
        let active = self.session.active;
        self.refresh_url(active);
        self.push_state();
        self.save_now();
    }

    fn on_resize(&self) {
        let s = self.window.inner_size();
        if s.width == 0 || s.height == 0 {
            return; // окно свёрнуто
        }
        if let Some(v) = self.views.get(&self.session.active) {
            let _ = v.wv.set_bounds(self.bounds());
        }
    }

    fn focus_active(&self) {
        if let Some(v) = self.views.get(&self.session.active) {
            let _ = v.wv.focus();
        }
    }

    fn shutdown(&mut self) {
        let active = self.session.active;
        self.refresh_url(active);
        self.save_now();
    }

    fn on_user_event(&mut self, event: UserEvent) {
        match event {
            UserEvent::Ipc { tab, trusted, body } => self.on_ipc(tab, trusted, body.trim()),
            UserEvent::NewTab(url) => self.new_tab(&url),
            UserEvent::Title(id, title) => {
                self.session.set_title(id, &title);
                self.schedule_flush();
            }
            UserEvent::Navigated(id, url) => {
                self.session.on_navigated(id, &url);
                self.schedule_flush();
            }
        }
    }

    fn on_ipc(&mut self, tab: u32, trusted: bool, body: &str) {
        if body == "go_home" {
            self.go_home(tab);
        } else if let Some(url) = body.strip_prefix("navigate:") {
            self.navigate(tab, url);
        } else if let Some(url) = body.strip_prefix("tab_new:") {
            self.new_tab(url);
        } else if let Some(id) = body.strip_prefix("tab_switch:") {
            if let Ok(id) = id.trim().parse::<u32>() {
                self.activate(id);
            }
        } else if let Some(id) = body.strip_prefix("tab_close:") {
            if let Ok(id) = id.trim().parse::<u32>() {
                self.close_tab(id);
            }
        } else if let Some(flag) = body.strip_prefix("tab_media:") {
            let mut recheck = false;
            if let Some(v) = self.views.get_mut(&tab) {
                v.playing = flag.trim() == "1";
                if v.playing && v.trimmed {
                    // началось воспроизведение в скрытой вкладке — вернуть обычный режим
                    set_memory_low(&v.wv, false);
                    v.trimmed = false;
                }
                recheck = !v.playing && v.hidden_at.is_some();
            }
            if recheck {
                self.schedule_sweep(); // воспроизведение закончилось — вкладку теперь можно урезать
            }
        } else if let Some(enabled) = body.strip_prefix("set_fullscreen:") {
            if tab == self.session.active {
                let fs = enabled.trim() == "true";
                self.window.set_fullscreen(if fs { Some(Fullscreen::Borderless(None)) } else { None });
            }
        } else if let Some(json) = body.strip_prefix("save_cards:") {
            // Карточки может менять только стартовая страница Aether, но не сторонние сайты
            if trusted {
                if let Ok(cards) = serde_json::from_str::<Value>(json) {
                    self.session.set_cards(cards);
                    let _ = std::fs::write(&self.cards_path, json);
                    self.push_state();
                }
            }
        } else if body == "open_log_in_explorer" {
            #[cfg(target_os = "windows")]
            if trusted {
                let _ = std::process::Command::new("explorer")
                    .arg(format!("/select,{}", get_desktop_log_path().display()))
                    .spawn();
            }
        } else if let Some(msg) = body.strip_prefix("log:") {
            if trusted {
                log_desktop(&format!("[JS] {}", msg.chars().take(500).collect::<String>()));
            }
        }
    }
}

// ───────────────────────────── запуск ─────────────────────────────

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    log_desktop(&format!("=== Aether Browser v{}: запуск ===", VERSION));

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window_icon = Icon::from_rgba(ICON_RGBA.to_vec(), 256, 256).ok();

    let window = WindowBuilder::new()
        .with_title(format!("{} v{}", APP_NAME, VERSION))
        .with_window_icon(window_icon)
        .with_maximized(true)
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 900.0))
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)?;

    // Тёмная полоса заголовка Windows DWM
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

    // Каталог профиля: куки, сессии сайтов, карточки и session.json (вкладки)
    let profile_dir = if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
        proj_dirs.data_dir().to_path_buf()
    } else {
        PathBuf::from("./aether_profile")
    };
    std::fs::create_dir_all(&profile_dir).ok();

    let cards_path = profile_dir.join("cards.json");
    let cards: Value = std::fs::read_to_string(&cards_path)
        .ok()
        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
        .filter(|v| v.is_array())
        .unwrap_or_else(|| Value::Array(Vec::new()));

    let session_path = profile_dir.join("session.json");
    let session = Session::load(&session_path, cards);
    log_desktop(&format!("Восстановлено вкладок: {}", session.tabs.len()));

    let mut app = App {
        window,
        web_context: WebContext::new(Some(profile_dir)),
        proxy,
        session,
        views: HashMap::new(),
        tick: 0,
        cards_path,
        session_path,
        last_saved: String::new(),
        flush_at: None,
        sweep_at: None,
    };

    // Открываем только активную вкладку — остальные оживут при первом переходе на них
    let active = app.session.active;
    app.activate(active);
    if app.views.is_empty() {
        return Err("не удалось создать окно просмотра (WebView2)".into());
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                app.shutdown();
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Resized(_), .. } => app.on_resize(),
            Event::WindowEvent { event: WindowEvent::Focused(true), .. } => app.focus_active(),
            Event::UserEvent(ev) => app.on_user_event(ev),
            _ => {}
        }

        if !matches!(*control_flow, ControlFlow::Exit) {
            let now = Instant::now();
            if app.flush_at.is_some_and(|t| now >= t) {
                app.flush();
            }
            if app.sweep_at.is_some_and(|t| now >= t) {
                app.sweep();
            }
            // Просыпаемся только к ближайшему отложенному делу, иначе спим до события
            if let Some(next) = [app.flush_at, app.sweep_at].into_iter().flatten().min() {
                *control_flow = ControlFlow::WaitUntil(next);
            }
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
