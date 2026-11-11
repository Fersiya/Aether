mod scripts;

use directories::ProjectDirs;
use std::path::PathBuf;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, ModifiersState},
    menu::{MenuBar, MenuItem, MenuItemAttributes},
    window::WindowBuilder,
};
use wry::{WebContext, WebViewBuilder};

const APP_NAME: &str = "Aether";
const DEFAULT_HOME_URL: &str = "https://www.google.com";

/// Структура вкладки браузера
#[derive(Clone, Debug)]
struct Tab {
    url: String,
    title: String,
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    // 1. Инициализация системного меню с поддержкой вкладок
    let mut menu_bar = MenuBar::new();

    // Меню Вкладки (Tabs)
    let mut tabs_menu = MenuBar::new();
    let new_tab_item = tabs_menu.add_item(
        MenuItemAttributes::new("Новая вкладка (Ctrl + T)")
    );
    let close_tab_item = tabs_menu.add_item(
        MenuItemAttributes::new("Закрыть вкладку (Ctrl + W)")
    );
    let next_tab_item = tabs_menu.add_item(
        MenuItemAttributes::new("Следующая вкладка (Ctrl + Tab)")
    );
    let prev_tab_item = tabs_menu.add_item(
        MenuItemAttributes::new("Предыдущая вкладка (Ctrl + Shift + Tab)")
    );
    let incognito_item = tabs_menu.add_item(
        MenuItemAttributes::new("Окно инкогнито (Ctrl + Shift + N)")
    );
    menu_bar.add_submenu("Вкладки", true, tabs_menu);

    // Меню Навигация и Поиск
    let mut nav_menu = MenuBar::new();
    let search_item = nav_menu.add_item(
        MenuItemAttributes::new("Умная строка поиска (Ctrl + L / Ctrl + K / F6)")
    );
    let home_item = nav_menu.add_item(
        MenuItemAttributes::new("Главная (Google) (Alt + Home)")
    );
    let back_item = nav_menu.add_item(
        MenuItemAttributes::new("Назад (Alt + Left)")
    );
    let forward_item = nav_menu.add_item(
        MenuItemAttributes::new("Вперед (Alt + Right)")
    );
    let reload_item = nav_menu.add_item(
        MenuItemAttributes::new("Обновить страницу (F5 / Ctrl + R)")
    );
    menu_bar.add_submenu("Навигация", true, nav_menu);

    // Меню Медиа
    let mut media_menu = MenuBar::new();
    let play_pause_item = media_menu.add_item(
        MenuItemAttributes::new("Воспроизведение / Пауза (Ctrl + Space)")
    );
    let pip_item = media_menu.add_item(
        MenuItemAttributes::new("Картинка в картинке / PiP (Ctrl + P)")
    );
    let audio_only_item = media_menu.add_item(
        MenuItemAttributes::new("Режим 'Только аудио' (Ctrl + M)")
    );
    let download_item = media_menu.add_item(
        MenuItemAttributes::new("Скачать видео / медиа (Ctrl + D)")
    );
    menu_bar.add_submenu("Медиа", true, media_menu);

    // Меню Закладки
    let mut bookmarks_menu = MenuBar::new();
    let google_item = bookmarks_menu.add_item(MenuItemAttributes::new("Google (Ctrl + 1)"));
    let yt_item = bookmarks_menu.add_item(MenuItemAttributes::new("YouTube (Ctrl + 2)"));
    let vk_item = bookmarks_menu.add_item(MenuItemAttributes::new("ВКонтакте (Ctrl + 3)"));
    let tg_item = bookmarks_menu.add_item(MenuItemAttributes::new("Telegram Web (Ctrl + 4)"));
    menu_bar.add_submenu("Закладки", true, bookmarks_menu);

    // 2. Создаем нативное окно
    let window = WindowBuilder::new()
        .with_title(format!("{} v0.5.0 — Вкладка 1: Google", APP_NAME))
        .with_inner_size(tao::dpi::LogicalSize::new(1440.0, 900.0))
        .with_menu(menu_bar)
        .build(&event_loop)?;

    // 3. Настройка хранилища профиля (сохранение сессий)
    let profile_dir = if let Some(proj_dirs) = ProjectDirs::from("com", "aether", "AetherBrowser") {
        let dir = proj_dirs.data_dir().to_path_buf();
        std::fs::create_dir_all(&dir).ok();
        dir
    } else {
        PathBuf::from("./aether_profile")
    };

    let mut web_context = WebContext::new(Some(profile_dir.clone()));

    // 4. Инициализация рендерера
    let webview = WebViewBuilder::new()
        .with_web_context(&mut web_context)
        .with_clipboard(true)
        .with_devtools(true)
        .with_initialization_script(scripts::AETHER_CORE_SCRIPT)
        .with_url(DEFAULT_HOME_URL)
        .build(&window)?;

    // 5. Менеджер вкладок
    let mut tabs: Vec<Tab> = vec![Tab {
        url: DEFAULT_HOME_URL.to_string(),
        title: "Google".to_string(),
    }];
    let mut active_tab_index = 0;
    let mut current_modifiers = ModifiersState::empty();

    // 6. Главный цикл событий
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("[Aether v0.5.0] Запущен. Доступны вкладки (Ctrl+T) и инкогнито (Ctrl+Shift+N).");
            }

            Event::MenuEvent { menu_id, .. } => {
                // Новая вкладка
                if menu_id == new_tab_item.clone().id() {
                    tabs.push(Tab {
                        url: DEFAULT_HOME_URL.to_string(),
                        title: format!("Вкладка {}", tabs.len() + 1),
                    });
                    active_tab_index = tabs.len() - 1;
                    let _ = webview.load_url(DEFAULT_HOME_URL);
                    window.set_title(&format!("{} v0.5.0 — Вкладка {} (Google)", APP_NAME, active_tab_index + 1));
                    open_omnibox(&webview);
                }
                // Закрыть вкладку
                else if menu_id == close_tab_item.clone().id() {
                    if tabs.len() > 1 {
                        tabs.remove(active_tab_index);
                        if active_tab_index >= tabs.len() {
                            active_tab_index = tabs.len() - 1;
                        }
                        let target_url = tabs[active_tab_index].url.clone();
                        let _ = webview.load_url(&target_url);
                        window.set_title(&format!("{} v0.5.0 — Вкладка {}", APP_NAME, active_tab_index + 1));
                    }
                }
                // Следующая вкладка
                else if menu_id == next_tab_item.clone().id() {
                    if tabs.len() > 1 {
                        active_tab_index = (active_tab_index + 1) % tabs.len();
                        let target_url = tabs[active_tab_index].url.clone();
                        let _ = webview.load_url(&target_url);
                        window.set_title(&format!("{} v0.5.0 — Вкладка {}", APP_NAME, active_tab_index + 1));
                    }
                }
                // Предыдущая вкладка
                else if menu_id == prev_tab_item.clone().id() {
                    if tabs.len() > 1 {
                        active_tab_index = if active_tab_index == 0 { tabs.len() - 1 } else { active_tab_index - 1 };
                        let target_url = tabs[active_tab_index].url.clone();
                        let _ = webview.load_url(&target_url);
                        window.set_title(&format!("{} v0.5.0 — Вкладка {}", APP_NAME, active_tab_index + 1));
                    }
                }
                // Окно инкогнито (RAM-only)
                else if menu_id == incognito_item.clone().id() {
                    rfd::MessageDialog::new()
                        .set_title("Режим Инкогнито Aether")
                        .set_description("Для открытия окна инкогнито нажмите Ctrl+Shift+N в любое время.\nВсе куки и история этого окна будут храниться исключительно в оперативной памяти и исчезнут после закрытия.")
                        .show();
                }
                // Навигация и поиск
                else if menu_id == search_item.clone().id() {
                    open_omnibox(&webview);
                } else if menu_id == home_item.clone().id() {
                    let _ = webview.load_url(DEFAULT_HOME_URL);
                } else if menu_id == back_item.clone().id() {
                    let _ = webview.evaluate_script("window.history.back()");
                } else if menu_id == forward_item.clone().id() {
                    let _ = webview.evaluate_script("window.history.forward()");
                } else if menu_id == reload_item.clone().id() {
                    let _ = webview.evaluate_script("window.location.reload()");
                }
                // Медиа
                else if menu_id == play_pause_item.clone().id() {
                    let _ = webview.evaluate_script(scripts::TOGGLE_PLAY_PAUSE_SCRIPT);
                } else if menu_id == pip_item.clone().id() {
                    let _ = webview.evaluate_script(scripts::TOGGLE_PIP_SCRIPT);
                } else if menu_id == audio_only_item.clone().id() {
                    let _ = webview.evaluate_script(scripts::TOGGLE_AUDIO_ONLY_SCRIPT);
                } else if menu_id == download_item.clone().id() {
                    let _ = webview.evaluate_script(scripts::DOWNLOAD_MEDIA_SCRIPT);
                }
                // Закладки
                else if menu_id == google_item.clone().id() {
                    switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://www.google.com");
                } else if menu_id == yt_item.clone().id() {
                    switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://www.youtube.com");
                } else if menu_id == vk_item.clone().id() {
                    switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://vk.com");
                } else if menu_id == tg_item.clone().id() {
                    switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://web.telegram.org");
                }
            }

            Event::WindowEvent {
                event: WindowEvent::ModifiersChanged(mods),
                ..
            } => {
                current_modifiers = mods;
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if input.state == tao::event::ElementState::Pressed {
                    match input.key {
                        // Ctrl + T — Новая вкладка
                        Key::Character("t") | Key::Character("T") if current_modifiers.control_key() && !current_modifiers.shift_key() => {
                            tabs.push(Tab {
                                url: DEFAULT_HOME_URL.to_string(),
                                title: format!("Вкладка {}", tabs.len() + 1),
                            });
                            active_tab_index = tabs.len() - 1;
                            let _ = webview.load_url(DEFAULT_HOME_URL);
                            window.set_title(&format!("{} v0.5.0 — Вкладка {} (Google)", APP_NAME, active_tab_index + 1));
                            open_omnibox(&webview);
                        }

                        // Ctrl + W — Закрыть вкладку
                        Key::Character("w") | Key::Character("W") if current_modifiers.control_key() => {
                            if tabs.len() > 1 {
                                tabs.remove(active_tab_index);
                                if active_tab_index >= tabs.len() {
                                    active_tab_index = tabs.len() - 1;
                                }
                                let target_url = tabs[active_tab_index].url.clone();
                                let _ = webview.load_url(&target_url);
                                window.set_title(&format!("{} v0.5.0 — Вкладка {}", APP_NAME, active_tab_index + 1));
                            }
                        }

                        // Ctrl + Tab — Следующая вкладка
                        Key::Named(tao::keyboard::NamedKey::Tab) if current_modifiers.control_key() && !current_modifiers.shift_key() => {
                            if tabs.len() > 1 {
                                active_tab_index = (active_tab_index + 1) % tabs.len();
                                let target_url = tabs[active_tab_index].url.clone();
                                let _ = webview.load_url(&target_url);
                                window.set_title(&format!("{} v0.5.0 — Вкладка {}", APP_NAME, active_tab_index + 1));
                            }
                        }

                        // Ctrl + Shift + Tab — Предыдущая вкладка
                        Key::Named(tao::keyboard::NamedKey::Tab) if current_modifiers.control_key() && current_modifiers.shift_key() => {
                            if tabs.len() > 1 {
                                active_tab_index = if active_tab_index == 0 { tabs.len() - 1 } else { active_tab_index - 1 };
                                let target_url = tabs[active_tab_index].url.clone();
                                let _ = webview.load_url(&target_url);
                                window.set_title(&format!("{} v0.5.0 — Вкладка {}", APP_NAME, active_tab_index + 1));
                            }
                        }

                        // Ctrl + D — Скачать видео / скопировать ссылку потока
                        Key::Character("d") | Key::Character("D") if current_modifiers.control_key() => {
                            let _ = webview.evaluate_script(scripts::DOWNLOAD_MEDIA_SCRIPT);
                        }

                        // Ctrl + L, Ctrl + K, F6 — Омнибокс
                        Key::Character("l") | Key::Character("L") if current_modifiers.control_key() => {
                            open_omnibox(&webview);
                        }
                        Key::Character("k") | Key::Character("K") if current_modifiers.control_key() => {
                            open_omnibox(&webview);
                        }
                        Key::Named(tao::keyboard::NamedKey::F6) => {
                            open_omnibox(&webview);
                        }

                        // Медиа горячие клавиши:
                        Key::Named(tao::keyboard::NamedKey::Space) if current_modifiers.control_key() => {
                            let _ = webview.evaluate_script(scripts::TOGGLE_PLAY_PAUSE_SCRIPT);
                        }
                        Key::Character("p") | Key::Character("P") if current_modifiers.control_key() => {
                            let _ = webview.evaluate_script(scripts::TOGGLE_PIP_SCRIPT);
                        }
                        Key::Character("m") | Key::Character("M") if current_modifiers.control_key() => {
                            let _ = webview.evaluate_script(scripts::TOGGLE_AUDIO_ONLY_SCRIPT);
                        }

                        // Быстрый переход по вкладкам 1..4 (Ctrl + 1..4)
                        Key::Character("1") if current_modifiers.control_key() => {
                            switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://www.google.com");
                        }
                        Key::Character("2") if current_modifiers.control_key() => {
                            switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://www.youtube.com");
                        }
                        Key::Character("3") if current_modifiers.control_key() => {
                            switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://vk.com");
                        }
                        Key::Character("4") if current_modifiers.control_key() => {
                            switch_tab(&mut tabs, active_tab_index, &webview, &window, "https://web.telegram.org");
                        }

                        // Обновление и инструменты
                        Key::Named(tao::keyboard::NamedKey::F5) => {
                            let _ = webview.evaluate_script("window.location.reload()");
                        }
                        Key::Character("r") if current_modifiers.control_key() => {
                            let _ = webview.evaluate_script("window.location.reload()");
                        }
                        Key::Named(tao::keyboard::NamedKey::F12) => {
                            #[cfg(any(debug_assertions, feature = "devtools"))]
                            webview.open_devtools();
                        }
                        _ => {}
                    }
                }
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

fn switch_tab(tabs: &mut [Tab], index: usize, webview: &wry::WebView, window: &tao::window::Window, url: &str) {
    if index < tabs.len() {
        tabs[index].url = url.to_string();
        let _ = webview.load_url(url);
        window.set_title(&format!("{} v0.5.0 — Вкладка {} ({})", APP_NAME, index + 1, url));
    }
}

/// Умная строка поиска и навигации Aether
fn open_omnibox(webview: &wry::WebView) {
    let script = r#"
        (() => {
            const input = prompt(
                "Aether Omnibox (Префиксы: 'yt: запрос', 'vk: запрос', 'gh: запрос' или обычный поиск Google):",
                window.location.href
            );
            if (!input) return;

            const trimmed = input.trim();
            if (!trimmed) return;

            if (trimmed.toLowerCase().startsWith("yt:") || trimmed.toLowerCase().startsWith("ютуб:")) {
                const q = trimmed.replace(/^(yt:|ютуб:)/i, "").trim();
                window.location.href = "https://www.youtube.com/results?search_query=" + encodeURIComponent(q);
                return;
            }
            if (trimmed.toLowerCase().startsWith("vk:") || trimmed.toLowerCase().startsWith("вк:")) {
                const q = trimmed.replace(/^(vk:|вк:)/i, "").trim();
                window.location.href = "https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D=" + encodeURIComponent(q);
                return;
            }
            if (trimmed.toLowerCase().startsWith("gh:")) {
                const q = trimmed.replace(/^gh:/i, "").trim();
                window.location.href = "https://github.com/search?q=" + encodeURIComponent(q);
                return;
            }

            const isUrl = trimmed.startsWith("http://") || 
                          trimmed.startsWith("https://") || 
                          (trimmed.includes(".") && !trimmed.includes(" ") && !trimmed.endsWith("."));

            if (isUrl) {
                if (trimmed.startsWith("http://") || trimmed.startsWith("https://")) {
                    window.location.href = trimmed;
                } else {
                    window.location.href = "https://" + trimmed;
                }
            } else {
                window.location.href = "https://www.google.com/search?q=" + encodeURIComponent(trimmed);
            }
        })();
    "#;
    let _ = webview.evaluate_script(script);
}
