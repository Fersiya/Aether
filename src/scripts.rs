//! Встроенный рантайм Aether Browser v0.5.6:
//! - Вариант Б: Компактный, сбалансированный Dynamic Island без визуального дублирования
//! - Вкладки СПРАВА с сохранением состояния через window.name (работает на YouTube и всех сайтах!)
//! - Векторная иконка Домой (SVG) вместо пиксельного эмодзи
//! - Полный иммунитет к CSP YouTube через inline cssText
//! - Окно настроек (Settings) и официальные SVG-логотипы

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
            padding: 60px 20px 40px;
            user-select: none;
            overflow-x: hidden;
            position: relative;
        }

        #aether-home-settings-btn {
            position: absolute;
            top: 24px;
            right: 28px;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(168, 85, 247, 0.3);
            border-radius: 50%;
            width: 40px;
            height: 40px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #c084fc;
            cursor: pointer;
            transition: all 0.2s ease;
        }
        #aether-home-settings-btn:hover {
            background: rgba(168, 85, 247, 0.3);
            color: #ffffff;
            transform: rotate(30deg) scale(1.08);
            box-shadow: 0 0 16px rgba(168, 85, 247, 0.5);
        }

        #aether-clock-wrap {
            text-align: center;
            margin-bottom: 24px;
        }
        #aether-clock {
            font-size: 64px;
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
            font-size: 32px;
            filter: drop-shadow(0 0 15px rgba(168, 85, 247, 0.7));
        }
        #aether-logo-text {
            font-size: 26px;
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
    <button id="aether-home-settings-btn" title="Настройки Aether">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
    </button>

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
        <span class="aether-hint-pill"><b>Ctrl + Space</b> Пауза/Плей</span>
        <span class="aether-hint-pill"><b>Ctrl + M</b> Только аудио</span>
        <span class="aether-hint-pill"><b>Ctrl + D</b> Скачать видео</span>
    </div>

    <script>
        function updateClock() {
            const now = new Date();
            const hours = String(now.getHours()).padStart(2, '0');
            const minutes = String(now.getMinutes()).padStart(2, '0');
            document.getElementById('aether-clock').textContent = `${hours}:${minutes}`;

            const options = { weekday: 'long', day: 'numeric', month: 'long' };
            document.getElementById('aether-date').textContent = now.toLocaleDateString('ru-RU', options);
        }
        setInterval(updateClock, 1000);
        updateClock();

        document.getElementById("aether-home-settings-btn").addEventListener("click", () => {
            if (window.__AETHER_OPEN_SETTINGS__) {
                window.__AETHER_OPEN_SETTINGS__();
            }
        });

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
</html>
"###;

pub const AETHER_CORE_SCRIPT: &str = r###"
(() => {
    // 1. Запуск строго в верхнем окне (защита от iframe рекламы)
    try {
        if (window.top !== window.self) return;
    } catch (_) {
        return;
    }

    if (window.__AETHER_CORE_ATTACHED__) return;
    window.__AETHER_CORE_ATTACHED__ = true;

    console.log("⚡ Aether Engine v0.5.6: Ядро управления запущено на", window.location.href);

    // ==========================================
    // 2. МЕЖДОМЕННОЕ ХРАНИЛИЩЕ ВКЛАДОК (WINDOW.NAME)
    // ==========================================
    // window.name сохраняется при переходе между ЛЮБЫМИ доменами (start.html -> youtube.com -> vk.com)
    function getTabsState() {
        try {
            if (window.name && window.name.startsWith("__AETHER_TABS__:")) {
                const parsed = JSON.parse(window.name.substring(16));
                if (parsed && Array.isArray(parsed.tabs) && parsed.tabs.length > 0) {
                    return parsed;
                }
            }
        } catch (_) {}

        const isStart = window.location.href.includes("start.html");
        const title = isStart ? "Старт" : (document.title || window.location.hostname || "Вкладка 1");
        return {
            tabs: [
                { id: "tab_" + Date.now(), title: title.substring(0, 16), url: window.location.href, active: true }
            ],
            activeIndex: 0
        };
    }

    function saveTabsState(state) {
        try {
            window.name = "__AETHER_TABS__:" + JSON.stringify(state);
        } catch (_) {}
    }

    function updateCurrentTabInfo() {
        const state = getTabsState();
        if (state.tabs && state.tabs[state.activeIndex]) {
            const isStart = window.location.href.includes("start.html");
            const title = isStart ? "Старт" : (document.title || window.location.hostname || "Сайт");
            state.tabs[state.activeIndex].title = title.substring(0, 16);
            state.tabs[state.activeIndex].url = window.location.href;
            saveTabsState(state);
            renderTabs();
        }
    }

    function createNewTab() {
        const state = getTabsState();
        // Ищем стартовую страницу среди существующих или берем google
        let startUrl = "https://www.google.com";
        const startTab = state.tabs.find(t => t.url.includes("start.html"));
        if (startTab) startUrl = startTab.url;

        const newTab = {
            id: "tab_" + Date.now(),
            title: "Старт",
            url: startUrl,
            active: true
        };
        state.tabs.forEach(t => t.active = false);
        state.tabs.push(newTab);
        state.activeIndex = state.tabs.length - 1;
        saveTabsState(state);

        window.location.href = startUrl;
    }

    function closeTab(index) {
        const state = getTabsState();
        if (state.tabs.length <= 1) {
            showToast("Это единственная вкладка", "ℹ️");
            return;
        }

        const wasActive = state.activeIndex === index;
        state.tabs.splice(index, 1);

        if (state.activeIndex >= state.tabs.length) {
            state.activeIndex = state.tabs.length - 1;
        }

        state.tabs.forEach((t, i) => t.active = (i === state.activeIndex));
        saveTabsState(state);

        if (wasActive) {
            window.location.href = state.tabs[state.activeIndex].url;
        } else {
            renderTabs();
        }
    }

    function switchToTab(index) {
        const state = getTabsState();
        if (index === state.activeIndex) return;
        if (index >= 0 && index < state.tabs.length) {
            state.activeIndex = index;
            state.tabs.forEach((t, i) => t.active = (i === index));
            saveTabsState(state);
            window.location.href = state.tabs[index].url;
        }
    }

    // ==========================================
    // 3. ОТРИСОВКА ВКЛАДОК (СПРАВА В ОСТРОВЕ)
    // ==========================================
    function renderTabs() {
        const strip = document.getElementById("aether-tabs-strip");
        if (!strip) return;
        strip.innerHTML = "";

        const state = getTabsState();
        state.tabs.forEach((tab, index) => {
            const tabEl = document.createElement("div");
            tabEl.className = "aether-tab-item" + (tab.active ? " aether-tab-active" : "");
            
            // Прямые inline стили для обхода CSP
            tabEl.style.cssText = `
                display: flex !important;
                align-items: center !important;
                gap: 6px !important;
                background: ${tab.active ? "rgba(147, 51, 234, 0.4)" : "rgba(255, 255, 255, 0.06)"} !important;
                border: 1px solid ${tab.active ? "rgba(192, 132, 252, 0.65)" : "rgba(255, 255, 255, 0.1)"} !important;
                padding: 4px 10px !important;
                border-radius: 9999px !important;
                font-size: 12px !important;
                color: ${tab.active ? "#ffffff" : "#cbd5e1"} !important;
                cursor: pointer !important;
                white-space: nowrap !important;
                transition: all 0.15s ease !important;
            `;

            tabEl.innerHTML = `
                <span>${tab.title}</span>
                <span class="aether-tab-close" style="margin-left: 2px; font-size: 13px; opacity: 0.7; padding: 0 3px; border-radius: 50%;">✕</span>
            `;

            tabEl.addEventListener("click", (e) => {
                if (e.target.classList.contains("aether-tab-close")) {
                    e.stopPropagation();
                    closeTab(index);
                } else {
                    switchToTab(index);
                }
            });

            strip.appendChild(tabEl);
        });
    }

    // ==========================================
    // 4. МОНТАЖ DYNAMIC ISLAND (ВАРИАНТ Б — КОМПАКТНЫЙ)
    // ==========================================
    function mountDynamicIsland() {
        const target = document.body || document.documentElement;
        if (!target) return;
        if (document.getElementById("aether-island-root")) return;

        const isStart = window.location.href.includes("start.html");
        const domainText = isStart ? "Aether Home" : (window.location.hostname || "Веб");

        const root = document.createElement("div");
        root.id = "aether-island-root";
        root.style.cssText = "position: fixed !important; top: 10px !important; left: 50% !important; transform: translateX(-50%) !important; z-index: 2147483647 !important; pointer-events: auto !important; user-select: none !important; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif !important;";

        // ВАРИАНТ Б: Чистый, не дублирующий текст дизайн
        root.innerHTML = `
            <div id="aether-island" style="
                background: rgba(12, 7, 26, 0.94) !important;
                backdrop-filter: blur(24px) saturate(180%) !important;
                -webkit-backdrop-filter: blur(24px) saturate(180%) !important;
                border: 1.5px solid rgba(168, 85, 247, 0.45) !important;
                border-radius: 9999px !important;
                padding: 4px 10px !important;
                display: flex !important;
                align-items: center !important;
                gap: 6px !important;
                box-shadow: 0 12px 35px rgba(0, 0, 0, 0.7), 0 0 25px rgba(168, 85, 247, 0.25) !important;
            ">
                <!-- 1. Навигация с векторным домиком (Home) -->
                <button id="aether-nav-home" class="aether-icon-btn" title="Домашняя страница Aether" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #d8b4fe; cursor: pointer;">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.3" stroke-linecap="round" stroke-linejoin="round"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
                </button>

                <button id="aether-nav-back" class="aether-icon-btn" title="Назад (Alt + ←)" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #cbd5e1; cursor: pointer;">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
                </button>

                <button id="aether-nav-forward" class="aether-icon-btn" title="Вперед (Alt + →)" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #cbd5e1; cursor: pointer;">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                </button>

                <button id="aether-nav-reload" class="aether-icon-btn" title="Обновить (F5)" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #cbd5e1; cursor: pointer;">
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/></svg>
                </button>

                <!-- 2. Адресная строка (чистый домен, без дублирования со вкладкой) -->
                <div id="aether-url-pill" title="Поиск или ввод адреса (Ctrl + L / F6)" style="
                    display: flex; align-items: center; gap: 6px;
                    background: rgba(26, 17, 48, 0.75); border: 1px solid rgba(168, 85, 247, 0.35);
                    border-radius: 9999px; padding: 4px 12px; cursor: pointer; max-width: 180px;
                ">
                    <span style="font-size: 11px;">🔒</span>
                    <span id="aether-url-text" style="font-size: 12.5px; font-weight: 500; color: #e2e8f0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">${domainText}</span>
                </div>

                <div style="width: 1px; height: 18px; background: rgba(255,255,255,0.12); margin: 0 2px;"></div>

                <!-- 3. Медиа-кнопки и Настройки -->
                <button id="aether-tool-audio" class="aether-icon-btn" title="Режим 'Только аудио' (Ctrl + M) — разгрузка GPU" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #cbd5e1; cursor: pointer;">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 18v-6a9 9 0 0 1 18 0v6"/><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/></svg>
                </button>
                <button id="aether-tool-pip" class="aether-icon-btn" title="Картинка в картинке / PiP (Ctrl + P)" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #cbd5e1; cursor: pointer;">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><rect x="2" y="3" width="20" height="14" rx="2"/><rect x="12" y="9" width="8" height="6" rx="1"/></svg>
                </button>
                <button id="aether-tool-download" class="aether-icon-btn" title="Скачать медиа / yt-dlp (Ctrl + D)" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #cbd5e1; cursor: pointer;">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                </button>
                <button id="aether-tool-settings" class="aether-icon-btn" title="Настройки Aether" style="width: 28px; height: 28px; border-radius: 50%; background: rgba(255,255,255,0.06); border: none; display: flex; align-items: center; justify-content: center; color: #c084fc; cursor: pointer;">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
                </button>

                <div style="width: 1px; height: 18px; background: rgba(255,255,255,0.12); margin: 0 2px;"></div>

                <!-- 4. ВКЛАДКИ СПРАВА -->
                <div id="aether-tabs-strip" style="display: flex; align-items: center; gap: 5px; max-width: 440px; overflow-x: auto;"></div>
                <button id="aether-btn-new-tab" title="Новая вкладка (Ctrl + T)" style="
                    width: 25px; height: 25px; border-radius: 50%;
                    background: rgba(147, 51, 234, 0.3); border: 1px solid rgba(168, 85, 247, 0.45);
                    color: #d8b4fe; font-size: 15px; display: flex; align-items: center; justify-content: center;
                    cursor: pointer; padding: 0;
                ">+</button>
            </div>
        `;

        target.appendChild(root);

        document.getElementById("aether-btn-new-tab").addEventListener("click", createNewTab);
        document.getElementById("aether-nav-home").addEventListener("click", createNewTab);
        document.getElementById("aether-nav-back").addEventListener("click", () => window.history.back());
        document.getElementById("aether-nav-forward").addEventListener("click", () => window.history.forward());
        document.getElementById("aether-nav-reload").addEventListener("click", () => window.location.reload());
        document.getElementById("aether-url-pill").addEventListener("click", openOmnibox);

        document.getElementById("aether-tool-audio").addEventListener("click", toggleAudioOnly);
        document.getElementById("aether-tool-pip").addEventListener("click", togglePiP);
        document.getElementById("aether-tool-download").addEventListener("click", downloadMedia);
        document.getElementById("aether-tool-settings").addEventListener("click", openSettings);

        renderTabs();
    }

    function ensureIslandAttached() {
        if (!document.getElementById("aether-island-root")) {
            mountDynamicIsland();
        }
    }

    // ==========================================
    // 5. НАСТРОЙКИ (SETTINGS)
    // ==========================================
    let settingsOverlay = null;

    function openSettings() {
        const target = document.body || document.documentElement;
        if (!target) return;

        if (!settingsOverlay) {
            settingsOverlay = document.createElement("div");
            settingsOverlay.id = "aether-settings-overlay";
            settingsOverlay.style.cssText = "position: fixed !important; inset: 0 !important; z-index: 2147483646 !important; background: rgba(0,0,0,0.65) !important; backdrop-filter: blur(14px) !important; display: none; align-items: flex-start; justify-content: center; padding-top: 12vh; opacity: 0; transition: opacity 0.18s ease-out;";

            settingsOverlay.innerHTML = `
                <div style="width: 90%; max-width: 520px; background: #0d071a; border: 1.5px solid rgba(168,85,247,0.5); border-radius: 18px; padding: 22px 26px; box-shadow: 0 25px 60px rgba(0,0,0,0.8); color: #f8fafc; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px; border-bottom: 1px solid rgba(168,85,247,0.25); padding-bottom: 12px;">
                        <div style="font-size: 17px; font-weight: 600; color: #d8b4fe; display: flex; align-items: center; gap: 8px;">
                            <span>⚙️</span> <span>Настройки Aether</span>
                        </div>
                        <div id="aether-settings-close" style="font-size: 18px; color: #94a3b8; cursor: pointer; padding: 2px 6px;">✕</div>
                    </div>

                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;">
                        <div>
                            <div style="font-size: 14px; color: #cbd5e1;">Поисковая система</div>
                            <div style="font-size: 11.5px; color: #94a3b8;">Используется в омнибоксе и на стартовой</div>
                        </div>
                        <select id="aether-set-search" style="background: rgba(30, 20, 56, 0.9); color: #fff; border: 1px solid rgba(168,85,247,0.4); border-radius: 8px; padding: 6px 10px; outline: none;">
                            <option value="google">Google</option>
                            <option value="yandex">Яндекс</option>
                            <option value="duckduckgo">DuckDuckGo</option>
                        </select>
                    </div>

                    <div style="margin-top: 24px; padding-top: 14px; border-top: 1px solid rgba(255,255,255,0.08); display: flex; justify-content: space-between; align-items: center;">
                        <span style="font-size: 12px; color: #64748b;">Aether Browser v0.5.6</span>
                        <button id="aether-reset-tabs-btn" style="background: rgba(239,68,68,0.2); border: 1px solid rgba(239,68,68,0.4); color: #fca5a5; border-radius: 8px; padding: 6px 12px; font-size: 12px; cursor: pointer;">Сбросить вкладки</button>
                    </div>
                </div>
            `;

            target.appendChild(settingsOverlay);

            document.getElementById("aether-settings-close").addEventListener("click", closeSettings);
            settingsOverlay.addEventListener("click", (e) => {
                if (e.target === settingsOverlay) closeSettings();
            });

            document.getElementById("aether-reset-tabs-btn").addEventListener("click", () => {
                window.name = "";
                showToast("Вкладки сброшены", "🔄");
                setTimeout(() => window.location.reload(), 500);
            });
        }

        settingsOverlay.style.display = "flex";
        requestAnimationFrame(() => settingsOverlay.style.opacity = "1");
    }

    function closeSettings() {
        if (!settingsOverlay) return;
        settingsOverlay.style.opacity = "0";
        setTimeout(() => settingsOverlay.style.display = "none", 180);
    }

    window.__AETHER_OPEN_SETTINGS__ = openSettings;

    // ==========================================
    // 6. HUD ОМНИБОКС
    // ==========================================
    let omniboxOverlay = null;

    function openOmnibox() {
        const target = document.body || document.documentElement;
        if (!target) return;

        if (!omniboxOverlay) {
            omniboxOverlay = document.createElement("div");
            omniboxOverlay.id = "aether-omnibox-overlay";
            omniboxOverlay.style.cssText = "position: fixed !important; inset: 0 !important; z-index: 2147483646 !important; background: rgba(0,0,0,0.65) !important; backdrop-filter: blur(14px) !important; display: none; align-items: flex-start; justify-content: center; padding-top: 14vh; opacity: 0; transition: opacity 0.18s ease-out;";

            omniboxOverlay.innerHTML = `
                <div style="width: 90%; max-width: 640px; background: #0d071a; border: 1.5px solid rgba(168,85,247,0.5); border-radius: 16px; padding: 18px 22px; box-shadow: 0 25px 60px rgba(0,0,0,0.75);">
                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px;">
                        <div style="font-size: 15px; font-weight: 600; color: #d8b4fe;">🌌 Aether Omnibox</div>
                        <div style="font-size: 11px; color: #94a3b8; background: rgba(255,255,255,0.08); padding: 3px 8px; border-radius: 6px;">Esc — закрыть</div>
                    </div>
                    <input id="aether-omnibox-input" style="width: 100%; box-sizing: border-box; background: rgba(30, 20, 56, 0.9); border: 1.5px solid rgba(168, 85, 247, 0.5); border-radius: 12px; padding: 13px 18px; font-size: 15.5px; color: #fff; outline: none;" type="text" autocomplete="off" spellcheck="false" placeholder="Адрес сайта, поисковый запрос или команда (yt:, vk:, gh:)..." />
                </div>
            `;
            target.appendChild(omniboxOverlay);

            omniboxOverlay.addEventListener("click", (e) => {
                if (e.target === omniboxOverlay) closeOmnibox();
            });

            const input = document.getElementById("aether-omnibox-input");
            input.addEventListener("keydown", (e) => {
                e.stopPropagation();
                if (e.key === "Escape") closeOmnibox();
                else if (e.key === "Enter") {
                    closeOmnibox();
                    navigateTo(input.value);
                }
            });
        }

        const input = document.getElementById("aether-omnibox-input");
        input.value = window.location.href.includes("start.html") ? "" : window.location.href;
        omniboxOverlay.style.display = "flex";

        requestAnimationFrame(() => {
            omniboxOverlay.style.opacity = "1";
            input.focus();
            input.select();
        });
    }

    function closeOmnibox() {
        if (!omniboxOverlay) return;
        omniboxOverlay.style.opacity = "0";
        setTimeout(() => omniboxOverlay.style.display = "none", 180);
    }

    function navigateTo(rawInput) {
        if (!rawInput) return;
        const trimmed = rawInput.trim();
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

        const isUrl = trimmed.startsWith("http://") || trimmed.startsWith("https://") || (trimmed.includes(".") && !trimmed.includes(" ") && !trimmed.endsWith("."));
        if (isUrl) {
            window.location.href = trimmed.startsWith("http") ? trimmed : "https://" + trimmed;
        } else {
            window.location.href = "https://www.google.com/search?q=" + encodeURIComponent(trimmed);
        }
    }

    // ==========================================
    // 7. TOASTS & МЕДИА-КОНТРОЛЬ
    // ==========================================
    function showToast(message, icon = "🌌") {
        const target = document.body || document.documentElement;
        if (!target) return;

        let container = document.getElementById("aether-toast-container");
        if (!container) {
            container = document.createElement("div");
            container.id = "aether-toast-container";
            container.style.cssText = "position: fixed !important; bottom: 24px !important; right: 24px !important; z-index: 2147483647 !important; display: flex !important; flex-direction: column !important; gap: 10px !important; pointer-events: none !important;";
            target.appendChild(container);
        }

        const toast = document.createElement("div");
        toast.style.cssText = "display: flex !important; align-items: center !important; gap: 10px !important; background: rgba(15, 23, 42, 0.94) !important; backdrop-filter: blur(14px) !important; border: 1px solid rgba(168, 85, 247, 0.5) !important; box-shadow: 0 12px 30px rgba(0,0,0,0.6) !important; border-radius: 12px !important; padding: 12px 18px !important; color: #f8fafc !important; font-size: 13.5px !important; max-width: 420px !important; opacity: 0; transform: translateY(12px); transition: all 0.22s ease-out !important;";
        toast.innerHTML = `<span style="font-size: 16px;">${icon}</span><span>${message}</span>`;
        container.appendChild(toast);

        requestAnimationFrame(() => {
            toast.style.opacity = "1";
            toast.style.transform = "translateY(0)";
        });

        setTimeout(() => {
            toast.style.opacity = "0";
            toast.style.transform = "translateY(10px)";
            setTimeout(() => {
                if (toast.parentNode) toast.parentNode.removeChild(toast);
            }, 250);
        }, 3000);
    }

    function togglePlayPause() {
        if (window.vk && window.vk.audio) {
            const playBtn = document.querySelector('.top_audio_player_play, .AudioPlayer__play');
            if (playBtn) { playBtn.click(); showToast("ВК Музыка", "🎵"); return; }
        }
        const media = document.querySelector('video, audio');
        if (media) {
            if (media.paused) { media.play(); showToast("Воспроизведение", "▶️"); }
            else { media.pause(); showToast("Пауза", "⏸️"); }
        }
    }

    function togglePiP() {
        const video = document.querySelector('video');
        if (!video) { showToast("Видео не найдено", "⚠️"); return; }
        if (document.pictureInPictureElement) {
            document.exitPictureInPicture().then(() => showToast("PiP выключен", "📺")).catch(console.error);
        } else if (document.pictureInPictureEnabled) {
            video.requestPictureInPicture().then(() => showToast("Режим PiP", "📺")).catch(console.error);
        }
    }

    function toggleAudioOnly() {
        const video = document.querySelector('video');
        const audioBtn = document.getElementById("aether-tool-audio");
        if (!video) { showToast("Видео не найдено", "⚠️"); return; }

        if (video.style.opacity === '0.001') {
            video.style.opacity = '1';
            video.style.visibility = 'visible';
            if (audioBtn) audioBtn.style.background = "rgba(255,255,255,0.06)";
            showToast("Обычный режим видео", "🎬");
        } else {
            video.style.opacity = '0.001';
            if (audioBtn) audioBtn.style.background = "rgba(168, 85, 247, 0.6)";
            showToast("Режим 'Только аудио' (GPU разгружен)", "🎧");
        }
    }

    function downloadMedia() {
        const curUrl = window.location.href;
        const video = document.querySelector('video');
        if (video && video.src && !video.src.startsWith('blob:')) {
            const a = document.createElement('a');
            a.href = video.src;
            a.download = document.title || 'video.mp4';
            a.target = '_blank';
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            showToast("Скачивание начато", "📥");
            return;
        }

        const cmd = `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best" "${curUrl}"`;
        navigator.clipboard.writeText(cmd).then(() => {
            showToast("Команда yt-dlp скопирована!", "📋");
        }).catch(() => {
            showToast(`Ссылка: ${curUrl}`, "🔗");
        });
    }

    // ==========================================
    // 8. YOUTUBE ADBLOCK
    // ==========================================
    function setupYouTubeAdBlock() {
        if (!location.hostname.includes("youtube.com")) return;

        setInterval(() => {
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
        }, 300);
    }

    // ==========================================
    // 9. ГОРЯЧИЕ КЛАВИШИ
    // ==========================================
    window.addEventListener('keydown', (e) => {
        const ctrlOrCmd = e.ctrlKey || e.metaKey;
        const activeEl = document.activeElement;
        const isTyping = activeEl && (['INPUT', 'TEXTAREA'].includes(activeEl.tagName) || activeEl.isContentEditable);

        if (ctrlOrCmd && (e.key === 't' || e.key === 'T' || e.key === 'е' || e.key === 'Е')) {
            e.preventDefault();
            createNewTab();
            return;
        }

        if (ctrlOrCmd && (e.key === 'w' || e.key === 'W' || e.key === 'ц' || e.key === 'Ц')) {
            e.preventDefault();
            const state = getTabsState();
            closeTab(state.activeIndex);
            return;
        }

        if (ctrlOrCmd && e.key === 'Tab') {
            e.preventDefault();
            const state = getTabsState();
            const nextIndex = e.shiftKey 
                ? (state.activeIndex - 1 + state.tabs.length) % state.tabs.length
                : (state.activeIndex + 1) % state.tabs.length;
            switchToTab(nextIndex);
            return;
        }

        if (e.key === 'F6' || (ctrlOrCmd && (e.key === 'l' || e.key === 'L' || e.key === 'д' || e.key === 'Д'))) {
            e.preventDefault();
            openOmnibox();
            return;
        }

        if (isTyping) return;

        if (ctrlOrCmd && e.code === 'Space') {
            e.preventDefault();
            togglePlayPause();
        }
        else if (ctrlOrCmd && (e.key === 'p' || e.key === 'P' || e.key === 'з' || e.key === 'З')) {
            e.preventDefault();
            togglePiP();
        }
        else if (ctrlOrCmd && (e.key === 'm' || e.key === 'M' || e.key === 'ь' || e.key === 'Ь')) {
            e.preventDefault();
            toggleAudioOnly();
        }
        else if (ctrlOrCmd && (e.key === 'd' || e.key === 'D' || e.key === 'в' || e.key === 'В')) {
            e.preventDefault();
            downloadMedia();
        }
        else if (e.altKey && e.key === 'ArrowLeft') {
            e.preventDefault();
            window.history.back();
        }
        else if (e.altKey && e.key === 'ArrowRight') {
            e.preventDefault();
            window.history.forward();
        }
        else if (e.key === 'F5' || (ctrlOrCmd && (e.key === 'r' || e.key === 'R' || e.key === 'к' || e.key === 'К'))) {
            e.preventDefault();
            window.location.reload();
        }
    });

    // ==========================================
    // 10. ИНИЦИАЛИЗАЦИЯ И ПОСТОЯННЫЙ СЕРДЦЕБИЕНИЕ
    // ==========================================
    function init() {
        mountDynamicIsland();
        updateCurrentTabInfo();
        setupYouTubeAdBlock();

        window.addEventListener('yt-navigate-finish', () => {
            ensureIslandAttached();
            updateCurrentTabInfo();
        });
        window.addEventListener('popstate', () => {
            ensureIslandAttached();
            updateCurrentTabInfo();
        });

        // Сердцебиение: проверка видимости острова каждые 400мс
        setInterval(ensureIslandAttached, 400);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
"###;
