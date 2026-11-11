//! Встроенный рантайм Aether Browser v0.6.1 (Arc / Zen Style):
//! - Мощный вертикальный сайдбар слева (боковые вкладки, быстрые сервисы, омнибокс)
//! - Чистый веб-контент справа без навязчивых оверлеев на поисковую строку YouTube
//! - Сохранение и переключение вкладок через window.name
//! - Кнопка и горячая клавиша Ctrl+B для сворачивания/разворачивания сайдбара
//! - Полный иммунитет к CSP YouTube через Trusted Types
//! - Встроенный AdBlocker для YouTube

pub const START_PAGE_HTML: &str = r###"<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aether — Новая вкладка</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            min-height: 100vh;
            background: 
                radial-gradient(circle at 50% 15%, rgba(147, 51, 234, 0.32) 0%, transparent 50%),
                radial-gradient(circle at 80% 75%, rgba(124, 58, 237, 0.20) 0%, transparent 45%),
                radial-gradient(circle at 20% 80%, rgba(168, 85, 247, 0.15) 0%, transparent 50%),
                #07040d;
            color: #f8fafc;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 40px 20px;
            user-select: none;
            overflow-x: hidden;
            position: relative;
        }

        #aether-clock-wrap {
            text-align: center;
            margin-bottom: 24px;
        }
        #aether-clock {
            font-size: 68px;
            font-weight: 200;
            letter-spacing: 2px;
            color: #ffffff;
            text-shadow: 0 0 25px rgba(192, 132, 252, 0.45);
        }
        #aether-date {
            font-size: 15px;
            font-weight: 400;
            color: #c084fc;
            text-transform: capitalize;
            margin-top: 4px;
        }

        #aether-logo-wrap {
            display: flex;
            align-items: center;
            gap: 12px;
            margin-bottom: 32px;
        }
        #aether-logo-icon {
            font-size: 34px;
            filter: drop-shadow(0 0 15px rgba(168, 85, 247, 0.7));
        }
        #aether-logo-text {
            font-size: 28px;
            font-weight: 700;
            letter-spacing: 6px;
            background: linear-gradient(135deg, #f8fafc 20%, #c084fc 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        #aether-search-box {
            width: 100%;
            max-width: 640px;
            background: rgba(18, 12, 33, 0.85);
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
            border: 1.5px solid rgba(168, 85, 247, 0.45);
            border-radius: 9999px;
            padding: 8px 12px 8px 22px;
            display: flex;
            align-items: center;
            gap: 12px;
            box-shadow: 0 15px 40px rgba(0, 0, 0, 0.6), 0 0 25px rgba(168, 85, 247, 0.2);
            transition: all 0.25s ease;
            margin-bottom: 36px;
        }
        #aether-search-box:focus-within {
            border-color: #c084fc;
            box-shadow: 0 20px 50px rgba(0, 0, 0, 0.7), 0 0 35px rgba(192, 132, 252, 0.4);
            transform: translateY(-2px);
        }
        #aether-search-input {
            flex: 1;
            background: transparent;
            border: none;
            outline: none;
            font-size: 16px;
            color: #ffffff;
        }
        #aether-search-input::placeholder { color: #94a3b8; }
        #aether-search-btn {
            background: linear-gradient(135deg, #9333ea, #7c3aed);
            border: none;
            border-radius: 50%;
            width: 38px;
            height: 38px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #ffffff;
            cursor: pointer;
            transition: transform 0.15s, box-shadow 0.15s;
        }
        #aether-search-btn:hover {
            transform: scale(1.08);
            box-shadow: 0 0 16px rgba(168, 85, 247, 0.6);
        }

        #aether-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 16px;
            width: 100%;
            max-width: 640px;
            margin-bottom: 40px;
        }
        @media (max-width: 600px) {
            #aether-grid { grid-template-columns: repeat(2, 1fr); }
        }
        .aether-card {
            background: rgba(22, 14, 42, 0.65);
            backdrop-filter: blur(12px);
            border: 1px solid rgba(168, 85, 247, 0.25);
            border-radius: 16px;
            padding: 18px 14px;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 10px;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
            text-decoration: none;
            color: #f1f5f9;
        }
        .aether-card:hover {
            background: rgba(30, 18, 58, 0.9);
            border-color: rgba(192, 132, 252, 0.6);
            transform: translateY(-3px);
            box-shadow: 0 12px 30px rgba(0, 0, 0, 0.55), 0 0 20px rgba(168, 85, 247, 0.35);
        }
        .aether-card-icon {
            width: 52px;
            height: 52px;
            border-radius: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
            transition: transform 0.2s ease;
        }
        .aether-card:hover .aether-card-icon { transform: scale(1.08); }
        .aether-card-name { font-size: 13.5px; font-weight: 500; color: #e2e8f0; }

        #aether-hints {
            display: flex;
            flex-wrap: wrap;
            justify-content: center;
            gap: 14px;
            font-size: 12px;
            color: #a78bfa;
            opacity: 0.85;
        }
        .aether-hint-pill {
            background: rgba(168, 85, 247, 0.12);
            border: 1px solid rgba(168, 85, 247, 0.25);
            padding: 4px 10px;
            border-radius: 6px;
        }
        .aether-hint-pill b { color: #f8fafc; }
    </style>
</head>
<body>
    <div id="aether-clock-wrap">
        <div id="aether-clock">--:--</div>
        <div id="aether-date">--</div>
    </div>

    <div id="aether-logo-wrap">
        <span id="aether-logo-icon">🌌</span>
        <span id="aether-logo-text">AETHER</span>
    </div>

    <form id="aether-search-box" onsubmit="handleSearch(event)">
        <input id="aether-search-input" type="text" placeholder="Поиск в Google, запрос 'yt: ...' или ввод адреса..." autocomplete="off" autofocus />
        <button id="aether-search-btn" type="submit">
            <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
        </button>
    </form>

    <div id="aether-grid">
        <a class="aether-card" href="https://www.youtube.com">
            <div class="aether-card-icon" style="background: rgba(239, 68, 68, 0.2); border: 1px solid rgba(239, 68, 68, 0.45);">
                <svg viewBox="0 0 24 24" width="28" height="28" fill="#ff0000"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
            </div>
            <div class="aether-card-name">YouTube</div>
        </a>

        <a class="aether-card" href="https://vk.com">
            <div class="aether-card-icon" style="background: rgba(0, 119, 255, 0.2); border: 1px solid rgba(0, 119, 255, 0.45);">
                <svg viewBox="0 0 24 24" width="28" height="28" fill="#0077ff"><path d="M15.684 0H8.316C2.992 0 0 2.992 0 8.316v7.368C0 21.008 2.992 24 8.316 24h7.368C21.008 24 24 21.008 24 15.684V8.316C24 2.992 21.008 0 15.684 0zm3.692 17.123h-1.744c-.66 0-.862-.525-2.05-1.714-1.036-1.01-1.496-1.144-1.75-1.144-.356 0-.457.102-.457.593v1.642c0 .423-.135.686-1.253.686-1.846 0-3.896-1.12-5.336-3.201-2.17-3.074-2.778-5.386-2.778-5.86 0-.254.102-.491.593-.491h1.745c.44 0 .61.203.78.677.847 2.49 2.27 4.675 2.862 4.675.22 0 .322-.102.322-.66V11.23c-.068-1.185-.695-1.287-.695-1.71 0-.203.17-.406.44-.406h2.744c.373 0 .508.186.508.626v3.354c0 .355.153.474.254.474.22 0 .407-.119.814-.525 1.27-1.423 2.185-3.642 2.185-3.642.119-.254.339-.491.78-.491h1.744c.525 0 .644.27.525.643-.22.983-2.287 3.913-2.388 4.082-.204.322-.288.474 0 .864.203.27 1.05 1.016 1.592 1.643.999 1.152 1.762 2.117 1.965 2.778.084.271-.102.66-.542.66z"/></svg>
            </div>
            <div class="aether-card-name">ВКонтакте</div>
        </a>

        <a class="aether-card" href="https://web.telegram.org">
            <div class="aether-card-icon" style="background: rgba(41, 182, 246, 0.2); border: 1px solid rgba(41, 182, 246, 0.45);">
                <svg viewBox="0 0 24 24" width="28" height="28" fill="#29b6f6"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm5.894 8.221l-1.97 9.28c-.145.658-.537.818-1.084.508l-3-2.21-1.446 1.394c-.16.16-.295.295-.605.295l.213-3.053 5.56-5.023c.242-.213-.054-.333-.373-.121l-6.871 4.326-2.962-.924c-.643-.204-.657-.643.136-.953l11.57-4.458c.538-.196 1.006.128.832.935z"/></svg>
            </div>
            <div class="aether-card-name">Telegram</div>
        </a>

        <a class="aether-card" href="https://github.com">
            <div class="aether-card-icon" style="background: rgba(255, 255, 255, 0.12); border: 1px solid rgba(255, 255, 255, 0.3);">
                <svg viewBox="0 0 24 24" width="28" height="28" fill="#ffffff"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/></svg>
            </div>
            <div class="aether-card-name">GitHub</div>
        </a>
    </div>

    <div id="aether-hints">
        <span class="aether-hint-pill"><b>Ctrl + T</b> Новая вкладка</span>
        <span class="aether-hint-pill"><b>Ctrl + W</b> Закрыть вкладку</span>
        <span class="aether-hint-pill"><b>Ctrl + B</b> Скрыть / открыть сайдбар</span>
        <span class="aether-hint-pill"><b>Ctrl + M</b> Только аудио</span>
        <span class="aether-hint-pill"><b>Ctrl + D</b> Скачать видео</span>
    </div>

    <script>
        function updateClock() {
            const now = new Date();
            const hours = String(now.getHours()).padStart(2, '0');
            const minutes = String(now.getMinutes()).padStart(2, '0');
            document.getElementById('aether-clock').textContent = hours + ':' + minutes;

            const options = { weekday: 'long', day: 'numeric', month: 'long' };
            document.getElementById('aether-date').textContent = now.toLocaleDateString('ru-RU', options);
        }
        setInterval(updateClock, 1000);
        updateClock();

        function handleSearch(e) {
            e.preventDefault();
            const q = document.getElementById('aether-search-input').value.trim();
            if (!q) return;

            if (q.toLowerCase().startsWith("yt:") || q.toLowerCase().startsWith("ютуб:")) {
                const query = q.replace(/^(yt:|ютуб:)/i, "").trim();
                window.location.href = "https://www.youtube.com/results?search_query=" + encodeURIComponent(query);
                return;
            }
            if (q.toLowerCase().startsWith("vk:") || q.toLowerCase().startsWith("вк:")) {
                const query = q.replace(/^(vk:|вк:)/i, "").trim();
                window.location.href = "https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D=" + encodeURIComponent(query);
                return;
            }
            if (q.toLowerCase().startsWith("gh:")) {
                const query = q.replace(/^gh:/i, "").trim();
                window.location.href = "https://github.com/search?q=" + encodeURIComponent(query);
                return;
            }

            const isUrl = q.startsWith("http://") || q.startsWith("https://") || (q.includes(".") && !q.includes(" ") && !q.endsWith("."));
            if (isUrl) {
                window.location.href = q.startsWith("http") ? q : "https://" + q;
            } else {
                window.location.href = "https://www.google.com/search?q=" + encodeURIComponent(q);
            }
        }
    </script>
</body>
</html>"###;

pub fn get_core_script(start_url: &str) -> String {
    format!(
        "window.__AETHER_GLOBAL_START_URL__ = {:?};\n{}",
        start_url,
        AETHER_CORE_SCRIPT
    )
}

pub const AETHER_CORE_SCRIPT: &str = r###"
(() => {
    try {
        if (window.top !== window.self) return;
    } catch (_) {
        return;
    }

    if (window.__AETHER_CORE_ATTACHED__) return;
    window.__AETHER_CORE_ATTACHED__ = true;

    // 1. TRUSTED TYPES CSP BYPASS (YouTube, Google, GitHub)
    let aetherPolicy = {
        createHTML: (s) => s,
        createScript: (s) => s,
        createScriptURL: (s) => s
    };

    try {
        if (window.trustedTypes && typeof window.trustedTypes.createPolicy === 'function') {
            try {
                window.trustedTypes.createPolicy('default', {
                    createHTML: (s) => s,
                    createScript: (s) => s,
                    createScriptURL: (s) => s
                });
            } catch (_) {}
            try {
                aetherPolicy = window.trustedTypes.createPolicy('aether-trusted-policy', {
                    createHTML: (s) => s,
                    createScript: (s) => s,
                    createScriptURL: (s) => s
                });
            } catch (_) {}
        }
    } catch (_) {}

    function safeSetHTML(element, htmlString) {
        if (!element) return;
        try {
            if (aetherPolicy && typeof aetherPolicy.createHTML === 'function') {
                element.innerHTML = aetherPolicy.createHTML(htmlString);
                return;
            }
        } catch (_) {}
        try {
            element.innerHTML = htmlString;
        } catch (_) {}
    }

    function getStartPageUrl() {
        if (window.__AETHER_GLOBAL_START_URL__) {
            return window.__AETHER_GLOBAL_START_URL__;
        }
        const state = getTabsState();
        const found = state.tabs.find(t => t.url && t.url.includes("start.html"));
        if (found) return found.url;
        return "https://www.google.com";
    }

    // 2. УПРАВЛЕНИЕ СОСТОЯНИЕМ ВКЛАДОК (Менеджер вкладок в window.name)
    function getTabsState() {
        try {
            if (window.name && window.name.startsWith("__AETHER_STATE__=")) {
                const json = window.name.substring(17);
                const parsed = JSON.parse(json);
                if (parsed && Array.isArray(parsed.tabs) && parsed.tabs.length > 0) {
                    return parsed;
                }
            }
        } catch (_) {}

        const startUrl = window.location.href;
        const initialTitle = startUrl.includes("start.html") ? "Старт" : (document.title || "Вкладка");
        return {
            activeTabId: 1,
            nextTabId: 2,
            sidebarCollapsed: false,
            tabs: [
                { id: 1, title: initialTitle, url: startUrl }
            ]
        };
    }

    function saveTabsState(state) {
        try {
            window.name = "__AETHER_STATE__=" + JSON.stringify(state);
        } catch (_) {}
    }

    function updateCurrentTabInfo() {
        const state = getTabsState();
        let tab = state.tabs.find(t => t.id === state.activeTabId);
        const curUrl = window.location.href;
        const curTitle = curUrl.includes("start.html") ? "Старт" : (document.title || (location.hostname.replace("www.", "") || "Вкладка"));

        if (!tab) {
            tab = { id: state.activeTabId, title: curTitle, url: curUrl };
            state.tabs.push(tab);
        } else {
            tab.url = curUrl;
            if (curTitle && curTitle !== "YouTube" && curTitle !== "Вкладка") {
                tab.title = curTitle;
            } else if (!tab.title || tab.title === "Новая вкладка") {
                tab.title = curTitle;
            }
        }
        saveTabsState(state);
    }

    updateCurrentTabInfo();

    // 3. СТИЛИ И ИНЪЕКЦИЯ САЙДБАРА ARC
    const SIDEBAR_CSS = `
        :root {
            --aether-sidebar-w: 240px;
        }
        html {
            margin-left: 240px !important;
            width: calc(100vw - 240px) !important;
            max-width: calc(100vw - 240px) !important;
            box-sizing: border-box !important;
            transition: margin-left 0.22s cubic-bezier(0.16, 1, 0.3, 1), width 0.22s cubic-bezier(0.16, 1, 0.3, 1) !important;
        }
        body {
            width: 100% !important;
            max-width: 100% !important;
        }
        #masthead-container, #guide, #header, header[role="banner"] {
            left: 240px !important;
            width: calc(100vw - 240px) !important;
            transition: left 0.22s cubic-bezier(0.16, 1, 0.3, 1), width 0.22s cubic-bezier(0.16, 1, 0.3, 1) !important;
        }

        html.aether-collapsed {
            margin-left: 0 !important;
            width: 100vw !important;
            max-width: 100vw !important;
        }
        html.aether-collapsed #masthead-container, html.aether-collapsed #guide, html.aether-collapsed #header {
            left: 0 !important;
            width: 100vw !important;
        }

        #aether-arc-sidebar {
            position: fixed !important;
            top: 0 !important;
            left: 0 !important;
            width: 240px !important;
            height: 100vh !important;
            background: linear-gradient(180deg, #090514 0%, #0d071a 50%, #070310 100%) !important;
            border-right: 1px solid rgba(168, 85, 247, 0.25) !important;
            box-shadow: 4px 0 24px rgba(0, 0, 0, 0.65) !important;
            z-index: 2147483640 !important;
            display: flex !important;
            flex-direction: column !important;
            padding: 12px 10px 10px !important;
            box-sizing: border-box !important;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
            user-select: none !important;
            color: #f8fafc !important;
            transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1) !important;
        }
        #aether-arc-sidebar.collapsed {
            transform: translateX(-240px) !important;
        }

        /* Expand floating button */
        #aether-expand-pill {
            position: fixed !important;
            top: 10px !important;
            left: 10px !important;
            width: 32px !important;
            height: 32px !important;
            border-radius: 9px !important;
            background: rgba(18, 12, 33, 0.9) !important;
            border: 1px solid rgba(168, 85, 247, 0.45) !important;
            color: #c084fc !important;
            display: none !important;
            align-items: center !important;
            justify-content: center !important;
            cursor: pointer !important;
            z-index: 2147483647 !important;
            box-shadow: 0 4px 14px rgba(0, 0, 0, 0.6), 0 0 10px rgba(168, 85, 247, 0.35) !important;
            transition: all 0.18s ease !important;
        }
        #aether-expand-pill:hover {
            background: rgba(30, 18, 58, 0.95) !important;
            color: #ffffff !important;
            border-color: #c084fc !important;
            transform: scale(1.08) !important;
        }
        html.aether-collapsed #aether-expand-pill {
            display: flex !important;
        }

        /* Sidebar Elements */
        .aether-sb-header {
            display: flex !important;
            align-items: center !important;
            justify-content: space-between !important;
            padding: 2px 4px 10px !important;
        }
        .aether-sb-brand {
            display: flex !important;
            align-items: center !important;
            gap: 8px !important;
            font-size: 14.5px !important;
            font-weight: 700 !important;
            letter-spacing: 2px !important;
            background: linear-gradient(135deg, #f8fafc 20%, #c084fc 100%) !important;
            -webkit-background-clip: text !important;
            -webkit-text-fill-color: transparent !important;
        }
        .aether-sb-brand-icon {
            font-size: 16px !important;
            -webkit-text-fill-color: initial !important;
        }
        .aether-sb-btn-box {
            display: flex !important;
            align-items: center !important;
            gap: 4px !important;
        }
        .aether-sb-tool-btn {
            background: rgba(255, 255, 255, 0.05) !important;
            border: 1px solid rgba(168, 85, 247, 0.25) !important;
            border-radius: 6px !important;
            width: 25px !important;
            height: 25px !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            color: #c084fc !important;
            cursor: pointer !important;
            transition: all 0.15s ease !important;
        }
        .aether-sb-tool-btn:hover {
            background: rgba(168, 85, 247, 0.25) !important;
            color: #ffffff !important;
            box-shadow: 0 0 10px rgba(168, 85, 247, 0.4) !important;
        }

        /* Omnibox */
        .aether-sb-nav-row {
            display: flex !important;
            align-items: center !important;
            gap: 4px !important;
            margin-bottom: 8px !important;
        }
        .aether-sb-nav-btn {
            width: 24px !important;
            height: 24px !important;
            border-radius: 6px !important;
            background: rgba(255, 255, 255, 0.04) !important;
            border: 1px solid rgba(255, 255, 255, 0.06) !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            color: #94a3b8 !important;
            cursor: pointer !important;
            transition: all 0.15s ease !important;
        }
        .aether-sb-nav-btn:hover {
            background: rgba(168, 85, 247, 0.2) !important;
            color: #ffffff !important;
            border-color: rgba(168, 85, 247, 0.35) !important;
        }
        .aether-sb-omnibox-wrap {
            flex: 1 !important;
            position: relative !important;
            display: flex !important;
            align-items: center !important;
        }
        .aether-sb-omnibox-input {
            width: 100% !important;
            background: rgba(20, 12, 38, 0.85) !important;
            border: 1px solid rgba(168, 85, 247, 0.35) !important;
            border-radius: 7px !important;
            padding: 4px 6px 4px 22px !important;
            font-size: 11.5px !important;
            color: #ffffff !important;
            outline: none !important;
            transition: all 0.2s ease !important;
            box-sizing: border-box !important;
        }
        .aether-sb-omnibox-input:focus {
            border-color: #c084fc !important;
            box-shadow: 0 0 10px rgba(168, 85, 247, 0.35) !important;
            background: rgba(28, 16, 52, 0.95) !important;
        }
        .aether-sb-omnibox-icon {
            position: absolute !important;
            left: 6px !important;
            font-size: 10px !important;
            color: #a855f7 !important;
            pointer-events: none !important;
        }

        /* Favorites / Pinned Dock */
        .aether-sb-favorites {
            display: grid !important;
            grid-template-columns: repeat(4, 1fr) !important;
            gap: 5px !important;
            margin-bottom: 12px !important;
        }
        .aether-sb-fav-tile {
            height: 38px !important;
            background: rgba(22, 14, 42, 0.65) !important;
            border: 1px solid rgba(168, 85, 247, 0.22) !important;
            border-radius: 8px !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            cursor: pointer !important;
            transition: all 0.16s cubic-bezier(0.16, 1, 0.3, 1) !important;
            position: relative !important;
        }
        .aether-sb-fav-tile:hover {
            background: rgba(36, 20, 68, 0.95) !important;
            border-color: rgba(192, 132, 252, 0.6) !important;
            transform: translateY(-2px) !important;
            box-shadow: 0 5px 14px rgba(0, 0, 0, 0.4), 0 0 10px rgba(168, 85, 247, 0.3) !important;
        }
        .aether-sb-active-dot {
            position: absolute !important;
            bottom: 3px !important;
            width: 4px !important;
            height: 4px !important;
            border-radius: 50% !important;
            background: #c084fc !important;
            box-shadow: 0 0 5px #c084fc !important;
            display: none !important;
        }

        /* Tabs Section */
        .aether-sb-tabs-header {
            display: flex !important;
            align-items: center !important;
            justify-content: space-between !important;
            padding: 0 4px 6px !important;
            font-size: 10.5px !important;
            font-weight: 600 !important;
            color: #94a3b8 !important;
            letter-spacing: 0.8px !important;
            text-transform: uppercase !important;
        }
        .aether-sb-new-tab-btn {
            background: rgba(147, 51, 234, 0.25) !important;
            border: 1px solid rgba(168, 85, 247, 0.35) !important;
            border-radius: 5px !important;
            width: 19px !important;
            height: 19px !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            color: #d8b4fe !important;
            font-size: 12px !important;
            cursor: pointer !important;
            transition: all 0.15s ease !important;
        }
        .aether-sb-new-tab-btn:hover {
            background: rgba(168, 85, 247, 0.5) !important;
            color: #ffffff !important;
            box-shadow: 0 0 8px rgba(168, 85, 247, 0.5) !important;
            transform: scale(1.08) !important;
        }

        .aether-sb-tabs-list {
            flex: 1 !important;
            overflow-y: auto !important;
            overflow-x: hidden !important;
            display: flex !important;
            flex-direction: column !important;
            gap: 4px !important;
            padding: 2px !important;
        }
        .aether-sb-tabs-list::-webkit-scrollbar { width: 3px !important; }
        .aether-sb-tabs-list::-webkit-scrollbar-thumb { background: rgba(168, 85, 247, 0.3) !important; border-radius: 3px !important; }

        .aether-sb-tab-item {
            display: flex !important;
            align-items: center !important;
            gap: 7px !important;
            padding: 6px 8px !important;
            border-radius: 7px !important;
            font-size: 12px !important;
            color: #cbd5e1 !important;
            cursor: pointer !important;
            border: 1px solid transparent !important;
            transition: all 0.15s ease !important;
        }
        .aether-sb-tab-item:hover {
            background: rgba(255, 255, 255, 0.05) !important;
            color: #ffffff !important;
        }
        .aether-sb-tab-item.active {
            background: linear-gradient(135deg, rgba(147, 51, 234, 0.35), rgba(124, 58, 237, 0.2)) !important;
            border-color: rgba(192, 132, 252, 0.55) !important;
            color: #ffffff !important;
            font-weight: 500 !important;
            box-shadow: 0 3px 12px rgba(0, 0, 0, 0.3), 0 0 8px rgba(168, 85, 247, 0.2) !important;
        }
        .aether-sb-tab-badge {
            width: 15px !important;
            height: 15px !important;
            flex-shrink: 0 !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            font-size: 11px !important;
        }
        .aether-sb-tab-title {
            flex: 1 !important;
            white-space: nowrap !important;
            overflow: hidden !important;
            text-overflow: ellipsis !important;
        }
        .aether-sb-tab-close {
            opacity: 0 !important;
            font-size: 11px !important;
            color: #94a3b8 !important;
            padding: 1px 4px !important;
            border-radius: 3px !important;
            transition: all 0.12s !important;
        }
        .aether-sb-tab-item:hover .aether-sb-tab-close {
            opacity: 0.75 !important;
        }
        .aether-sb-tab-close:hover {
            opacity: 1 !important;
            color: #fca5a5 !important;
            background: rgba(239, 68, 68, 0.25) !important;
        }

        /* Bottom Utilities Dock */
        .aether-sb-dock {
            margin-top: auto !important;
            padding-top: 8px !important;
            border-top: 1px solid rgba(255, 255, 255, 0.08) !important;
            display: flex !important;
            align-items: center !important;
            justify-content: space-around !important;
        }
        .aether-sb-dock-btn {
            width: 30px !important;
            height: 30px !important;
            border-radius: 7px !important;
            background: rgba(255, 255, 255, 0.04) !important;
            border: 1px solid rgba(255, 255, 255, 0.06) !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            color: #94a3b8 !important;
            cursor: pointer !important;
            transition: all 0.15s ease !important;
        }
        .aether-sb-dock-btn:hover {
            background: rgba(168, 85, 247, 0.25) !important;
            color: #d8b4fe !important;
            border-color: rgba(168, 85, 247, 0.45) !important;
            box-shadow: 0 0 10px rgba(168, 85, 247, 0.3) !important;
            transform: translateY(-1px) !important;
        }
    `;

    function injectStyle() {
        if (document.getElementById('aether-arc-style')) return;
        const style = document.createElement('style');
        style.id = 'aether-arc-style';
        style.textContent = SIDEBAR_CSS;
        (document.head || document.documentElement).appendChild(style);
    }

    function getTabIcon(url, title) {
        const u = (url || '').toLowerCase();
        if (u.includes('youtube.com')) return '<span style="color:#ef4444; font-size:12px;">▶</span>';
        if (u.includes('vk.com')) return '<span style="color:#0077ff; font-weight:bold; font-size:10px;">VK</span>';
        if (u.includes('telegram.org')) return '<span style="color:#38bdf8; font-size:11px;">✈</span>';
        if (u.includes('github.com')) return '<span style="color:#f8fafc; font-size:11px;">🐙</span>';
        if (u.includes('start.html')) return '<span style="color:#c084fc; font-size:11px;">🌌</span>';
        return '<span style="color:#94a3b8; font-size:11px;">🌐</span>';
    }

    // 4. ПОСТРОЕНИЕ САЙДБАРА
    function renderSidebar() {
        injectStyle();

        let sidebar = document.getElementById('aether-arc-sidebar');
        if (!sidebar) {
            sidebar = document.createElement('div');
            sidebar.id = 'aether-arc-sidebar';
            document.documentElement.appendChild(sidebar);
        }

        let expandPill = document.getElementById('aether-expand-pill');
        if (!expandPill) {
            expandPill = document.createElement('div');
            expandPill.id = 'aether-expand-pill';
            expandPill.title = 'Развернуть сайдбар Aether (Ctrl + B)';
            expandPill.innerHTML = '<svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>';
            expandPill.onclick = () => toggleSidebar(false);
            document.documentElement.appendChild(expandPill);
        }

        const state = getTabsState();
        if (state.sidebarCollapsed) {
            sidebar.classList.add('collapsed');
            document.documentElement.classList.add('aether-collapsed');
        } else {
            sidebar.classList.remove('collapsed');
            document.documentElement.classList.remove('aether-collapsed');
        }

        // Внутренняя разметка
        safeSetHTML(sidebar, `
            <div class="aether-sb-header">
                <div class="aether-sb-brand">
                    <span class="aether-sb-brand-icon">🌌</span>
                    <span>AETHER</span>
                </div>
                <div class="aether-sb-btn-box">
                    <button id="aether-sb-toggle-btn" class="aether-sb-tool-btn" title="Свернуть сайдбар (Ctrl + B)">
                        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
                    </button>
                </div>
            </div>

            <div class="aether-sb-nav-row">
                <button id="aether-sb-back" class="aether-sb-nav-btn" title="Назад (Alt + ←)">
                    <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
                </button>
                <button id="aether-sb-forward" class="aether-sb-nav-btn" title="Вперед (Alt + →)">
                    <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                </button>
                <button id="aether-sb-reload" class="aether-sb-nav-btn" title="Обновить (F5)">
                    <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/></svg>
                </button>
                <div class="aether-sb-omnibox-wrap">
                    <span class="aether-sb-omnibox-icon">🔍</span>
                    <input id="aether-sb-omnibox" class="aether-sb-omnibox-input" type="text" placeholder="Поиск или URL..." autocomplete="off" />
                </div>
            </div>

            <div class="aether-sb-favorites">
                <div class="aether-sb-fav-tile" title="YouTube" id="fav-yt">
                    <svg viewBox="0 0 24 24" width="18" height="18" fill="#ff0000"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
                    <div id="dot-yt" class="aether-sb-active-dot"></div>
                </div>
                <div class="aether-sb-fav-tile" title="ВКонтакте" id="fav-vk">
                    <svg viewBox="0 0 24 24" width="18" height="18" fill="#0077ff"><path d="M15.684 0H8.316C2.992 0 0 2.992 0 8.316v7.368C0 21.008 2.992 24 8.316 24h7.368C21.008 24 24 21.008 24 15.684V8.316C24 2.992 21.008 0 15.684 0zm3.692 17.123h-1.744c-.66 0-.862-.525-2.05-1.714-1.036-1.01-1.496-1.144-1.75-1.144-.356 0-.457.102-.457.593v1.642c0 .423-.135.686-1.253.686-1.846 0-3.896-1.12-5.336-3.201-2.17-3.074-2.778-5.386-2.778-5.86 0-.254.102-.491.593-.491h1.745c.44 0 .61.203.78.677.847 2.49 2.27 4.675 2.862 4.675.22 0 .322-.102.322-.66V11.23c-.068-1.185-.695-1.287-.695-1.71 0-.203.17-.406.44-.406h2.744c.373 0 .508.186.508.626v3.354c0 .355.153.474.254.474.22 0 .407-.119.814-.525 1.27-1.423 2.185-3.642 2.185-3.642.119-.254.339-.491.78-.491h1.744c.525 0 .644.27.525.643-.22.983-2.287 3.913-2.388 4.082-.204.322-.288.474 0 .864.203.27 1.05 1.016 1.592 1.643.999 1.152 1.762 2.117 1.965 2.778.084.271-.102.66-.542.66z"/></svg>
                    <div id="dot-vk" class="aether-sb-active-dot"></div>
                </div>
                <div class="aether-sb-fav-tile" title="Telegram" id="fav-tg">
                    <svg viewBox="0 0 24 24" width="18" height="18" fill="#29b6f6"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm5.894 8.221l-1.97 9.28c-.145.658-.537.818-1.084.508l-3-2.21-1.446 1.394c-.16.16-.295.295-.605.295l.213-3.053 5.56-5.023c.242-.213-.054-.333-.373-.121l-6.871 4.326-2.962-.924c-.643-.204-.657-.643.136-.953l11.57-4.458c.538-.196 1.006.128.832.935z"/></svg>
                    <div id="dot-tg" class="aether-sb-active-dot"></div>
                </div>
                <div class="aether-sb-fav-tile" title="GitHub" id="fav-gh">
                    <svg viewBox="0 0 24 24" width="18" height="18" fill="#ffffff"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/></svg>
                    <div id="dot-gh" class="aether-sb-active-dot"></div>
                </div>
            </div>

            <div class="aether-sb-tabs-header">
                <span id="aether-sb-tabs-counter">Вкладки (${state.tabs.length})</span>
                <button id="aether-sb-new-tab" class="aether-sb-new-tab-btn" title="Новая вкладка (Ctrl + T)">+</button>
            </div>

            <div id="aether-sb-tabs-list" class="aether-sb-tabs-list"></div>

            <div class="aether-sb-dock">
                <button id="aether-sb-audio" class="aether-sb-dock-btn" title="Только аудио (Ctrl + M) — экономия ресурсов">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 18v-6a9 9 0 0 1 18 0v6"/><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/></svg>
                </button>
                <button id="aether-sb-pip" class="aether-sb-dock-btn" title="Картинка в картинке / PiP (Ctrl + P)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><rect x="2" y="3" width="20" height="14" rx="2"/><rect x="12" y="9" width="8" height="6" rx="1"/></svg>
                </button>
                <button id="aether-sb-download" class="aether-sb-dock-btn" title="Скачать видео через yt-dlp (Ctrl + D)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                </button>
                <button id="aether-sb-home" class="aether-sb-dock-btn" title="Стартовая страница Aether">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
                </button>
            </div>
        `);

        // Заполняем вкладки
        const tabsContainer = document.getElementById('aether-sb-tabs-list');
        let hasYt = false, hasVk = false, hasTg = false, hasGh = false;

        state.tabs.forEach(tab => {
            const u = (tab.url || '').toLowerCase();
            if (u.includes('youtube.com')) hasYt = true;
            if (u.includes('vk.com')) hasVk = true;
            if (u.includes('telegram.org')) hasTg = true;
            if (u.includes('github.com')) hasGh = true;

            const item = document.createElement('div');
            item.className = 'aether-sb-tab-item' + (tab.id === state.activeTabId ? ' active' : '');

            const badge = document.createElement('div');
            badge.className = 'aether-sb-tab-badge';
            badge.innerHTML = getTabIcon(tab.url, tab.title);

            const title = document.createElement('div');
            title.className = 'aether-sb-tab-title';
            title.textContent = tab.title || 'Новая вкладка';

            const closeBtn = document.createElement('div');
            closeBtn.className = 'aether-sb-tab-close';
            closeBtn.textContent = '✕';
            closeBtn.onclick = (e) => {
                e.stopPropagation();
                closeTab(tab.id);
            };

            item.onclick = () => {
                if (tab.id !== state.activeTabId) {
                    switchTab(tab.id);
                }
            };

            item.appendChild(badge);
            item.appendChild(title);
            item.appendChild(closeBtn);
            tabsContainer.appendChild(item);
        });

        // Индикаторы активных избранных
        if (document.getElementById('dot-yt')) document.getElementById('dot-yt').style.display = hasYt ? 'block' : 'none';
        if (document.getElementById('dot-vk')) document.getElementById('dot-vk').style.display = hasVk ? 'block' : 'none';
        if (document.getElementById('dot-tg')) document.getElementById('dot-tg').style.display = hasTg ? 'block' : 'none';
        if (document.getElementById('dot-gh')) document.getElementById('dot-gh').style.display = hasGh ? 'block' : 'none';

        // Omnibox значение
        const omnibox = document.getElementById('aether-sb-omnibox');
        if (omnibox) {
            const isStart = window.location.href.includes("start.html");
            omnibox.value = isStart ? "" : window.location.href;
            omnibox.onkeydown = (e) => {
                if (e.key === 'Enter') {
                    handleNavigate(omnibox.value.trim());
                }
            };
        }

        // События кнопок навигации
        document.getElementById('aether-sb-back').onclick = () => window.history.back();
        document.getElementById('aether-sb-forward').onclick = () => window.history.forward();
        document.getElementById('aether-sb-reload').onclick = () => window.location.reload();
        document.getElementById('aether-sb-toggle-btn').onclick = () => toggleSidebar(true);
        document.getElementById('aether-sb-new-tab').onclick = () => createNewTab();
        document.getElementById('aether-sb-home').onclick = () => navigateToUrl(getStartPageUrl());

        // Быстрые сервисы
        document.getElementById('fav-yt').onclick = () => openService('https://www.youtube.com');
        document.getElementById('fav-vk').onclick = () => openService('https://vk.com');
        document.getElementById('fav-tg').onclick = () => openService('https://web.telegram.org');
        document.getElementById('fav-gh').onclick = () => openService('https://github.com');

        // Медиа-инструменты
        document.getElementById('aether-sb-audio').onclick = () => toggleAudioMode();
        document.getElementById('aether-sb-pip').onclick = () => togglePiP();
        document.getElementById('aether-sb-download').onclick = () => copyYtDlp();
    }

    // 5. ДЕЙСТВИЯ С ВКЛАДКАМИ И НАВИГАЦИЕЙ
    function toggleSidebar(collapsed) {
        const state = getTabsState();
        state.sidebarCollapsed = (typeof collapsed === 'boolean') ? collapsed : !state.sidebarCollapsed;
        saveTabsState(state);
        renderSidebar();
    }

    function createNewTab(url) {
        const state = getTabsState();
        const targetUrl = url || getStartPageUrl();
        const newId = state.nextTabId;
        state.nextTabId++;
        const title = targetUrl.includes("start.html") ? "Старт" : "Новая вкладка";
        state.tabs.push({ id: newId, title, url: targetUrl });
        state.activeTabId = newId;
        saveTabsState(state);
        window.location.href = targetUrl;
    }

    function switchTab(id) {
        const state = getTabsState();
        const tab = state.tabs.find(t => t.id === id);
        if (tab && tab.url) {
            state.activeTabId = id;
            saveTabsState(state);
            window.location.href = tab.url;
        }
    }

    function closeTab(id) {
        const state = getTabsState();
        const idx = state.tabs.findIndex(t => t.id === id);
        if (idx !== -1) {
            if (state.tabs.length <= 1) {
                // Если осталась одна вкладка, сбрасываем ее на Старт
                state.tabs[0].url = getStartPageUrl();
                state.tabs[0].title = "Старт";
                saveTabsState(state);
                window.location.href = getStartPageUrl();
                return;
            }

            const wasActive = state.activeTabId === id;
            state.tabs.splice(idx, 1);

            if (wasActive) {
                const nextTab = state.tabs[Math.max(0, idx - 1)] || state.tabs[0];
                state.activeTabId = nextTab.id;
                saveTabsState(state);
                window.location.href = nextTab.url;
            } else {
                saveTabsState(state);
                renderSidebar();
            }
        }
    }

    function openService(url) {
        const state = getTabsState();
        const domain = url.replace("https://", "").replace("http://", "").replace("www.", "").split('/')[0];
        const existing = state.tabs.find(t => t.url && t.url.includes(domain));
        if (existing) {
            switchTab(existing.id);
        } else {
            // Если на странице старт, переходим в текущей, иначе создаем новую
            if (window.location.href.includes("start.html")) {
                navigateToUrl(url);
            } else {
                createNewTab(url);
            }
        }
    }

    function navigateToUrl(url) {
        const state = getTabsState();
        let tab = state.tabs.find(t => t.id === state.activeTabId);
        if (tab) {
            tab.url = url;
            tab.title = url.includes("start.html") ? "Старт" : "Загрузка...";
        }
        saveTabsState(state);
        window.location.href = url;
    }

    function handleNavigate(input) {
        if (!input) return;
        let targetUrl = "";
        const lower = input.toLowerCase();

        if (lower.startsWith("yt:") || lower.startsWith("ютуб:")) {
            const q = input.replace(/^(yt:|ютуб:)/i, "").trim();
            targetUrl = "https://www.youtube.com/results?search_query=" + encodeURIComponent(q);
        } else if (lower.startsWith("vk:") || lower.startsWith("вк:")) {
            const q = input.replace(/^(vk:|вк:)/i, "").trim();
            targetUrl = "https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D=" + encodeURIComponent(q);
        } else if (lower.startsWith("gh:")) {
            const q = input.replace(/^gh:/i, "").trim();
            targetUrl = "https://github.com/search?q=" + encodeURIComponent(q);
        } else if (input.startsWith("http://") || input.startsWith("https://")) {
            targetUrl = input;
        } else if (input.includes(".") && !input.includes(" ") && !input.endsWith(".")) {
            targetUrl = "https://" + input;
        } else {
            targetUrl = "https://www.google.com/search?q=" + encodeURIComponent(input);
        }

        navigateToUrl(targetUrl);
    }

    // 6. МЕДИА ФУНКЦИИ
    function toggleAudioMode() {
        const video = document.querySelector('video');
        if (video) {
            const isAudioOnly = video.style.opacity === '0.001';
            video.style.opacity = isAudioOnly ? '1' : '0.001';
            video.style.transform = isAudioOnly ? 'scale(1)' : 'scale(0.01)';
        }
    }

    function togglePiP() {
        const video = document.querySelector('video');
        if (video) {
            if (document.pictureInPictureElement) {
                document.exitPictureInPicture().catch(() => {});
            } else if (document.pictureInPictureEnabled) {
                video.requestPictureInPicture().catch(() => {});
            }
        }
    }

    function copyYtDlp() {
        const url = window.location.href;
        const cmd = `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best" "${url}"`;
        try {
            navigator.clipboard.writeText(cmd);
            alert("Команда для скачивания скопирована в буфер обмена!\n\n" + cmd);
        } catch (_) {
            prompt("Скопируйте команду:", cmd);
        }
    }

    // 7. YOUTUBE АВТО-ПРОПУСК РЕКЛАМЫ
    if (location.hostname.includes("youtube.com")) {
        setInterval(() => {
            try {
                const skipBtn = document.querySelector('.ytp-skip-ad-button, .ytp-ad-skip-button, .ytp-ad-skip-button-modern');
                if (skipBtn) skipBtn.click();

                const video = document.querySelector('video');
                const adShowing = document.querySelector('.ad-showing, .ad-interrupting');
                if (video && adShowing) {
                    video.playbackRate = 16.0;
                    video.muted = true;
                    if (isFinite(video.duration) && video.duration > 0) video.currentTime = video.duration;
                }

                const adOverlays = document.querySelectorAll('ytd-ad-slot-renderer, #player-ads, ytd-banner-promo-renderer, ytd-in-feed-ad-layout-renderer');
                adOverlays.forEach(el => el.style.display = 'none');
            } catch (_) {}
        }, 300);
    }

    // 8. ГОРЯЧИЕ КЛАВИШИ БРАУЗЕРА
    window.addEventListener('keydown', (e) => {
        const ctrl = e.ctrlKey || e.metaKey;
        if (ctrl && (e.key === 't' || e.key === 'T' || e.key === 'е' || e.key === 'Е')) {
            e.preventDefault();
            createNewTab();
        } else if (ctrl && (e.key === 'w' || e.key === 'W' || e.key === 'ц' || e.key === 'Ц')) {
            e.preventDefault();
            const state = getTabsState();
            closeTab(state.activeTabId);
        } else if (ctrl && (e.key === 'b' || e.key === 'B' || e.key === 'и' || e.key === 'И')) {
            e.preventDefault();
            toggleSidebar();
        } else if (ctrl && (e.key === 'l' || e.key === 'L' || e.key === 'д' || e.key === 'Д') || e.key === 'F6') {
            e.preventDefault();
            const omnibox = document.getElementById('aether-sb-omnibox');
            if (omnibox) {
                toggleSidebar(false);
                omnibox.focus();
                omnibox.select();
            }
        } else if (ctrl && (e.key === 'm' || e.key === 'M' || e.key === 'ь' || e.key === 'Ь')) {
            e.preventDefault();
            toggleAudioMode();
        } else if (ctrl && (e.key === 'd' || e.key === 'D' || e.key === 'в' || e.key === 'В')) {
            e.preventDefault();
            copyYtDlp();
        } else if (ctrl && (e.key === 'p' || e.key === 'P' || e.key === 'з' || e.key === 'З')) {
            e.preventDefault();
            togglePiP();
        }
    });

    // 9. СЛЕЖЕНИЕ ЗА ИЗМЕНЕНИЕМ ЗАГОЛОВКА
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', renderSidebar);
    } else {
        renderSidebar();
    }
    window.addEventListener('load', renderSidebar);
    window.addEventListener('yt-navigate-finish', () => {
        updateCurrentTabInfo();
        renderSidebar();
    });
    window.addEventListener('popstate', () => {
        updateCurrentTabInfo();
        renderSidebar();
    });

    let prevTitle = "";
    setInterval(() => {
        if (document.title !== prevTitle) {
            prevTitle = document.title;
            updateCurrentTabInfo();
            renderSidebar();
        }
    }, 500);
})();
"###;
