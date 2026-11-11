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
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    http::Request,
    Rect, WebContext, WebView, WebViewBuilder,
};

const APP_NAME: &str = "Aether";
const SIDEBAR_WIDTH: f64 = 240.0;

#[derive(Debug, Clone)]
enum UserEvent {
    SwitchTab(u64),
    NewTab(Option<String>),
    CloseTab(u64),
    CloseActiveTab,
    Navigate(String),
    OpenFavorite(String),
    Back,
    Forward,
    Reload,
    GoHome,
    ToggleSidebar,
    TabNavigated { id: u64, title: String, url: String },
    AudioToggle,
    PiPToggle,
    DownloadMedia,
}

struct Tab {
    id: u64,
    title: String,
    url: String,
    webview: WebView,
}

fn sync_sidebar(sidebar_wv: &WebView, tabs: &[Tab], active_id: u64) {
    let mut json = String::from("[");
    for (i, t) in tabs.iter().enumerate() {
        if i > 0 { json.push(','); }
        json.push_str(&format!(
            "{{\"id\":{},\"title\":{:?},\"url\":{:?}}}",
            t.id, t.title, t.url
        ));
    }
    json.push(']');
    let script = format!("if (window.updateTabs) {{ window.updateTabs({}, {}); }}", json, active_id);
    sidebar_wv.evaluate_script(&script).ok();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // 1. Создаем нативное окно Tao с тёмной темой
    let window = WindowBuilder::new()
        .with_title(format!("{} v0.6.0 — Легкий медиа-браузер", APP_NAME))
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
                set_attr(hwnd, 20, &dark, 4);
                set_attr(hwnd, 19, &dark, 4);
                let caption_color: u32 = 0x0014070B;
                set_attr(hwnd, 35, &caption_color as *const _ as *const i32, 4);
            }
        }
    }

    // 2. Каталог профиля
    let profile_dir = if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
        let dir = proj_dirs.data_dir().to_path_buf();
        std::fs::create_dir_all(&dir).ok();
        dir
    } else {
        let dir = PathBuf::from("./aether_profile");
        std::fs::create_dir_all(&dir).ok();
        dir
    };

    // 2.1 Подготавливаем локальные файлы
    let start_html_path = profile_dir.join("start.html");
    std::fs::write(&start_html_path, scripts::START_PAGE_HTML).ok();
    let start_url = format!("file:///{}", start_html_path.to_string_lossy().replace('\\', "/"));

    let sidebar_html_path = profile_dir.join("sidebar.html");
    std::fs::write(&sidebar_html_path, scripts::SIDEBAR_HTML).ok();
    let sidebar_url = format!("file:///{}", sidebar_html_path.to_string_lossy().replace('\\', "/"));

    let mut web_context = WebContext::new(Some(profile_dir));

    let win_size = window.inner_size();
    let scale = window.scale_factor();
    let mut current_width = (win_size.width as f64) / scale;
    let mut current_height = (win_size.height as f64) / scale;
    let mut sidebar_collapsed = false;

    // 3. Создаем боковой сайдбар (Sidebar WebView)
    let sb_proxy = proxy.clone();
    let sidebar_bounds = Rect {
        position: LogicalPosition::new(0.0, 0.0).into(),
        size: LogicalSize::new(SIDEBAR_WIDTH, current_height).into(),
    };

    let sidebar_wv = WebViewBuilder::with_web_context(&mut web_context)
        .with_bounds(sidebar_bounds)
        .with_clipboard(true)
        .with_devtools(true)
        .with_url(&sidebar_url)
        .with_ipc_handler(move |req: Request<String>| {
            let body = req.body().trim();
            if let Some(id_str) = body.strip_prefix("switch_tab:") {
                if let Ok(id) = id_str.parse::<u64>() {
                    sb_proxy.send_event(UserEvent::SwitchTab(id)).ok();
                }
            } else if let Some(url_str) = body.strip_prefix("new_tab:") {
                let u = if url_str.is_empty() { None } else { Some(url_str.to_string()) };
                sb_proxy.send_event(UserEvent::NewTab(u)).ok();
            } else if let Some(id_str) = body.strip_prefix("close_tab:") {
                if let Ok(id) = id_str.parse::<u64>() {
                    sb_proxy.send_event(UserEvent::CloseTab(id)).ok();
                }
            } else if let Some(url) = body.strip_prefix("navigate:") {
                sb_proxy.send_event(UserEvent::Navigate(url.to_string())).ok();
            } else if let Some(fav_url) = body.strip_prefix("open_favorite:") {
                sb_proxy.send_event(UserEvent::OpenFavorite(fav_url.to_string())).ok();
            } else if body == "back" {
                sb_proxy.send_event(UserEvent::Back).ok();
            } else if body == "forward" {
                sb_proxy.send_event(UserEvent::Forward).ok();
            } else if body == "reload" {
                sb_proxy.send_event(UserEvent::Reload).ok();
            } else if body == "go_home" {
                sb_proxy.send_event(UserEvent::GoHome).ok();
            } else if body == "toggle_sidebar" {
                sb_proxy.send_event(UserEvent::ToggleSidebar).ok();
            } else if body == "close_active_tab" {
                sb_proxy.send_event(UserEvent::CloseActiveTab).ok();
            } else if body == "toggle_audio" {
                sb_proxy.send_event(UserEvent::AudioToggle).ok();
            } else if body == "toggle_pip" {
                sb_proxy.send_event(UserEvent::PiPToggle).ok();
            } else if body == "download" {
                sb_proxy.send_event(UserEvent::DownloadMedia).ok();
            }
        })
        .build_as_child(&window)?;

    // 4. Менеджер вкладок
    let mut tabs: Vec<Tab> = Vec::new();
    let mut active_tab_id: u64 = 1;
    let mut next_tab_id: u64 = 2;

    let calc_content_bounds = |collapsed: bool, w: f64, h: f64| -> Rect {
        let sb_w = if collapsed { 0.0 } else { SIDEBAR_WIDTH };
        let content_w = (w - sb_w).max(100.0);
        Rect {
            position: LogicalPosition::new(sb_w, 0.0).into(),
            size: LogicalSize::new(content_w, h).into(),
        }
    };

    // Создаем начальную вкладку (Стартовая страница)
    {
        let tab_proxy = proxy.clone();
        let id = 1;
        let c_bounds = calc_content_bounds(sidebar_collapsed, current_width, current_height);
        let tab_wv = WebViewBuilder::with_web_context(&mut web_context)
            .with_bounds(c_bounds)
            .with_clipboard(true)
            .with_devtools(true)
            .with_visible(true)
            .with_initialization_script(scripts::CONTENT_SCRIPT)
            .with_url(&start_url)
            .with_ipc_handler(move |req: Request<String>| {
                let body = req.body().trim();
                if let Some(payload) = body.strip_prefix("tab_navigated:") {
                    let parts: Vec<&str> = payload.splitn(2, "|||").collect();
                    let title = parts.get(0).unwrap_or(&"Вкладка").to_string();
                    let url = parts.get(1).unwrap_or(&"").to_string();
                    tab_proxy.send_event(UserEvent::TabNavigated { id, title, url }).ok();
                } else if let Some(new_url) = body.strip_prefix("new_tab:") {
                    let u = if new_url.is_empty() { None } else { Some(new_url.to_string()) };
                    tab_proxy.send_event(UserEvent::NewTab(u)).ok();
                } else if body == "close_active_tab" {
                    tab_proxy.send_event(UserEvent::CloseActiveTab).ok();
                } else if body == "toggle_sidebar" {
                    tab_proxy.send_event(UserEvent::ToggleSidebar).ok();
                }
            })
            .build_as_child(&window)?;

        tabs.push(Tab {
            id,
            title: "Старт".to_string(),
            url: start_url.clone(),
            webview: tab_wv,
        });

        sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
    }

    // 5. Главный цикл событий
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("[Aether v0.6.0] Запущен успешно в стиле Arc!");
                sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                let scale = window.scale_factor();
                current_width = (physical_size.width as f64) / scale;
                current_height = (physical_size.height as f64) / scale;

                let sb_w = if sidebar_collapsed { 0.0 } else { SIDEBAR_WIDTH };
                sidebar_wv.set_bounds(Rect {
                    position: LogicalPosition::new(0.0, 0.0).into(),
                    size: LogicalSize::new(sb_w, current_height).into(),
                }).ok();

                let c_bounds = calc_content_bounds(sidebar_collapsed, current_width, current_height);
                if let Some(active_tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                    active_tab.webview.set_bounds(c_bounds).ok();
                }
            }

            Event::UserEvent(user_event) => {
                match user_event {
                    UserEvent::SwitchTab(id) => {
                        if id != active_tab_id {
                            let c_bounds = calc_content_bounds(sidebar_collapsed, current_width, current_height);
                            for t in &tabs {
                                if t.id == id {
                                    t.webview.set_bounds(c_bounds).ok();
                                    t.webview.set_visible(true).ok();
                                } else {
                                    t.webview.set_visible(false).ok();
                                }
                            }
                            active_tab_id = id;
                            sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
                        }
                    }

                    UserEvent::NewTab(url_opt) => {
                        let target_url = url_opt.unwrap_or_else(|| start_url.clone());
                        let id = next_tab_id;
                        next_tab_id += 1;

                        // Скрываем текущие вкладки
                        for t in &tabs {
                            t.webview.set_visible(false).ok();
                        }

                        let c_bounds = calc_content_bounds(sidebar_collapsed, current_width, current_height);
                        let tab_proxy = proxy.clone();

                        let new_wv_res = WebViewBuilder::with_web_context(&mut web_context)
                            .with_bounds(c_bounds)
                            .with_clipboard(true)
                            .with_devtools(true)
                            .with_visible(true)
                            .with_initialization_script(scripts::CONTENT_SCRIPT)
                            .with_url(&target_url)
                            .with_ipc_handler(move |req: Request<String>| {
                                let body = req.body().trim();
                                if let Some(payload) = body.strip_prefix("tab_navigated:") {
                                    let parts: Vec<&str> = payload.splitn(2, "|||").collect();
                                    let title = parts.get(0).unwrap_or(&"Вкладка").to_string();
                                    let url = parts.get(1).unwrap_or(&"").to_string();
                                    tab_proxy.send_event(UserEvent::TabNavigated { id, title, url }).ok();
                                } else if let Some(new_url) = body.strip_prefix("new_tab:") {
                                    let u = if new_url.is_empty() { None } else { Some(new_url.to_string()) };
                                    tab_proxy.send_event(UserEvent::NewTab(u)).ok();
                                } else if body == "close_active_tab" {
                                    tab_proxy.send_event(UserEvent::CloseActiveTab).ok();
                                } else if body == "toggle_sidebar" {
                                    tab_proxy.send_event(UserEvent::ToggleSidebar).ok();
                                }
                            })
                            .build_as_child(&window);

                        if let Ok(new_wv) = new_wv_res {
                            let title = if target_url.contains("start.html") { "Старт" } else { "Новая вкладка" }.to_string();
                            tabs.push(Tab {
                                id,
                                title,
                                url: target_url,
                                webview: new_wv,
                            });
                            active_tab_id = id;
                            sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
                        }
                    }

                    UserEvent::CloseTab(id) => {
                        if let Some(pos) = tabs.iter().position(|t| t.id == id) {
                            if tabs.len() <= 1 {
                                tabs[0].url = start_url.clone();
                                tabs[0].title = "Старт".to_string();
                                tabs[0].webview.evaluate_script(&format!("window.location.href = {:?};", start_url)).ok();
                                sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
                            } else {
                                let was_active = active_tab_id == id;
                                tabs.remove(pos);

                                if was_active {
                                    let next_pos = if pos >= tabs.len() { tabs.len() - 1 } else { pos };
                                    active_tab_id = tabs[next_pos].id;
                                    let c_bounds = calc_content_bounds(sidebar_collapsed, current_width, current_height);
                                    tabs[next_pos].webview.set_bounds(c_bounds).ok();
                                    tabs[next_pos].webview.set_visible(true).ok();
                                }
                                sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
                            }
                        }
                    }

                    UserEvent::CloseActiveTab => {
                        proxy.send_event(UserEvent::CloseTab(active_tab_id)).ok();
                    }

                    UserEvent::Navigate(raw_input) => {
                        let trimmed = raw_input.trim();
                        if !trimmed.is_empty() {
                            let target_url = if trimmed.to_lowercase().starts_with("yt:") || trimmed.to_lowercase().starts_with("ютуб:") {
                                let q = if trimmed.len() > 3 { trimmed[3..].trim() } else { "" };
                                let q_encoded = q.replace(' ', "+");
                                format!("https://www.youtube.com/results?search_query={}", q_encoded)
                            } else if trimmed.to_lowercase().starts_with("vk:") || trimmed.to_lowercase().starts_with("вк:") {
                                let q = if trimmed.len() > 3 { trimmed[3..].trim() } else { "" };
                                let q_encoded = q.replace(' ', "+");
                                format!("https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D={}", q_encoded)
                            } else if trimmed.to_lowercase().starts_with("gh:") {
                                let q = if trimmed.len() > 3 { trimmed[3..].trim() } else { "" };
                                let q_encoded = q.replace(' ', "+");
                                format!("https://github.com/search?q={}", q_encoded)
                            } else if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                                trimmed.to_string()
                            } else if trimmed.contains('.') && !trimmed.contains(' ') && !trimmed.ends_with('.') {
                                format!("https://{}", trimmed)
                            } else {
                                let q_encoded = trimmed.replace(' ', "+");
                                format!("https://www.google.com/search?q={}", q_encoded)
                            };

                            if let Some(tab) = tabs.iter_mut().find(|t| t.id == active_tab_id) {
                                tab.url = target_url.clone();
                                tab.webview.evaluate_script(&format!("window.location.href = {:?};", target_url)).ok();
                                sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
                            }
                        }
                    }

                    UserEvent::OpenFavorite(fav_url) => {
                        let domain = fav_url.replace("https://", "").replace("http://", "").replace("www.", "");
                        let domain_keyword = domain.split('/').next().unwrap_or(&domain);

                        if let Some(existing) = tabs.iter().find(|t| t.url.contains(domain_keyword)) {
                            let exist_id = existing.id;
                            proxy.send_event(UserEvent::SwitchTab(exist_id)).ok();
                        } else {
                            proxy.send_event(UserEvent::NewTab(Some(fav_url))).ok();
                        }
                    }

                    UserEvent::Back => {
                        if let Some(tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                            tab.webview.evaluate_script("window.history.back()").ok();
                        }
                    }

                    UserEvent::Forward => {
                        if let Some(tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                            tab.webview.evaluate_script("window.history.forward()").ok();
                        }
                    }

                    UserEvent::Reload => {
                        if let Some(tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                            tab.webview.evaluate_script("window.location.reload()").ok();
                        }
                    }

                    UserEvent::GoHome => {
                        if let Some(tab) = tabs.iter_mut().find(|t| t.id == active_tab_id) {
                            tab.url = start_url.clone();
                            tab.webview.evaluate_script(&format!("window.location.href = {:?};", start_url)).ok();
                            sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
                        }
                    }

                    UserEvent::ToggleSidebar => {
                        sidebar_collapsed = !sidebar_collapsed;
                        let sb_w = if sidebar_collapsed { 0.0 } else { SIDEBAR_WIDTH };
                        sidebar_wv.set_visible(!sidebar_collapsed).ok();
                        sidebar_wv.set_bounds(Rect {
                            position: LogicalPosition::new(0.0, 0.0).into(),
                            size: LogicalSize::new(sb_w, current_height).into(),
                        }).ok();

                        let c_bounds = calc_content_bounds(sidebar_collapsed, current_width, current_height);
                        if let Some(active_tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                            active_tab.webview.set_bounds(c_bounds).ok();
                        }
                    }

                    UserEvent::TabNavigated { id, title, url } => {
                        if let Some(tab) = tabs.iter_mut().find(|t| t.id == id) {
                            tab.title = title;
                            tab.url = url;
                            sync_sidebar(&sidebar_wv, &tabs, active_tab_id);
                        }
                    }

                    UserEvent::AudioToggle => {
                        if let Some(tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                            tab.webview.evaluate_script(r#"
                                const v = document.querySelector('video');
                                if (v) {
                                    v.style.opacity = (v.style.opacity === '0.001') ? '1' : '0.001';
                                }
                            "#).ok();
                        }
                    }

                    UserEvent::PiPToggle => {
                        if let Some(tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                            tab.webview.evaluate_script(r#"
                                const v = document.querySelector('video');
                                if (v) {
                                    if (document.pictureInPictureElement) document.exitPictureInPicture();
                                    else if (document.pictureInPictureEnabled) v.requestPictureInPicture();
                                }
                            "#).ok();
                        }
                    }

                    UserEvent::DownloadMedia => {
                        if let Some(tab) = tabs.iter().find(|t| t.id == active_tab_id) {
                            tab.webview.evaluate_script(r#"
                                const cmd = `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best" "${window.location.href}"`;
                                navigator.clipboard.writeText(cmd);
                                alert("Команда yt-dlp скопирована в буфер обмена!\n" + cmd);
                            "#).ok();
                        }
                    }
                }
            }

            _ => (),
        }
    });
}
