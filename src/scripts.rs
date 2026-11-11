//! Встроенный рантайм Aether Browser v0.6.2 (Arc / Zen Style):
//! - Мощный вертикальный сайдбар слева (боковые вкладки, омнибокс)
//! - Пользовательские быстрые карточки (изначально пусты, пользователь сам добавляет любые сервисы)
//! - Кнопка Домика гарантированно ведет на стартовую вкладку из любого сайта через Rust IPC
//! - Вкладки с сохранением состояния через window.name и Rust IPC
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

        /* Быстрые сервисы */
        #aether-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
            gap: 16px;
            width: 100%;
            max-width: 680px;
            margin-bottom: 36px;
        }
        .aether-card {
            background: rgba(22, 14, 42, 0.65);
            backdrop-filter: blur(12px);
            border: 1px solid rgba(168, 85, 247, 0.25);
            border-radius: 16px;
            padding: 16px 12px;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 10px;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
            text-decoration: none;
            color: #f1f5f9;
            position: relative;
        }
        .aether-card:hover {
            background: rgba(30, 18, 58, 0.9);
            border-color: rgba(192, 132, 252, 0.6);
            transform: translateY(-3px);
            box-shadow: 0 12px 30px rgba(0, 0, 0, 0.55), 0 0 20px rgba(168, 85, 247, 0.35);
        }
        .aether-card-icon {
            width: 48px;
            height: 48px;
            border-radius: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 20px;
            font-weight: 600;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
            transition: transform 0.2s ease;
            background: linear-gradient(135deg, rgba(147, 51, 234, 0.3), rgba(124, 58, 237, 0.2));
            border: 1px solid rgba(168, 85, 247, 0.4);
            color: #d8b4fe;
        }
        .aether-card:hover .aether-card-icon { transform: scale(1.08); }
        .aether-card-name { font-size: 13px; font-weight: 500; color: #e2e8f0; text-align: center; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 100px; }
        
        .aether-card-del {
            position: absolute;
            top: 6px;
            right: 6px;
            width: 20px;
            height: 20px;
            border-radius: 50%;
            background: rgba(239, 68, 68, 0.2);
            border: 1px solid rgba(239, 68, 68, 0.4);
            color: #fca5a5;
            display: none;
            align-items: center;
            justify-content: center;
            font-size: 11px;
            cursor: pointer;
            transition: all 0.15s;
        }
        .aether-card:hover .aether-card-del { display: flex; }
        .aether-card-del:hover { background: rgba(239, 68, 68, 0.8); color: #fff; transform: scale(1.1); }

        /* Кнопка добавления карточки */
        .aether-card-add {
            background: rgba(22, 14, 42, 0.35);
            border: 1.5px dashed rgba(168, 85, 247, 0.35);
            border-radius: 16px;
            padding: 16px 12px;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            gap: 10px;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
            color: #c084fc;
            min-height: 115px;
        }
        .aether-card-add:hover {
            background: rgba(30, 18, 58, 0.6);
            border-color: rgba(192, 132, 252, 0.8);
            transform: translateY(-2px);
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4), 0 0 15px rgba(168, 85, 247, 0.3);
            color: #ffffff;
        }
        .aether-card-add-icon {
            width: 44px;
            height: 44px;
            border-radius: 14px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 24px;
            background: rgba(168, 85, 247, 0.15);
            border: 1px solid rgba(168, 85, 247, 0.3);
        }
        .aether-card-add-name { font-size: 12.5px; font-weight: 500; }

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

        /* Модальное окно добавления */
        #aether-modal-backdrop {
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: rgba(0, 0, 0, 0.7);
            backdrop-filter: blur(8px);
            z-index: 2147483647;
            display: none;
            align-items: center;
            justify-content: center;
        }
        #aether-modal {
            background: #100a20;
            border: 1px solid rgba(168, 85, 247, 0.45);
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8), 0 0 30px rgba(168, 85, 247, 0.3);
            border-radius: 20px;
            width: 100%;
            max-width: 440px;
            padding: 24px 26px;
            display: flex;
            flex-direction: column;
            gap: 16px;
        }
        .aether-modal-title { font-size: 18px; font-weight: 600; color: #f8fafc; display: flex; align-items: center; justify-content: space-between; }
        .aether-modal-input-group { display: flex; flex-direction: column; gap: 6px; }
        .aether-modal-label { font-size: 12px; font-weight: 500; color: #cbd5e1; }
        .aether-modal-input {
            background: rgba(22, 14, 42, 0.85);
            border: 1px solid rgba(168, 85, 247, 0.3);
            border-radius: 9px;
            padding: 9px 12px;
            font-size: 13.5px;
            color: #ffffff;
            outline: none;
            transition: all 0.2s;
        }
        .aether-modal-input:focus { border-color: #c084fc; box-shadow: 0 0 10px rgba(168, 85, 247, 0.35); }
        .aether-presets { display: flex; flex-wrap: wrap; gap: 6px; }
        .aether-preset-chip {
            background: rgba(168, 85, 247, 0.12);
            border: 1px solid rgba(168, 85, 247, 0.25);
            border-radius: 6px;
            padding: 4px 9px;
            font-size: 11.5px;
            color: #d8b4fe;
            cursor: pointer;
            transition: all 0.15s;
        }
        .aether-preset-chip:hover { background: rgba(168, 85, 247, 0.3); color: #fff; }
        .aether-modal-actions { display: flex; justify-content: flex-end; gap: 10px; margin-top: 6px; }
        .aether-btn-cancel {
            background: rgba(255, 255, 255, 0.06);
            border: 1px solid rgba(255, 255, 255, 0.1);
            color: #94a3b8;
            padding: 8px 16px;
            border-radius: 8px;
            font-size: 13px;
            cursor: pointer;
            transition: all 0.15s;
        }
        .aether-btn-cancel:hover { background: rgba(255, 255, 255, 0.12); color: #fff; }
        .aether-btn-submit {
            background: linear-gradient(135deg, #9333ea, #7c3aed);
            border: none;
            color: #ffffff;
            padding: 8px 18px;
            border-radius: 8px;
            font-size: 13px;
            font-weight: 500;
            cursor: pointer;
            box-shadow: 0 4px 15px rgba(147, 51, 234, 0.4);
            transition: all 0.15s;
        }
        .aether-btn-submit:hover { transform: translateY(-1px); box-shadow: 0 6px 20px rgba(147, 51, 234, 0.6); }
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

    <div id="aether-grid"></div>

    <div id="aether-hints">
        <span class="aether-hint-pill"><b>Ctrl + T</b> Новая вкладка</span>
        <span class="aether-hint-pill"><b>Ctrl + W</b> Закрыть вкладку</span>
        <span class="aether-hint-pill"><b>Ctrl + B</b> Скрыть / открыть сайдбар</span>
        <span class="aether-hint-pill"><b>Ctrl + M</b> Только аудио</span>
        <span class="aether-hint-pill"><b>Ctrl + D</b> Скачать видео</span>
    </div>

    <!-- Модальное окно добавления сервиса -->
    <div id="aether-modal-backdrop">
        <div id="aether-modal">
            <div class="aether-modal-title">
                <span>Добавить быстрый сервис</span>
                <span onclick="closeAddModal()" style="cursor:pointer; color:#94a3b8; font-size:16px;">✕</span>
            </div>
            <div class="aether-presets">
                <span class="aether-preset-chip" onclick="applyPreset('YouTube', 'https://www.youtube.com')">YouTube</span>
                <span class="aether-preset-chip" onclick="applyPreset('ВКонтакте', 'https://vk.com')">ВКонтакте</span>
                <span class="aether-preset-chip" onclick="applyPreset('Telegram', 'https://web.telegram.org')">Telegram</span>
                <span class="aether-preset-chip" onclick="applyPreset('GitHub', 'https://github.com')">GitHub</span>
                <span class="aether-preset-chip" onclick="applyPreset('Twitch', 'https://www.twitch.tv')">Twitch</span>
                <span class="aether-preset-chip" onclick="applyPreset('Кинопоиск', 'https://www.kinopoisk.ru')">Кинопоиск</span>
            </div>
            <div class="aether-modal-input-group">
                <label class="aether-modal-label">Название сервиса</label>
                <input id="modal-name" class="aether-modal-input" placeholder="например, YouTube" autocomplete="off" />
            </div>
            <div class="aether-modal-input-group">
                <label class="aether-modal-label">URL адрес</label>
                <input id="modal-url" class="aether-modal-input" placeholder="например, https://www.youtube.com" autocomplete="off" />
            </div>
            <div class="aether-modal-actions">
                <button type="button" class="aether-btn-cancel" onclick="closeAddModal()">Отмена</button>
                <button type="button" class="aether-btn-submit" onclick="submitAddCard()">Добавить</button>
            </div>
        </div>
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

        // --- Управление быстрыми карточками ---
        function getCustomCards() {
            try {
                if (window.name && window.name.startsWith("__AETHER_STATE__=")) {
                    const parsed = JSON.parse(window.name.substring(17));
                    if (parsed && Array.isArray(parsed.cards)) return parsed.cards;
                }
            } catch (_) {}
            try {
                const local = localStorage.getItem('aether_custom_cards');
                if (local) {
                    const parsed = JSON.parse(local);
                    if (Array.isArray(parsed)) return parsed;
                }
            } catch (_) {}
            return []; // Изначально пусто!
        }

        function saveCustomCards(cards) {
            try { localStorage.setItem('aether_custom_cards', JSON.stringify(cards)); } catch (_) {}
            try {
                if (window.name && window.name.startsWith("__AETHER_STATE__=")) {
                    const parsed = JSON.parse(window.name.substring(17));
                    parsed.cards = cards;
                    window.name = "__AETHER_STATE__=" + JSON.stringify(parsed);
                }
            } catch (_) {}
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage("save_cards:" + JSON.stringify(cards));
            }
            renderGrid();
        }

        function renderGrid() {
            const grid = document.getElementById('aether-grid');
            grid.innerHTML = '';
            const cards = getCustomCards();

            cards.forEach((card, index) => {
                const el = document.createElement('div');
                el.className = 'aether-card';
                el.onclick = () => { window.location.href = card.url; };

                const icon = document.createElement('div');
                icon.className = 'aether-card-icon';
                const initial = (card.name || 'S').trim().charAt(0).toUpperCase();
                icon.textContent = initial;

                const name = document.createElement('div');
                name.className = 'aether-card-name';
                name.textContent = card.name;

                const del = document.createElement('div');
                del.className = 'aether-card-del';
                del.textContent = '✕';
                del.title = 'Удалить карточку';
                del.onclick = (e) => {
                    e.stopPropagation();
                    const newCards = cards.filter((_, i) => i !== index);
                    saveCustomCards(newCards);
                };

                el.appendChild(del);
                el.appendChild(icon);
                el.appendChild(name);
                grid.appendChild(el);
            });

            // Кнопка добавления
            const addBtn = document.createElement('div');
            addBtn.className = 'aether-card-add';
            addBtn.onclick = openAddModal;
            addBtn.innerHTML = `
                <div class="aether-card-add-icon">+</div>
                <div class="aether-card-add-name">Добавить сервис</div>
            `;
            grid.appendChild(addBtn);
        }

        function openAddModal() {
            document.getElementById('modal-name').value = '';
            document.getElementById('modal-url').value = '';
            document.getElementById('aether-modal-backdrop').style.display = 'flex';
            document.getElementById('modal-name').focus();
        }

        function closeAddModal() {
            document.getElementById('aether-modal-backdrop').style.display = 'none';
        }

        function applyPreset(name, url) {
            document.getElementById('modal-name').value = name;
            document.getElementById('modal-url').value = url;
        }

        function submitAddCard() {
            const name = document.getElementById('modal-name').value.trim();
            let url = document.getElementById('modal-url').value.trim();
            if (!name || !url) return;

            if (!url.startsWith('http://') && !url.startsWith('https://')) {
                url = 'https://' + url;
            }

            const cards = getCustomCards();
            cards.push({ id: Date.now().toString(), name, url });
            saveCustomCards(cards);
            closeAddModal();
        }

        renderGrid();
    </script>
</body>
</html>"###;

pub fn get_core_script(start_url: &str, saved_cards_json: &str) -> String {
    format!(
        "window.__AETHER_GLOBAL_START_URL__ = {:?};\nwindow.__AETHER_SAVED_CARDS__ = {:?};\n{}",
        start_url,
        saved_cards_json,
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
        return "file:///start.html";
    }

    // 2. УПРАВЛЕНИЕ КАРТОЧКАМИ И ВКЛАДКАМИ
    function getCustomCards() {
        try {
            if (window.name && window.name.startsWith("__AETHER_STATE__=")) {
                const parsed = JSON.parse(window.name.substring(17));
                if (parsed && Array.isArray(parsed.cards)) return parsed.cards;
            }
        } catch (_) {}
        try {
            const local = localStorage.getItem('aether_custom_cards');
            if (local) {
                const parsed = JSON.parse(local);
                if (Array.isArray(parsed)) return parsed;
            }
        } catch (_) {}
        try {
            if (window.__AETHER_SAVED_CARDS__) {
                const parsed = JSON.parse(window.__AETHER_SAVED_CARDS__);
                if (Array.isArray(parsed)) return parsed;
            }
        } catch (_) {}
        return []; // Изначально пусто!
    }

    function saveCustomCards(cards) {
        try { localStorage.setItem('aether_custom_cards', JSON.stringify(cards)); } catch (_) {}
        const state = getTabsState();
        state.cards = cards;
        saveTabsState(state);
        if (window.ipc && window.ipc.postMessage) {
            window.ipc.postMessage("save_cards:" + JSON.stringify(cards));
        }
        renderSidebar();
    }

    function getTabsState() {
        try {
            if (window.name && window.name.startsWith("__AETHER_STATE__=")) {
                const json = window.name.substring(17);
                const parsed = JSON.parse(json);
                if (parsed && Array.isArray(parsed.tabs) && parsed.tabs.length > 0) {
                    if (!Array.isArray(parsed.cards)) {
                        parsed.cards = getCustomCards();
                    }
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
            cards: getCustomCards(),
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

        /* Favorites / Custom Pinned Cards */
        .aether-sb-favorites {
            display: flex !important;
            flex-wrap: wrap !important;
            gap: 5px !important;
            margin-bottom: 10px !important;
            min-height: 28px !important;
            align-items: center !important;
        }
        .aether-sb-fav-tile {
            height: 32px !important;
            min-width: 32px !important;
            padding: 0 8px !important;
            background: rgba(22, 14, 42, 0.65) !important;
            border: 1px solid rgba(168, 85, 247, 0.25) !important;
            border-radius: 7px !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            gap: 5px !important;
            cursor: pointer !important;
            transition: all 0.16s cubic-bezier(0.16, 1, 0.3, 1) !important;
            position: relative !important;
            font-size: 11.5px !important;
            color: #e2e8f0 !important;
        }
        .aether-sb-fav-tile:hover {
            background: rgba(36, 20, 68, 0.95) !important;
            border-color: rgba(192, 132, 252, 0.6) !important;
            transform: translateY(-1px) !important;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4), 0 0 8px rgba(168, 85, 247, 0.3) !important;
            color: #fff !important;
        }
        .aether-sb-fav-tile.add-btn {
            border: 1px dashed rgba(168, 85, 247, 0.4) !important;
            color: #c084fc !important;
            padding: 0 7px !important;
        }
        .aether-sb-fav-tile.add-btn:hover {
            border-color: #c084fc !important;
            background: rgba(168, 85, 247, 0.2) !important;
        }
        .aether-sb-active-dot {
            position: absolute !important;
            bottom: 2px !important;
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
            padding: 2px 4px 6px !important;
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

            <div class="aether-sb-favorites" id="aether-sb-favorites"></div>

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
                <button id="aether-sb-home" class="aether-sb-dock-btn" title="Стартовая страница Aether (Домой)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
                </button>
            </div>
        `);

        // Рендерим пользовательские быстрые карточки в сайдбаре
        const favContainer = document.getElementById('aether-sb-favorites');
        const customCards = getCustomCards();

        customCards.forEach((card, idx) => {
            const tile = document.createElement('div');
            tile.className = 'aether-sb-fav-tile';
            tile.title = card.name + ' (' + card.url + ')';

            const dot = document.createElement('div');
            dot.className = 'aether-sb-active-dot';
            const domain = card.url.replace("https://", "").replace("http://", "").replace("www.", "").split('/')[0];
            const isOpen = state.tabs.some(t => t.url && t.url.includes(domain));
            if (isOpen) dot.style.display = 'block';

            const initial = (card.name || 'S').trim().charAt(0).toUpperCase();
            tile.textContent = initial + ' ' + card.name;
            tile.appendChild(dot);

            tile.onclick = () => openService(card.url);
            favContainer.appendChild(tile);
        });

        // Кнопка добавления сервиса в сайдбаре
        const addTile = document.createElement('div');
        addTile.className = 'aether-sb-fav-tile add-btn';
        addTile.title = 'Добавить быстрый сервис';
        addTile.innerHTML = '+ Сервис';
        addTile.onclick = () => {
            const name = prompt('Введите название сервиса:');
            if (!name) return;
            let url = prompt('Введите URL адрес:');
            if (!url) return;
            if (!url.startsWith('http://') && !url.startsWith('https://')) {
                url = 'https://' + url;
            }
            const cards = getCustomCards();
            cards.push({ id: Date.now().toString(), name, url });
            saveCustomCards(cards);
        };
        favContainer.appendChild(addTile);

        // Рендерим список вкладок
        const tabsContainer = document.getElementById('aether-sb-tabs-list');
        state.tabs.forEach(tab => {
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
        document.getElementById('aether-sb-home').onclick = () => goHome();

        // Медиа-инструменты
        document.getElementById('aether-sb-audio').onclick = () => toggleAudioMode();
        document.getElementById('aether-sb-pip').onclick = () => togglePiP();
        document.getElementById('aether-sb-download').onclick = () => copyYtDlp();
    }

    // 5. ДЕЙСТВИЯ С ВКЛАДКАМИ И ПЕРЕХОД ДОМОЙ ЧЕРЕЗ RUST IPC
    function toggleSidebar(collapsed) {
        const state = getTabsState();
        state.sidebarCollapsed = (typeof collapsed === 'boolean') ? collapsed : !state.sidebarCollapsed;
        saveTabsState(state);
        renderSidebar();
    }

    function goHome() {
        const state = getTabsState();
        // Ищем существующую вкладку Старт
        const startTab = state.tabs.find(t => t.url && t.url.includes("start.html"));
        if (startTab) {
            state.activeTabId = startTab.id;
        } else {
            let cur = state.tabs.find(t => t.id === state.activeTabId);
            if (cur) {
                cur.url = getStartPageUrl();
                cur.title = "Старт";
            }
        }
        saveTabsState(state);

        // Отправляем нативное IPC сообщение в Rust, чтобы обойти ограничения Chromium на file:///
        if (window.ipc && window.ipc.postMessage) {
            window.ipc.postMessage("go_home");
        } else {
            window.location.href = getStartPageUrl();
        }
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

        if (targetUrl.includes("start.html")) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage("go_home");
                return;
            }
        }
        window.location.href = targetUrl;
    }

    function switchTab(id) {
        const state = getTabsState();
        const tab = state.tabs.find(t => t.id === id);
        if (tab && tab.url) {
            state.activeTabId = id;
            saveTabsState(state);

            if (tab.url.includes("start.html")) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage("go_home");
                    return;
                }
            }
            window.location.href = tab.url;
        }
    }

    function closeTab(id) {
        const state = getTabsState();
        const idx = state.tabs.findIndex(t => t.id === id);
        if (idx !== -1) {
            if (state.tabs.length <= 1) {
                goHome();
                return;
            }

            const wasActive = state.activeTabId === id;
            state.tabs.splice(idx, 1);

            if (wasActive) {
                const nextTab = state.tabs[Math.max(0, idx - 1)] || state.tabs[0];
                state.activeTabId = nextTab.id;
                saveTabsState(state);
                if (nextTab.url.includes("start.html")) {
                    goHome();
                } else {
                    window.location.href = nextTab.url;
                }
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

        if (url.includes("start.html")) {
            goHome();
        } else {
            window.location.href = url;
        }
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
        } else if (ctrl && (e.key === 'h' || e.key === 'H' || e.key === 'р' || e.key === 'Р')) {
            e.preventDefault();
            goHome();
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
