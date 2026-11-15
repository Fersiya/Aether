//! Блокировка рекламы и трекеров на уровне сети.
//!
//! WebView2 зовёт обработчик только для адресов из списка доменов (узкие фильтры),
//! поэтому обычные запросы не замедляются. Сопоставление хоста — чистая функция (тесты ниже).

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::OnceLock;

const LIST: &str = include_str!("../assets/blocklist.txt");

static ENABLED: AtomicBool = AtomicBool::new(true);
static BLOCKED: AtomicU64 = AtomicU64::new(0);
static DOMAINS: OnceLock<HashSet<String>> = OnceLock::new();

pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}
pub fn blocked_count() -> u64 {
    BLOCKED.load(Ordering::Relaxed)
}

/// Разбирает текст списка: по домену в строке, `#` — комментарий.
pub fn parse_list(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_ascii_lowercase())
        .filter(|l| !l.is_empty() && !l.contains(char::is_whitespace))
        .collect()
}

pub fn domains() -> &'static HashSet<String> {
    DOMAINS.get_or_init(|| parse_list(LIST).into_iter().collect())
}

/// Хост (или любой его родительский домен) есть в списке?
pub fn host_blocked(host: &str, set: &HashSet<String>) -> bool {
    let mut h = host.trim_end_matches('.').to_ascii_lowercase();
    loop {
        if set.contains(&h) {
            return true;
        }
        match h.find('.') {
            Some(i) => h = h[i + 1..].to_string(),
            None => return false,
        }
    }
}

/// Хост из URL вида scheme://user@host:port/path.
pub fn host_of_url(url: &str) -> &str {
    let rest = url.split("://").nth(1).unwrap_or(url);
    let auth = rest.split(['/', '?', '#']).next().unwrap_or("");
    let auth = auth.rsplit('@').next().unwrap_or(auth);
    auth.split(':').next().unwrap_or(auth)
}

/// Подключает блокировку к WebView2 вкладки.
#[cfg(target_os = "windows")]
pub fn install(wv: &wry::WebView) {
    use wry::WebViewExtWindows;
    use webview2_com::take_pwstr;
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2_2, COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL,
    };
    use webview2_com::WebResourceRequestedEventHandler;
    use windows::Win32::System::Com::IStream;
    use windows_core::{Interface, HSTRING, PCWSTR, PWSTR};

    // COREWEBVIEW2_WEB_RESOURCE_CONTEXT_DOCUMENT — сами страницы не трогаем
    const CONTEXT_DOCUMENT: i32 = 1;

    unsafe {
        let Ok(core) = wv.controller().CoreWebView2() else { return };
        let Ok(core2) = core.cast::<ICoreWebView2_2>() else { return };
        let Ok(env) = core2.Environment() else { return };

        for d in domains() {
            let a = HSTRING::from(format!("*://{}/*", d));
            let b = HSTRING::from(format!("*://*.{}/*", d));
            let _ = core.AddWebResourceRequestedFilter(PCWSTR(a.as_ptr()), COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL);
            let _ = core.AddWebResourceRequestedFilter(PCWSTR(b.as_ptr()), COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL);
        }

        let handler = WebResourceRequestedEventHandler::create(Box::new(move |_sender, args| {
            let Some(args) = args else { return Ok(()) };
            if !enabled() {
                return Ok(());
            }
            let mut ctx = COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL;
            let _ = args.ResourceContext(&mut ctx);
            if ctx.0 == CONTEXT_DOCUMENT {
                return Ok(());
            }
            let req = args.Request()?;
            let mut uri = PWSTR::null();
            req.Uri(&mut uri)?;
            let url = take_pwstr(uri);
            if host_blocked(host_of_url(&url), domains()) {
                let resp = env.CreateWebResourceResponse(
                    None::<&IStream>,
                    403,
                    &HSTRING::from("Blocked"),
                    &HSTRING::from(""),
                )?;
                args.SetResponse(&resp)?;
                BLOCKED.fetch_add(1, Ordering::Relaxed);
            }
            Ok(())
        }));
        let mut token = Default::default();
        let _ = core.add_WebResourceRequested(&handler, &mut token);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn install(_wv: &wry::WebView) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(items: &[&str]) -> HashSet<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_comments_and_blanks() {
        let v = parse_list("# c\n\nAds.Example.com # tail\n  x.org\nbad line\n");
        assert_eq!(v, vec!["ads.example.com", "x.org"]);
    }

    #[test]
    fn matches_domain_and_subdomains_only() {
        let s = set(&["doubleclick.net", "mc.yandex.ru"]);
        assert!(host_blocked("doubleclick.net", &s));
        assert!(host_blocked("stats.g.doubleclick.net", &s));
        assert!(host_blocked("mc.yandex.ru", &s));
        assert!(!host_blocked("yandex.ru", &s));
        assert!(!host_blocked("notdoubleclick.net", &s));
        assert!(!host_blocked("example.com", &s));
    }

    #[test]
    fn extracts_host() {
        assert_eq!(host_of_url("https://u:p@a.b.com:8080/x?y#z"), "a.b.com");
        assert_eq!(host_of_url("https://a.com"), "a.com");
    }

    #[test]
    fn bundled_list_is_valid() {
        let d = domains();
        assert!(d.len() > 50);
        assert!(d.iter().all(|x| x.contains('.')));
    }
}
