//! Встроенный рантайм Aether Browser v0.5.4:
//! - Кастомная фиолетовая стартовая страница (Aether Cosmic Start Page)
//! - Полноценная система вкладок (Tabs Deck) прямо в Dynamic Island
//! - 100% защита от исчезновения на YouTube (прямой монтаж в body + обход CSP)
//! - Настраиваемые быстрые сервисы и хоткеи Ctrl+1..9
//! - AdBlock и 16x ускорение рекламы на YouTube
//! - Медиа-контроль: PiP, режим "Только аудио", скачивание через yt-dlp

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
        }

        /* Часы и дата */
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

        /* Логотип */
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

        /* Строка поиска */
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
        #aether-search-input::placeholder {
            color: #94a3b8;
        }
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

        /* Сетка быстрых сервисов */
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
            border-radius: 14px;
            padding: 16px 14px;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 8px;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
            text-decoration: none;
            color: #f1f5f9;
        }
        .aether-card:hover {
            background: rgba(30, 18, 58, 0.9);
            border-color: rgba(192, 132, 252, 0.6);
            transform: translateY(-3px);
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5), 0 0 20px rgba(168, 85, 247, 0.35);
        }
        .aether-card-icon {
            font-size: 24px;
            width: 44px;
            height: 44px;
            border-radius: 12px;
            background: rgba(255, 255, 255, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }
        .aether-card-name {
            font-size: 13px;
            font-weight: 500;
            color: #e2e8f0;
        }

        /* Футер с горячими клавишами */
        #aether-hints {
            display: flex;
            flex-wrap: wrap;
            justify-content: center;
            gap: 16px;
            font-size: 12px;
            color: #a78bfa;
            opacity: 0.8;
        }
        .aether-hint-pill {
            background: rgba(168, 85, 247, 0.12);
            border: 1px solid rgba(168, 85, 247, 0.25);
            padding: 4px 10px;
            border-radius: 6px;
        }
        .aether-hint-pill b {
            color: #f8fafc;
        }
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
            <div class="aether-card-icon" style="background: rgba(239, 68, 68, 0.2); border-color: rgba(239, 68, 68, 0.4);">📺</div>
            <div class="aether-card-name">YouTube</div>
        </a>
        <a class="aether-card" href="https://vk.com">
            <div class="aether-card-icon" style="background: rgba(59, 130, 246, 0.2); border-color: rgba(59, 130, 246, 0.4);">🎵</div>
            <div class="aether-card-name">ВКонтакте</div>
        </a>
        <a class="aether-card" href="https://web.telegram.org">
            <div class="aether-card-icon" style="background: rgba(56, 189, 248, 0.2); border-color: rgba(56, 189, 248, 0.4);">✈️</div>
            <div class="aether-card-name">Telegram</div>
        </a>
        <a class="aether-card" href="https://github.com">
            <div class="aether-card-icon" style="background: rgba(168, 85, 247, 0.2); border-color: rgba(168, 85, 247, 0.4);">🐙</div>
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
    // 1. Запускать ТОЛЬКО в верхнем окне (никогда не внутри iframe рекламы на YouTube!)
    if (window.top !== window.self) return;
    if (window.__AETHER_CORE_ATTACHED__) return;
    window.__AETHER_CORE_ATTACHED__ = true;

    console.log("⚡ Aether Engine v0.5.4: Ядро управления успешно подключено к", window.location.href);

    // ==========================================
    // 2. СИСТЕМА ВКЛАДОК (TABS ENGINE)
    // ==========================================
    const TABS_KEY = "__aether_tabs_state";

    function getTabsState() {
        try {
            const raw = sessionStorage.getItem(TABS_KEY);
            if (raw) return JSON.parse(raw);
        } catch (_) {}

        // Если вкладок нет — создаем начальную вкладку
        const isStartPage = window.location.href.includes("start.html");
        const title = isStartPage ? "Aether Старт" : (document.title || window.location.hostname || "Вкладка 1");
        return {
            tabs: [
                { id: "tab_" + Date.now(), title: title.substring(0, 18), url: window.location.href, active: true }
            ],
            activeIndex: 0
        };
    }

    function saveTabsState(state) {
        try {
            sessionStorage.setItem(TABS_KEY, JSON.stringify(state));
        } catch (_) {}
    }

    function updateCurrentTabInfo() {
        const state = getTabsState();
        if (state.tabs && state.tabs[state.activeIndex]) {
            const isStartPage = window.location.href.includes("start.html");
            const title = isStartPage ? "Aether Старт" : (document.title || window.location.hostname || "Сайт");
            state.tabs[state.activeIndex].title = title.substring(0, 18);
            state.tabs[state.activeIndex].url = window.location.href;
            saveTabsState(state);
            renderTabs();
        }
    }

    function createNewTab() {
        const state = getTabsState();
        // Находим стартовую страницу или google
        let startUrl = "https://www.google.com";
        const firstTab = state.tabs.find(t => t.url.includes("start.html"));
        if (firstTab) startUrl = firstTab.url;

        const newTab = {
            id: "tab_" + Date.now(),
            title: "Новая вкладка",
            url: startUrl,
            active: true
        };
        state.tabs.forEach(t => t.active = false);
        state.tabs.push(newTab);
        state.activeIndex = state.tabs.length - 1;
        saveTabsState(state);

        window.location.href = startUrl;
    }

    function closeTab(tabIndex) {
        const state = getTabsState();
        if (state.tabs.length <= 1) {
            // Если вкладка одна — просто переходим на домашнюю
            showToast("Это последняя вкладка", "ℹ️");
            return;
        }

        const wasActive = state.activeIndex === tabIndex;
        state.tabs.splice(tabIndex, 1);

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
    // 3. СТИЛИ И МОНТАЖ DYNAMIC ISLAND
    // ==========================================
    function injectIslandStyles() {
        if (document.getElementById('aether-island-styles')) return;
        const style = document.createElement('style');
        style.id = 'aether-island-styles';
        style.textContent = `
            :fullscreen #aether-island-root,
            :-webkit-full-screen #aether-island-root {
                display: none !important;
            }

            #aether-island-root {
                position: fixed !important;
                top: 10px !important;
                left: 50% !important;
                transform: translateX(-50%) !important;
                z-index: 2147483647 !important;
                pointer-events: none !important;
                user-select: none !important;
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
            }

            #aether-island {
                pointer-events: auto !important;
                background: rgba(12, 7, 26, 0.92) !important;
                backdrop-filter: blur(24px) saturate(180%) !important;
                -webkit-backdrop-filter: blur(24px) saturate(180%) !important;
                border: 1px solid rgba(168, 85, 247, 0.45) !important;
                border-radius: 9999px !important;
                padding: 4px 10px !important;
                display: flex !important;
                align-items: center !important;
                gap: 6px !important;
                box-shadow: 0 12px 35px rgba(0, 0, 0, 0.65), 0 0 25px rgba(168, 85, 247, 0.25) !important;
                transition: all 0.22s cubic-bezier(0.16, 1, 0.3, 1) !important;
            }

            #aether-island:hover {
                border-color: rgba(192, 132, 252, 0.75) !important;
                box-shadow: 0 15px 40px rgba(0, 0, 0, 0.75), 0 0 30px rgba(168, 85, 247, 0.4) !important;
            }

            /* Панель вкладок внутри острова */
            #aether-tabs-strip {
                display: flex;
                align-items: center;
                gap: 4px;
                max-width: 420px;
                overflow-x: auto;
            }
            #aether-tabs-strip::-webkit-scrollbar { display: none; }

            .aether-tab-item {
                display: flex;
                align-items: center;
                gap: 6px;
                background: rgba(255, 255, 255, 0.05);
                border: 1px solid rgba(255, 255, 255, 0.08);
                padding: 4px 10px;
                border-radius: 9999px;
                font-size: 12px;
                color: #94a3b8;
                cursor: pointer;
                transition: all 0.15s ease;
                white-space: nowrap;
            }
            .aether-tab-item:hover {
                background: rgba(168, 85, 247, 0.2);
                color: #f8fafc;
            }
            .aether-tab-item.aether-tab-active {
                background: rgba(147, 51, 234, 0.35);
                border-color: rgba(192, 132, 252, 0.6);
                color: #ffffff;
                box-shadow: 0 0 12px rgba(168, 85, 247, 0.35);
            }
            .aether-tab-close {
                font-size: 13px;
                color: #94a3b8;
                border-radius: 50%;
                width: 14px;
                height: 14px;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: all 0.15s;
            }
            .aether-tab-close:hover {
                background: rgba(239, 68, 68, 0.5);
                color: #ffffff;
            }

            .aether-btn-new-tab {
                width: 24px;
                height: 24px;
                border-radius: 50%;
                background: rgba(255, 255, 255, 0.06);
                border: 1px solid rgba(168, 85, 247, 0.35);
                color: #c084fc;
                font-size: 14px;
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: all 0.15s;
                padding: 0;
            }
            .aether-btn-new-tab:hover {
                background: #9333ea;
                color: #fff;
                transform: scale(1.08);
            }

            .aether-btn {
                width: 28px;
                height: 28px;
                border-radius: 50%;
                background: rgba(255, 255, 255, 0.05);
                border: 1px solid transparent;
                display: flex;
                align-items: center;
                justify-content: center;
                color: #cbd5e1;
                cursor: pointer;
                transition: all 0.15s ease;
                outline: none;
                padding: 0;
            }
            .aether-btn:hover {
                background: rgba(168, 85, 247, 0.25);
                border-color: rgba(168, 85, 247, 0.45);
                color: #ffffff;
                transform: scale(1.06);
            }
            .aether-btn.aether-active {
                background: rgba(168, 85, 247, 0.6) !important;
                color: #ffffff !important;
                box-shadow: 0 0 12px rgba(168, 85, 247, 0.6);
            }

            #aether-url-pill {
                display: flex;
                align-items: center;
                gap: 6px;
                background: rgba(26, 17, 48, 0.7);
                border: 1px solid rgba(168, 85, 247, 0.3);
                border-radius: 9999px;
                padding: 4px 12px;
                max-width: 200px;
                cursor: pointer;
                transition: all 0.2s ease;
            }
            #aether-url-pill:hover {
                background: rgba(30, 20, 56, 0.95);
                border-color: rgba(192, 132, 252, 0.6);
            }
            #aether-url-text {
                font-size: 12.5px;
                font-weight: 500;
                color: #e2e8f0;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }

            .aether-sep {
                width: 1px;
                height: 18px;
                background: rgba(255, 255, 255, 0.12);
                margin: 0 2px;
            }

            /* Toasts */
            #aether-toast-container {
                position: fixed !important;
                bottom: 24px !important;
                right: 24px !important;
                z-index: 2147483647 !important;
                display: flex !important;
                flex-direction: column !important;
                gap: 10px !important;
                pointer-events: none !important;
            }
            .aether-toast {
                display: flex;
                align-items: center;
                gap: 10px;
                background: rgba(15, 23, 42, 0.94);
                backdrop-filter: blur(14px);
                border: 1px solid rgba(168, 85, 247, 0.5);
                box-shadow: 0 12px 30px rgba(0, 0, 0, 0.6), 0 0 18px rgba(168, 85, 247, 0.25);
                border-radius: 12px;
                padding: 12px 18px;
                color: #f8fafc;
                font-size: 13.5px;
                max-width: 420px;
                opacity: 0;
                transform: translateY(12px);
                transition: opacity 0.22s ease-out, transform 0.22s ease-out;
            }

            /* Омнибокс HUD */
            #aether-omnibox-overlay {
                position: fixed !important;
                inset: 0 !important;
                z-index: 2147483646 !important;
                background: rgba(0, 0, 0, 0.65) !important;
                backdrop-filter: blur(14px) !important;
                display: none;
                align-items: flex-start;
                justify-content: center;
                padding-top: 14vh;
                opacity: 0;
                transition: opacity 0.18s ease-out;
            }
            #aether-omnibox-card {
                width: 90%;
                max-width: 640px;
                background: #0d071a;
                border: 1.5px solid rgba(168, 85, 247, 0.5);
                border-radius: 16px;
                padding: 18px 22px;
                box-shadow: 0 25px 60px rgba(0, 0, 0, 0.75), 0 0 35px rgba(168, 85, 247, 0.3);
                transform: translateY(-12px) scale(0.98);
                transition: transform 0.18s cubic-bezier(0.16, 1, 0.3, 1);
                color: #f8fafc;
            }
        `;
        (document.head || document.documentElement).appendChild(style);
    }

    function renderTabs() {
        const strip = document.getElementById("aether-tabs-strip");
        if (!strip) return;
        strip.innerHTML = "";

        const state = getTabsState();
        state.tabs.forEach((tab, index) => {
            const tabEl = document.createElement("div");
            tabEl.className = "aether-tab-item" + (tab.active ? " aether-tab-active" : "");
            tabEl.innerHTML = `
                <span>${tab.title}</span>
                <span class="aether-tab-close" title="Закрыть вкладку (Ctrl+W)">×</span>
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

    function mountDynamicIsland() {
        if (!document.body) return;
        if (document.getElementById("aether-island-root")) return;

        injectIslandStyles();

        const host = window.location.href.includes("start.html") ? "Aether Старт" : (window.location.hostname || "Web");
        const root = document.createElement("div");
        root.id = "aether-island-root";
        root.style.cssText = "position: fixed !important; top: 10px !important; left: 50% !important; transform: translateX(-50%) !important; z-index: 2147483647 !important; pointer-events: none !important;";

        root.innerHTML = `
            <div id="aether-island">
                <!-- Кнопка навигации Домой -->
                <button id="aether-nav-home" class="aether-btn" title="Главная Aether (Alt + Home)">
                    <span style="font-size: 13px;">🌌</span>
                </button>

                <!-- Вкладки -->
                <div id="aether-tabs-strip"></div>
                <button id="aether-btn-new-tab" class="aether-btn-new-tab" title="Новая вкладка (Ctrl + T)">+</button>

                <div class="aether-sep"></div>

                <!-- Навигация Назад / Вперед -->
                <button id="aether-nav-back" class="aether-btn" title="Назад (Alt + ←)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
                </button>
                <button id="aether-nav-forward" class="aether-btn" title="Вперед (Alt + →)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                </button>
                <button id="aether-nav-reload" class="aether-btn" title="Обновить (F5)">
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/></svg>
                </button>

                <!-- Адресная капсула -->
                <div id="aether-url-pill" title="Поиск или ввод адреса (Ctrl + L / F6)">
                    <span style="font-size: 11px;">🔒</span>
                    <span id="aether-url-text">${host}</span>
                </div>

                <div class="aether-sep"></div>

                <!-- Медиа-инструменты -->
                <button id="aether-tool-audio" class="aether-btn" title="Режим 'Только аудио' (Ctrl + M) — отключить видеоряд для разгрузки GPU">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 18v-6a9 9 0 0 1 18 0v6"/><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/></svg>
                </button>
                <button id="aether-tool-pip" class="aether-btn" title="Картинка в картинке / PiP (Ctrl + P)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><rect x="2" y="3" width="20" height="14" rx="2"/><rect x="12" y="9" width="8" height="6" rx="1"/></svg>
                </button>
                <button id="aether-tool-download" class="aether-btn" title="Скачать медиа / yt-dlp (Ctrl + D)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                </button>
            </div>
        `;

        document.body.appendChild(root);

        // Обработчики
        document.getElementById("aether-btn-new-tab").addEventListener("click", createNewTab);
        document.getElementById("aether-nav-home").addEventListener("click", createNewTab);
        document.getElementById("aether-nav-back").addEventListener("click", () => window.history.back());
        document.getElementById("aether-nav-forward").addEventListener("click", () => window.history.forward());
        document.getElementById("aether-nav-reload").addEventListener("click", () => window.location.reload());
        document.getElementById("aether-url-pill").addEventListener("click", openOmnibox);

        document.getElementById("aether-tool-audio").addEventListener("click", toggleAudioOnly);
        document.getElementById("aether-tool-pip").addEventListener("click", togglePiP);
        document.getElementById("aether-tool-download").addEventListener("click", downloadMedia);

        renderTabs();
    }

    // Постоянный мониторинг для YouTube SPA и динамических страниц
    function ensureIslandAttached() {
        if (!document.getElementById("aether-island-root") && document.body) {
            mountDynamicIsland();
        }
    }

    // ==========================================
    // 4. ВСПЛЫВАЮЩИЕ УВЕДОМЛЕНИЯ (TOASTS)
    // ==========================================
    function showToast(message, icon = "🌌") {
        let container = document.getElementById("aether-toast-container");
        if (!container && document.body) {
            container = document.createElement("div");
            container.id = "aether-toast-container";
            document.body.appendChild(container);
        }
        if (!container) return;

        const toast = document.createElement("div");
        toast.className = "aether-toast";
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
        }, 3200);
    }

    // ==========================================
    // 5. HUD ОМНИБОКС
    // ==========================================
    let omniboxOverlay = null;

    function openOmnibox() {
        if (!document.body) return;
        if (!omniboxOverlay) {
            omniboxOverlay = document.createElement("div");
            omniboxOverlay.id = "aether-omnibox-overlay";
            omniboxOverlay.innerHTML = `
                <div id="aether-omnibox-card">
                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px;">
                        <div style="font-size: 15px; font-weight: 600; color: #d8b4fe;">🌌 Aether Omnibox</div>
                        <div style="font-size: 11px; color: #94a3b8; background: rgba(255,255,255,0.08); padding: 3px 8px; border-radius: 6px;">Esc — закрыть</div>
                    </div>
                    <input id="aether-omnibox-input" style="width: 100%; box-sizing: border-box; background: rgba(30, 20, 56, 0.9); border: 1.5px solid rgba(168, 85, 247, 0.5); border-radius: 12px; padding: 13px 18px; font-size: 15.5px; color: #fff; outline: none;" type="text" autocomplete="off" spellcheck="false" placeholder="Адрес сайта, поисковый запрос или команда (yt:, vk:, gh:)..." />
                </div>
            `;
            document.body.appendChild(omniboxOverlay);

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
    // 6. МЕДИА-КОНТРОЛЬ И СКАЧИВАНИЕ
    // ==========================================
    function togglePlayPause() {
        if (window.vk && window.vk.audio) {
            const playBtn = document.querySelector('.top_audio_player_play, .AudioPlayer__play');
            if (playBtn) { 
                playBtn.click(); 
                showToast("ВК Музыка", "🎵");
                return; 
            }
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
            if (audioBtn) audioBtn.classList.remove("aether-active");
            showToast("Обычный режим видео", "🎬");
        } else {
            video.style.opacity = '0.001';
            if (audioBtn) audioBtn.classList.add("aether-active");
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
            showToast("Скачивание файла начато", "📥");
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
    // 7. YOUTUBE ADBLOCK И ОЧИСТКА
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
    // 8. ГОРЯЧИЕ КЛАВИШИ
    // ==========================================
    window.addEventListener('keydown', (e) => {
        const ctrlOrCmd = e.ctrlKey || e.metaKey;
        const activeEl = document.activeElement;
        const isTyping = activeEl && (['INPUT', 'TEXTAREA'].includes(activeEl.tagName) || activeEl.isContentEditable);

        // Ctrl+T (Е) — Новая вкладка
        if (ctrlOrCmd && (e.key === 't' || e.key === 'T' || e.key === 'е' || e.key === 'Е')) {
            e.preventDefault();
            createNewTab();
            return;
        }

        // Ctrl+W (Ц) — Закрыть текущую вкладку
        if (ctrlOrCmd && (e.key === 'w' || e.key === 'W' || e.key === 'ц' || e.key === 'Ц')) {
            e.preventDefault();
            const state = getTabsState();
            closeTab(state.activeIndex);
            return;
        }

        // Ctrl+Tab — Следующая вкладка
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
    // 9. ИНИЦИАЛИЗАЦИЯ И СЛУШАТЕЛИ НАВИГАЦИИ
    // ==========================================
    function init() {
        mountDynamicIsland();
        updateCurrentTabInfo();
        setupYouTubeAdBlock();

        // Поддержка YouTube SPA transitions
        window.addEventListener('yt-navigate-finish', () => {
            ensureIslandAttached();
            updateCurrentTabInfo();
        });
        window.addEventListener('popstate', () => {
            ensureIslandAttached();
            updateCurrentTabInfo();
        });

        // Контрольный интервал для гарантии видимости острова
        setInterval(ensureIslandAttached, 800);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
"###;
