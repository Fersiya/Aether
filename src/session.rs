//! Состояние браузера: список вкладок, активная вкладка, быстрые карточки и счётчики визитов.
//! Модуль не зависит от wry/tao — это чистые данные, поэтому он покрыт юнит-тестами.
//! Источник истины — Rust: страницы получают снимок состояния (`state_json`) и шлют намерения через IPC.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;

pub const START_URL: &str = "http://aether.localhost/";
const START_HOST: &str = "aether.localhost";
const START_TITLE: &str = "Старт";
const NEW_TAB_TITLE: &str = "Новая вкладка";
const MAX_TITLE_CHARS: usize = 200;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TabInfo {
    pub id: u32,
    pub url: String,
    pub title: String,
}

/// Формат файла session.json
#[derive(Serialize, Deserialize, Default)]
struct Saved {
    active: u32,
    next_id: u32,
    tabs: Vec<TabInfo>,
    #[serde(default)]
    visits: HashMap<String, u32>,
}

#[derive(Debug, PartialEq)]
pub enum CloseResult {
    NotFound,
    /// Закрыта последняя вкладка — вместо неё остаётся стартовая страница.
    ResetToStart,
    Removed { was_active: bool },
}

pub struct Session {
    pub tabs: Vec<TabInfo>,
    pub active: u32,
    next_id: u32,
    visits: HashMap<String, u32>,
    pub cards: Value,
}

pub fn host_of(url: &str) -> String {
    let rest = url.split("://").nth(1).unwrap_or(url);
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    let host = host.rsplit('@').next().unwrap_or(host);
    let host = host.split(':').next().unwrap_or(host);
    host.strip_prefix("www.").unwrap_or(host).to_string()
}

pub fn is_start(url: &str) -> bool {
    host_of(url) == START_HOST
}

pub fn normalize_url(raw: &str) -> String {
    let u = raw.trim();
    let l = u.to_ascii_lowercase();
    if l.starts_with("http://") || l.starts_with("https://") || l.starts_with("aether://") {
        u.to_string()
    } else {
        format!("https://{}", u)
    }
}

impl Session {
    /// Загружает сохранённую сессию; если файла нет или он повреждён — одна стартовая вкладка.
    pub fn load(path: &Path, cards: Value) -> Session {
        let saved: Option<Saved> = std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok());

        let mut s = Session {
            tabs: Vec::new(),
            active: 0,
            next_id: 1,
            visits: HashMap::new(),
            cards,
        };

        if let Some(sv) = saved {
            s.tabs = sv.tabs.into_iter().filter(|t| !t.url.trim().is_empty()).collect();
            s.visits = sv.visits;
            let max_id = s.tabs.iter().map(|t| t.id).max().unwrap_or(0);
            s.next_id = sv.next_id.max(max_id + 1);
            s.active = sv.active;
        }

        if s.tabs.is_empty() {
            let id = s.add_tab(START_URL);
            s.active = id;
        } else if !s.tabs.iter().any(|t| t.id == s.active) {
            s.active = s.tabs[0].id;
        }
        s
    }

    pub fn to_json(&self) -> String {
        let saved = Saved {
            active: self.active,
            next_id: self.next_id,
            tabs: self.tabs.clone(),
            visits: self.visits.clone(),
        };
        serde_json::to_string(&saved).unwrap_or_default()
    }

    /// Снимок для страниц (формат window.__AETHER_STATE__).
    pub fn state_json(&self) -> String {
        json!({
            "activeTabId": self.active,
            "tabs": self.tabs,
            "cards": self.cards,
            "visits": self.visits,
        })
        .to_string()
    }

    pub fn url_of(&self, id: u32) -> Option<String> {
        self.tabs.iter().find(|t| t.id == id).map(|t| t.url.clone())
    }

    pub fn add_tab(&mut self, url: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let title = if is_start(url) { START_TITLE } else { NEW_TAB_TITLE };
        self.tabs.push(TabInfo { id, url: url.to_string(), title: title.to_string() });
        id
    }

    pub fn close(&mut self, id: u32) -> CloseResult {
        let Some(idx) = self.tabs.iter().position(|t| t.id == id) else {
            return CloseResult::NotFound;
        };
        if self.tabs.len() == 1 {
            let t = &mut self.tabs[0];
            t.url = START_URL.to_string();
            t.title = START_TITLE.to_string();
            return CloseResult::ResetToStart;
        }
        self.tabs.remove(idx);
        let was_active = self.active == id;
        if was_active {
            self.active = self.tabs[idx.saturating_sub(1).min(self.tabs.len() - 1)].id;
        }
        CloseResult::Removed { was_active }
    }

    /// Первая вкладка со стартовой страницей (для «Домой»).
    pub fn start_tab(&self) -> Option<u32> {
        self.tabs.iter().find(|t| is_start(&t.url)).map(|t| t.id)
    }

    /// Пользователь инициировал переход: сразу показываем адрес и временный заголовок.
    pub fn begin_nav(&mut self, id: u32, url: &str) {
        if let Some(t) = self.tabs.iter_mut().find(|t| t.id == id) {
            t.url = url.to_string();
            t.title = if is_start(url) { START_TITLE.to_string() } else { host_of(url) };
        }
    }

    /// Вебвью сообщило о переходе (в том числе по ссылке на странице).
    pub fn on_navigated(&mut self, id: u32, url: &str) {
        if url.is_empty() || url.starts_with("about:") {
            return;
        }
        let host = host_of(url);
        let start = host == START_HOST;
        let Some(t) = self.tabs.iter_mut().find(|t| t.id == id) else { return };
        // Вкладка, открытая сразу с адресом, имеет тот же url, но ещё «пустой» заголовок — это тоже новый визит
        let fresh = t.title.is_empty() || t.title == NEW_TAB_TITLE;
        let changed = t.url != url || fresh;
        t.url = url.to_string();
        if changed {
            t.title = if start { START_TITLE.to_string() } else { host.clone() };
            if !start && host.contains('.') {
                *self.visits.entry(host).or_insert(0) += 1;
            }
        }
    }

    /// Обновление адреса без счёта визитов (например, SPA изменил URL через pushState).
    pub fn sync_url(&mut self, id: u32, url: &str) {
        if url.is_empty() || url.starts_with("about:") {
            return;
        }
        if let Some(t) = self.tabs.iter_mut().find(|t| t.id == id) {
            t.url = url.to_string();
        }
    }

    pub fn set_title(&mut self, id: u32, title: &str) {
        let title: String = title.trim().chars().take(MAX_TITLE_CHARS).collect();
        if title.is_empty() {
            return;
        }
        if let Some(t) = self.tabs.iter_mut().find(|t| t.id == id) {
            if is_start(&t.url) {
                return; // у стартовой страницы заголовок всегда «Старт»
            }
            t.title = title;
        }
    }

    pub fn set_cards(&mut self, cards: Value) {
        if cards.is_array() {
            self.cards = cards;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> Session {
        Session::load(Path::new("/nonexistent/session.json"), json!([]))
    }

    #[test]
    fn host_and_start_detection() {
        assert_eq!(host_of("https://www.youtube.com/watch?v=1"), "youtube.com");
        assert_eq!(host_of("http://user:pw@example.com:8080/x"), "example.com");
        assert_eq!(host_of("https://a.b.c?x=1"), "a.b.c");
        assert!(is_start("http://aether.localhost/"));
        assert!(is_start("http://aether.localhost/#x"));
        assert!(!is_start("https://example.com/aether.localhost"));
    }

    #[test]
    fn normalize() {
        assert_eq!(normalize_url("  example.com "), "https://example.com");
        assert_eq!(normalize_url("HTTP://x.y"), "HTTP://x.y");
        assert_eq!(normalize_url("https://x.y/a b"), "https://x.y/a b");
    }

    #[test]
    fn fresh_session_has_one_start_tab() {
        let s = fresh();
        assert_eq!(s.tabs.len(), 1);
        assert_eq!(s.active, s.tabs[0].id);
        assert!(is_start(&s.tabs[0].url));
        assert_eq!(s.tabs[0].title, "Старт");
    }

    #[test]
    fn close_picks_left_neighbour_and_keeps_one_tab() {
        let mut s = fresh();
        let a = s.tabs[0].id;
        let b = s.add_tab("https://b.example/");
        let c = s.add_tab("https://c.example/");
        s.active = c;
        assert_eq!(s.close(c), CloseResult::Removed { was_active: true });
        assert_eq!(s.active, b);
        assert_eq!(s.close(a), CloseResult::Removed { was_active: false });
        assert_eq!(s.active, b);
        assert_eq!(s.close(b), CloseResult::ResetToStart);
        assert_eq!(s.tabs.len(), 1);
        assert!(is_start(&s.tabs[0].url));
        assert_eq!(s.close(999), CloseResult::NotFound);
    }

    #[test]
    fn closing_first_active_tab_picks_new_first() {
        let mut s = fresh();
        let a = s.tabs[0].id;
        let b = s.add_tab("https://b.example/");
        s.active = a;
        assert_eq!(s.close(a), CloseResult::Removed { was_active: true });
        assert_eq!(s.active, b);
    }

    #[test]
    fn visits_count_only_real_changes() {
        let mut s = fresh();
        let id = s.add_tab("https://yt.example/");
        s.on_navigated(id, "https://www.youtube.com/");
        s.on_navigated(id, "https://www.youtube.com/"); // перезагрузка
        s.on_navigated(id, "https://www.youtube.com/feed");
        s.on_navigated(id, "http://aether.localhost/");
        s.on_navigated(id, "about:blank");
        let v: Value = serde_json::from_str(&s.state_json()).unwrap();
        assert_eq!(v["visits"]["youtube.com"], 2);
        assert!(v["visits"].get("aether.localhost").is_none());
        assert_eq!(s.tabs.iter().find(|t| t.id == id).unwrap().title, "Старт");
    }

    #[test]
    fn titles() {
        let mut s = fresh();
        let start = s.tabs[0].id;
        let id = s.add_tab("https://x.example/");
        s.on_navigated(id, "https://x.example/");
        assert_eq!(s.tabs[1].title, "x.example");
        s.set_title(id, "  Привет  ");
        assert_eq!(s.tabs[1].title, "Привет");
        s.set_title(id, "   ");
        assert_eq!(s.tabs[1].title, "Привет");
        s.set_title(start, "Aether — Новая вкладка");
        assert_eq!(s.tabs[0].title, "Старт");
        s.set_title(id, &"я".repeat(500));
        assert_eq!(s.tabs[1].title.chars().count(), 200);
    }

    #[test]
    fn save_and_restore_roundtrip() {
        let dir = std::env::temp_dir().join(format!("aether_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");

        let mut s = fresh();
        let b = s.add_tab("https://b.example/page");
        s.set_title(b, "B page");
        s.on_navigated(b, "https://b.example/page");
        s.active = b;
        std::fs::write(&path, s.to_json()).unwrap();

        let r = Session::load(&path, json!([{"name": "X", "url": "https://x.y"}]));
        assert_eq!(r.tabs, s.tabs);
        assert_eq!(r.active, b);
        let c = { let mut r = r; r.add_tab("https://c.example/") };
        assert!(c > b, "id новой вкладки не должен повторять старые");

        std::fs::write(&path, "{ битый json").unwrap();
        let broken = Session::load(&path, json!([]));
        assert_eq!(broken.tabs.len(), 1);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn stale_active_id_falls_back_to_first_tab() {
        let dir = std::env::temp_dir().join(format!("aether_test2_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("session.json");
        std::fs::write(
            &path,
            r#"{"active":42,"next_id":3,"tabs":[{"id":1,"url":"https://a.example/","title":"A"},{"id":2,"url":"","title":"empty"}]}"#,
        )
        .unwrap();
        let r = Session::load(&path, json!([]));
        assert_eq!(r.tabs.len(), 1);
        assert_eq!(r.active, 1);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn cards_must_be_array() {
        let mut s = fresh();
        s.set_cards(json!({"a": 1}));
        assert!(s.cards.as_array().unwrap().is_empty());
        s.set_cards(json!([{"name": "A", "url": "https://a.b"}]));
        assert_eq!(s.cards.as_array().unwrap().len(), 1);
    }
}
