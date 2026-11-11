//! Встроенный рантайм Aether Browser v0.6.3 (Arc / Zen Style):
//! - Мощный вертикальный компактный сайдбар слева (175px) — не ломает верстку и сетку YouTube
//! - Пользовательские быстрые карточки (кнопка добавления по центру, при добавлении — слева от карточек)
//! - Умный омнибокс: автодополнение по первым буквам, история поиска и URL, подсказки популярных сайтов
//! - Меню ПКМ по видео: скачивание через yt-dlp, PiP, режим Только аудио, копирование ссылки
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
            margin-bottom: 30px;
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

        /* Поисковая строка и выпадающий список автодополнения */
        #aether-search-container {
            width: 100%;
            max-width: 640px;
            position: relative;
            margin-bottom: 36px;
        }
        #aether-search-box {
            width: 100%;
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
        }
        #aether-search-container.active #aether-search-box,
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
            font-size: 15.5px;
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

        /* Выпадающее окно автодополнения */
        #aether-ac-dropdown {
            position: absolute;
            top: calc(100% + 8px);
            left: 0;
            right: 0;
            background: rgba(14, 8, 28, 0.96);
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
            border: 1px solid rgba(168, 85, 247, 0.4);
            border-radius: 16px;
            box-shadow: 0 20px 50px rgba(0, 0, 0, 0.8), 0 0 25px rgba(168, 85, 247, 0.25);
            overflow: hidden;
            z-index: 1000;
            display: none;
        }
        .aether-ac-item {
            padding: 10px 18px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            cursor: pointer;
            font-size: 14px;
            color: #e2e8f0;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
            transition: background 0.15s;
        }
        .aether-ac-item:last-child { border-bottom: none; }
        .aether-ac-item:hover, .aether-ac-item.selected {
            background: rgba(147, 51, 234, 0.28);
            color: #ffffff;
        }
        .aether-ac-left {
            display: flex;
            align-items: center;
            gap: 12px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
        .aether-ac-icon {
            font-size: 15px;
            color: #c084fc;
            width: 20px;
            text-align: center;
        }
        .aether-ac-text {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
        .aether-ac-text b { color: #f8fafc; }
        .aether-ac-del {
            font-size: 12px;
            color: #94a3b8;
            opacity: 0.6;
            padding: 2px 6px;
            border-radius: 4px;
            cursor: pointer;
        }
        .aether-ac-del:hover {
            opacity: 1;
            color: #fca5a5;
            background: rgba(239, 68, 68, 0.2);
        }

        /* Быстрые сервисы: гибкая центровка. Кнопка + всегда слева, всё по центру */
        #aether-grid {
            display: flex;
            justify-content: center;
            align-items: center;
            flex-wrap: wrap;
            gap: 14px;
            width: 100%;
            max-width: 760px;
            margin: 0 auto 36px;
        }
        .aether-card {
            width: 125px;
            min-height: 110px;
            background: rgba(22, 14, 42, 0.65);
            backdrop-filter: blur(12px);
            border: 1px solid rgba(168, 85, 247, 0.25);
            border-radius: 16px;
            padding: 16px 12px;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
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
            width: 44px;
            height: 44px;
            border-radius: 13px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 19px;
            font-weight: 600;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
            transition: transform 0.2s ease;
            background: linear-gradient(135deg, rgba(147, 51, 234, 0.3), rgba(124, 58, 237, 0.2));
            border: 1px solid rgba(168, 85, 247, 0.4);
            color: #d8b4fe;
        }
        .aether-card:hover .aether-card-icon { transform: scale(1.08); }
        .aether-card-name {
            font-size: 12.5px;
            font-weight: 500;
            color: #e2e8f0;
            text-align: center;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            max-width: 100px;
        }
        
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
            width: 125px;
            min-height: 110px;
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
            border-radius: 13px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 24px;
            background: rgba(168, 85, 247, 0.15);
            border: 1px solid rgba(168, 85, 247, 0.3);
        }
        .aether-card-add-name { font-size: 12.5px; font-weight: 500; text-align: center; }

        #aether-hints {
            display: flex;
            flex-wrap: wrap;
            justify-content: center;
            gap: 12px;
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

        /* Модальное окно добавления сервиса */
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

    <div id="aether-search-container">
        <form id="aether-search-box" onsubmit="handleSearchSubmit(event)">
            <input id="aether-search-input" type="text" placeholder="Поиск в Google, запрос 'yt: ...' или ввод адреса..." autocomplete="off" autofocus />
            <button id="aether-search-btn" type="submit">
                <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
            </button>
        </form>
        <div id="aether-ac-dropdown"></div>
    </div>

    <!-- Список быстрых сервисов -->
    <div id="aether-grid"></div>

    <div id="aether-hints">
        <span class="aether-hint-pill"><b>Ctrl + T</b> Новая вкладка</span>
        <span class="aether-hint-pill"><b>Ctrl + W</b> Закрыть вкладку</span>
        <span class="aether-hint-pill"><b>Ctrl + B</b> Скрыть / открыть сайдбар</span>
        <span class="aether-hint-pill"><b>Ctrl + H</b> Домой</span>
        <span class="aether-hint-pill"><b>ПКМ по видео</b> Скачать / PiP / Аудио</span>
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
                <input id="modal-url" class="aether-modal-input" placeholder="например, vk.ru или youtube.com" autocomplete="off" />
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

        // Популярные интернет-сервисы для автодополнения
        const POPULAR_SITES = [
            { name: 'YouTube', domain: 'youtube.com', url: 'https://www.youtube.com', icon: '▶' },
            { name: 'ВКонтакте', domain: 'vk.com', url: 'https://vk.com', icon: 'VK' },
            { name: 'Telegram', domain: 'web.telegram.org', url: 'https://web.telegram.org', icon: '✈' },
            { name: 'GitHub', domain: 'github.com', url: 'https://github.com', icon: '🐙' },
            { name: 'Twitch', domain: 'twitch.tv', url: 'https://www.twitch.tv', icon: '🎮' },
            { name: 'Кинопоиск', domain: 'kinopoisk.ru', url: 'https://www.kinopoisk.ru', icon: '🎬' },
            { name: 'Яндекс', domain: 'ya.ru', url: 'https://ya.ru', icon: 'Я' },
            { name: 'Google', domain: 'google.com', url: 'https://google.com', icon: 'G' },
            { name: 'Rutube', domain: 'rutube.ru', url: 'https://rutube.ru', icon: '📺' },
            { name: 'Reddit', domain: 'reddit.com', url: 'https://reddit.com', icon: '🤖' },
            { name: 'Википедия', domain: 'wikipedia.org', url: 'https://ru.wikipedia.org', icon: 'W' },
            { name: 'Habr', domain: 'habr.com', url: 'https://habr.com', icon: 'H' },
            { name: 'Mail.ru', domain: 'mail.ru', url: 'https://mail.ru', icon: '@' },
            { name: 'Ozon', domain: 'ozon.ru', url: 'https://www.ozon.ru', icon: '📦' },
            { name: 'Wildberries', domain: 'wildberries.ru', url: 'https://www.wildberries.ru', icon: '🛍' },
            { name: 'ChatGPT', domain: 'chatgpt.com', url: 'https://chatgpt.com', icon: '💬' }
        ];

        // --- Управление историей поиска и URL ---
        function getHistory() {
            try {
                const raw = localStorage.getItem('aether_search_history');
                if (raw) return JSON.parse(raw);
            } catch (_) {}
            return [];
        }

        function saveHistoryItem(val, type = 'query') {
            if (!val || typeof val !== 'string') return;
            const text = val.trim();
            if (!text) return;
            try {
                let list = getHistory();
                list = list.filter(item => item.val.toLowerCase() !== text.toLowerCase());
                list.unshift({ val: text, type: type, ts: Date.now() });
                if (list.length > 60) list.pop();
                localStorage.setItem('aether_search_history', JSON.stringify(list));
            } catch (_) {}
        }

        function deleteHistoryItem(val) {
            try {
                let list = getHistory();
                list = list.filter(item => item.val.toLowerCase() !== val.toLowerCase());
                localStorage.setItem('aether_search_history', JSON.stringify(list));
                renderAutocomplete();
            } catch (_) {}
        }

        // --- Обработка навигации ---
        function navigateTo(target) {
            if (!target) return;
            let url = target.trim();
            saveHistoryItem(url, url.startsWith('http') ? 'url' : 'query');

            if (url.toLowerCase().startsWith("yt:") || url.toLowerCase().startsWith("ютуб:")) {
                const query = url.replace(/^(yt:|ютуб:)/i, "").trim();
                window.location.href = "https://www.youtube.com/results?search_query=" + encodeURIComponent(query);
                return;
            }
            if (url.toLowerCase().startsWith("vk:") || url.toLowerCase().startsWith("вк:")) {
                const query = url.replace(/^(vk:|вк:)/i, "").trim();
                window.location.href = "https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D=" + encodeURIComponent(query);
                return;
            }
            if (url.toLowerCase().startsWith("gh:")) {
                const query = url.replace(/^gh:/i, "").trim();
                window.location.href = "https://github.com/search?q=" + encodeURIComponent(query);
                return;
            }

            const isUrl = url.startsWith("http://") || url.startsWith("https://") || (url.includes(".") && !url.includes(" ") && !url.endsWith("."));
            if (isUrl) {
                const dest = (url.startsWith("http://") || url.startsWith("https://")) ? url : "https://" + url;
                window.location.href = dest;
            } else {
                window.location.href = "https://www.google.com/search?q=" + encodeURIComponent(url);
            }
        }

        function handleSearchSubmit(e) {
            e.preventDefault();
            const input = document.getElementById('aether-search-input');
            navigateTo(input.value);
        }

        // --- Выпадающий список автодополнения ---
        const searchInput = document.getElementById('aether-search-input');
        const acDropdown = document.getElementById('aether-ac-dropdown');
        const searchContainer = document.getElementById('aether-search-container');
        let selectedIndex = -1;

        function renderAutocomplete() {
            const val = searchInput.value.trim().toLowerCase();
            acDropdown.innerHTML = '';
            selectedIndex = -1;

            const items = [];

            // 1. Поиск в истории
            const history = getHistory();
            if (val.length === 0) {
                // Если строка пустая, показываем последние 5 запросов из истории
                history.slice(0, 5).forEach(h => {
                    items.push({ text: h.val, type: 'history', target: h.val, icon: '🕒' });
                });
            } else {
                // Ищем по совпадению в истории
                history.filter(h => h.val.toLowerCase().includes(val)).slice(0, 4).forEach(h => {
                    items.push({ text: h.val, type: 'history', target: h.val, icon: '🕒' });
                });

                // 2. Интернет подсказки (популярные сервисы)
                POPULAR_SITES.filter(s => s.domain.includes(val) || s.name.toLowerCase().includes(val)).slice(0, 4).forEach(s => {
                    items.push({ text: s.name + ' (' + s.domain + ')', type: 'site', target: s.url, icon: s.icon });
                });

                // 3. Быстрый поиск в Google и YouTube
                items.push({ text: `Искать в Google: "${searchInput.value.trim()}"`, type: 'google', target: searchInput.value.trim(), icon: '🔍' });
                items.push({ text: `Искать на YouTube: "${searchInput.value.trim()}"`, type: 'youtube', target: "yt: " + searchInput.value.trim(), icon: '▶' });
            }

            if (items.length === 0) {
                acDropdown.style.display = 'none';
                searchContainer.classList.remove('active');
                return;
            }

            items.forEach((item, idx) => {
                const el = document.createElement('div');
                el.className = 'aether-ac-item';
                el.onclick = () => {
                    navigateTo(item.target);
                };

                const left = document.createElement('div');
                left.className = 'aether-ac-left';
                left.innerHTML = `<span class="aether-ac-icon">${item.icon}</span><span class="aether-ac-text">${item.text}</span>`;

                el.appendChild(left);

                if (item.type === 'history') {
                    const del = document.createElement('span');
                    del.className = 'aether-ac-del';
                    del.textContent = '✕';
                    del.title = 'Удалить из истории';
                    del.onclick = (e) => {
                        e.stopPropagation();
                        deleteHistoryItem(item.target);
                    };
                    el.appendChild(del);
                }

                acDropdown.appendChild(el);
            });

            acDropdown.style.display = 'block';
            searchContainer.classList.add('active');
        }

        searchInput.addEventListener('input', renderAutocomplete);
        searchInput.addEventListener('focus', renderAutocomplete);

        searchInput.addEventListener('keydown', (e) => {
            const list = acDropdown.querySelectorAll('.aether-ac-item');
            if (!list.length || acDropdown.style.display === 'none') return;

            if (e.key === 'ArrowDown') {
                e.preventDefault();
                selectedIndex = (selectedIndex + 1) % list.length;
                updateSelection(list);
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                selectedIndex = (selectedIndex - 1 + list.length) % list.length;
                updateSelection(list);
            } else if (e.key === 'Enter') {
                if (selectedIndex >= 0 && selectedIndex < list.length) {
                    e.preventDefault();
                    list[selectedIndex].click();
                }
            } else if (e.key === 'Escape') {
                acDropdown.style.display = 'none';
                searchContainer.classList.remove('active');
            }
        });

        function updateSelection(list) {
            list.forEach((el, i) => {
                if (i === selectedIndex) {
                    el.classList.add('selected');
                    el.scrollIntoView({ block: 'nearest' });
                } else {
                    el.classList.remove('selected');
                }
            });
        }

        document.addEventListener('click', (e) => {
            if (!searchContainer.contains(e.target)) {
                acDropdown.style.display = 'none';
                searchContainer.classList.remove('active');
            }
        });

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
                try {
                    window.ipc.postMessage("save_cards:" + JSON.stringify(cards));
                } catch (_) {}
            }
            renderGrid();
        }

        function renderGrid() {
            const grid = document.getElementById('aether-grid');
            grid.innerHTML = '';
            const cards = getCustomCards();

            // 1. Кнопка добавления: всегда слева от добавленных карточек
            // При пустом списке карточек она находится ровно по центру
            const addBtn = document.createElement('div');
            addBtn.className = 'aether-card-add';
            addBtn.onclick = openAddModal;
            addBtn.title = 'Добавить быстрый сервис';
            addBtn.innerHTML = `
                <div class="aether-card-add-icon">+</div>
                <div class="aether-card-add-name">Добавить сервис</div>
            `;
            grid.appendChild(addBtn);

            // 2. Карточки пользователя справа от кнопки "+"
            cards.forEach((card, index) => {
                const el = document.createElement('div');
                el.className = 'aether-card';
                el.onclick = () => { navigateTo(card.url); };

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
        return "aether://localhost/start.html";
    }

    // 2. УПРАВЛЕНИЕ ИСТОРИЕЙ
    function getHistory() {
        try {
            const raw = localStorage.getItem('aether_search_history');
            if (raw) return JSON.parse(raw);
        } catch (_) {}
        return [];
    }

    function saveHistoryItem(val, type = 'query') {
        if (!val || typeof val !== 'string') return;
        const text = val.trim();
        if (!text) return;
        try {
            let list = getHistory();
            list = list.filter(item => item.val.toLowerCase() !== text.toLowerCase());
            list.unshift({ val: text, type: type, ts: Date.now() });
            if (list.length > 60) list.pop();
            localStorage.setItem('aether_search_history', JSON.stringify(list));
        } catch (_) {}
    }

    // 3. УПРАВЛЕНИЕ КАРТОЧКАМИ И ВКЛАДКАМИ
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
        return []; // Изначально пусто
    }

    function saveCustomCards(cards) {
        try { localStorage.setItem('aether_custom_cards', JSON.stringify(cards)); } catch (_) {}
        const state = getTabsState();
        state.cards = cards;
        saveTabsState(state);
        if (window.ipc && window.ipc.postMessage) {
            try {
                window.ipc.postMessage("save_cards:" + JSON.stringify(cards));
            } catch (_) {}
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
        const initialTitle = (startUrl.includes("start.html") || startUrl.includes("aether://")) ? "Старт" : (document.title || "Вкладка");
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
        const curTitle = (curUrl.includes("start.html") || curUrl.includes("aether://")) ? "Старт" : (document.title || (location.hostname.replace("www.", "") || "Вкладка"));

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

    // 4. КОМПАКТНЫЕ СТИЛИ САЙДБАРА ARC (175px — сохраняет 4-колоночную сетку YouTube)
    const SIDEBAR_CSS = `
        :root {
            --aether-sidebar-w: 175px;
        }
        html {
            margin-left: 175px !important;
            width: calc(100vw - 175px) !important;
            max-width: calc(100vw - 175px) !important;
            box-sizing: border-box !important;
            transition: margin-left 0.2s cubic-bezier(0.16, 1, 0.3, 1), width 0.2s cubic-bezier(0.16, 1, 0.3, 1) !important;
        }
        body {
            width: 100% !important;
            max-width: 100% !important;
        }
        #masthead-container, #guide, #header, header[role="banner"] {
            left: 175px !important;
            width: calc(100vw - 175px) !important;
            transition: left 0.2s cubic-bezier(0.16, 1, 0.3, 1), width 0.2s cubic-bezier(0.16, 1, 0.3, 1) !important;
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
            width: 175px !important;
            height: 100vh !important;
            background: linear-gradient(180deg, #090514 0%, #0d071a 50%, #070310 100%) !important;
            border-right: 1px solid rgba(168, 85, 247, 0.25) !important;
            box-shadow: 4px 0 24px rgba(0, 0, 0, 0.65) !important;
            z-index: 2147483640 !important;
            display: flex !important;
            flex-direction: column !important;
            padding: 10px 7px 8px !important;
            box-sizing: border-box !important;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
            user-select: none !important;
            color: #f8fafc !important;
            transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1) !important;
        }
        #aether-arc-sidebar.collapsed {
            transform: translateX(-175px) !important;
        }

        /* Кнопка раскрытия при свернутом сайдбаре */
        #aether-expand-pill {
            position: fixed !important;
            top: 10px !important;
            left: 10px !important;
            width: 30px !important;
            height: 30px !important;
            border-radius: 8px !important;
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

        /* Шапка сайдбара */
        .aether-sb-header {
            display: flex !important;
            align-items: center !important;
            justify-content: space-between !important;
            padding: 2px 2px 8px !important;
        }
        .aether-sb-brand {
            display: flex !important;
            align-items: center !important;
            gap: 6px !important;
            font-size: 13.5px !important;
            font-weight: 700 !important;
            letter-spacing: 1.5px !important;
            background: linear-gradient(135deg, #f8fafc 20%, #c084fc 100%) !important;
            -webkit-background-clip: text !important;
            -webkit-text-fill-color: transparent !important;
        }
        .aether-sb-brand-icon {
            font-size: 15px !important;
            -webkit-text-fill-color: initial !important;
        }
        .aether-sb-tool-btn {
            background: rgba(255, 255, 255, 0.05) !important;
            border: 1px solid rgba(168, 85, 247, 0.25) !important;
            border-radius: 5px !important;
            width: 22px !important;
            height: 22px !important;
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

        /* Омнибокс и навигация */
        .aether-sb-nav-row {
            display: flex !important;
            align-items: center !important;
            gap: 3px !important;
            margin-bottom: 7px !important;
        }
        .aether-sb-nav-btn {
            width: 22px !important;
            height: 22px !important;
            border-radius: 5px !important;
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
            border-radius: 6px !important;
            padding: 3px 5px 3px 18px !important;
            font-size: 11px !important;
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
            left: 5px !important;
            font-size: 9px !important;
            color: #a855f7 !important;
            pointer-events: none !important;
        }

        /* Автодополнение в сайдбаре */
        #aether-sb-ac-dropdown {
            position: absolute !important;
            top: 26px !important;
            left: 0 !important;
            width: 220px !important;
            background: rgba(14, 8, 28, 0.98) !important;
            backdrop-filter: blur(16px) !important;
            border: 1px solid rgba(168, 85, 247, 0.45) !important;
            border-radius: 9px !important;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.8) !important;
            z-index: 2147483647 !important;
            display: none !important;
            overflow: hidden !important;
        }
        .aether-sb-ac-item {
            padding: 6px 10px !important;
            display: flex !important;
            align-items: center !important;
            gap: 8px !important;
            font-size: 11px !important;
            color: #cbd5e1 !important;
            cursor: pointer !important;
            border-bottom: 1px solid rgba(255, 255, 255, 0.04) !important;
        }
        .aether-sb-ac-item:hover, .aether-sb-ac-item.selected {
            background: rgba(147, 51, 234, 0.3) !important;
            color: #ffffff !important;
        }
        .aether-sb-ac-text {
            white-space: nowrap !important;
            overflow: hidden !important;
            text-overflow: ellipsis !important;
        }

        /* Быстрые сервисы в сайдбаре */
        .aether-sb-favorites {
            display: flex !important;
            flex-wrap: wrap !important;
            gap: 4px !important;
            margin-bottom: 8px !important;
            min-height: 24px !important;
            align-items: center !important;
        }
        .aether-sb-fav-tile {
            height: 25px !important;
            padding: 0 6px !important;
            background: rgba(22, 14, 42, 0.65) !important;
            border: 1px solid rgba(168, 85, 247, 0.25) !important;
            border-radius: 6px !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            gap: 4px !important;
            cursor: pointer !important;
            transition: all 0.16s ease !important;
            position: relative !important;
            font-size: 11px !important;
            color: #e2e8f0 !important;
        }
        .aether-sb-fav-tile:hover {
            background: rgba(36, 20, 68, 0.95) !important;
            border-color: rgba(192, 132, 252, 0.6) !important;
            color: #fff !important;
        }
        .aether-sb-fav-tile.add-btn {
            border: 1px dashed rgba(168, 85, 247, 0.4) !important;
            color: #c084fc !important;
        }
        .aether-sb-active-dot {
            position: absolute !important;
            bottom: 2px !important;
            width: 3px !important;
            height: 3px !important;
            border-radius: 50% !important;
            background: #c084fc !important;
            display: none !important;
        }

        /* Вкладки */
        .aether-sb-tabs-header {
            display: flex !important;
            align-items: center !important;
            justify-content: space-between !important;
            padding: 2px 3px 5px !important;
            font-size: 10px !important;
            font-weight: 600 !important;
            color: #94a3b8 !important;
            letter-spacing: 0.8px !important;
            text-transform: uppercase !important;
        }
        .aether-sb-new-tab-btn {
            background: rgba(147, 51, 234, 0.25) !important;
            border: 1px solid rgba(168, 85, 247, 0.35) !important;
            border-radius: 4px !important;
            width: 18px !important;
            height: 18px !important;
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
        }

        .aether-sb-tabs-list {
            flex: 1 !important;
            overflow-y: auto !important;
            overflow-x: hidden !important;
            display: flex !important;
            flex-direction: column !important;
            gap: 3px !important;
            padding: 1px !important;
        }
        .aether-sb-tabs-list::-webkit-scrollbar { width: 3px !important; }
        .aether-sb-tabs-list::-webkit-scrollbar-thumb { background: rgba(168, 85, 247, 0.3) !important; border-radius: 3px !important; }

        .aether-sb-tab-item {
            display: flex !important;
            align-items: center !important;
            gap: 6px !important;
            padding: 5px 6px !important;
            border-radius: 6px !important;
            font-size: 11.5px !important;
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
        }
        .aether-sb-tab-badge {
            width: 14px !important;
            height: 14px !important;
            flex-shrink: 0 !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            font-size: 10px !important;
        }
        .aether-sb-tab-title {
            flex: 1 !important;
            white-space: nowrap !important;
            overflow: hidden !important;
            text-overflow: ellipsis !important;
        }
        .aether-sb-tab-close {
            opacity: 0 !important;
            font-size: 10px !important;
            color: #94a3b8 !important;
            padding: 1px 3px !important;
            border-radius: 3px !important;
            transition: all 0.12s !important;
        }
        .aether-sb-tab-item:hover .aether-sb-tab-close { opacity: 0.75 !important; }
        .aether-sb-tab-close:hover { opacity: 1 !important; color: #fca5a5 !important; background: rgba(239, 68, 68, 0.25) !important; }

        /* Нижний док сайдбара (Только Аудио, PiP, Домой) */
        .aether-sb-dock {
            margin-top: auto !important;
            padding-top: 6px !important;
            border-top: 1px solid rgba(255, 255, 255, 0.08) !important;
            display: flex !important;
            align-items: center !important;
            justify-content: space-around !important;
        }
        .aether-sb-dock-btn {
            width: 28px !important;
            height: 28px !important;
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
        .aether-sb-dock-btn:hover {
            background: rgba(168, 85, 247, 0.25) !important;
            color: #d8b4fe !important;
            border-color: rgba(168, 85, 247, 0.45) !important;
        }

        /* Всплывающее меню ПКМ по видео */
        #aether-video-menu {
            position: fixed !important;
            background: rgba(15, 9, 30, 0.96) !important;
            backdrop-filter: blur(16px) !important;
            border: 1px solid rgba(168, 85, 247, 0.45) !important;
            border-radius: 12px !important;
            box-shadow: 0 15px 40px rgba(0, 0, 0, 0.85), 0 0 20px rgba(168, 85, 247, 0.3) !important;
            padding: 6px 0 !important;
            min-width: 210px !important;
            z-index: 2147483647 !important;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
            user-select: none !important;
            color: #f1f5f9 !important;
            display: none;
        }
        .aether-vm-header {
            padding: 6px 14px 4px !important;
            font-size: 10.5px !important;
            font-weight: 600 !important;
            color: #a78bfa !important;
            text-transform: uppercase !important;
            letter-spacing: 0.5px !important;
            border-bottom: 1px solid rgba(255, 255, 255, 0.06) !important;
            margin-bottom: 4px !important;
        }
        .aether-vm-item {
            padding: 8px 14px !important;
            font-size: 12.5px !important;
            display: flex !important;
            align-items: center !important;
            gap: 9px !important;
            cursor: pointer !important;
            transition: background 0.15s !important;
        }
        .aether-vm-item:hover {
            background: rgba(147, 51, 234, 0.35) !important;
            color: #ffffff !important;
        }

        /* Уведомление (Toast) */
        #aether-toast {
            position: fixed !important;
            top: 18px !important;
            left: 50% !important;
            transform: translateX(-50%) !important;
            background: rgba(20, 12, 40, 0.95) !important;
            border: 1px solid rgba(168, 85, 247, 0.6) !important;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.7), 0 0 20px rgba(168, 85, 247, 0.4) !important;
            color: #ffffff !important;
            padding: 10px 20px !important;
            border-radius: 10px !important;
            font-size: 13.5px !important;
            font-weight: 500 !important;
            z-index: 2147483647 !important;
            display: none;
            backdrop-filter: blur(12px) !important;
            transition: opacity 0.25s ease !important;
        }

        /* Модальное окно добавления сервиса в сайдбаре */
        #aether-sb-modal-backdrop {
            position: fixed !important;
            top: 0 !important; left: 0 !important; right: 0 !important; bottom: 0 !important;
            background: rgba(0, 0, 0, 0.7) !important;
            backdrop-filter: blur(6px) !important;
            z-index: 2147483647 !important;
            display: none;
            align-items: center !important;
            justify-content: center !important;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
        }
        #aether-sb-modal {
            background: #110a24 !important;
            border: 1px solid rgba(168, 85, 247, 0.45) !important;
            border-radius: 16px !important;
            padding: 20px 22px !important;
            width: 320px !important;
            display: flex !important;
            flex-direction: column !important;
            gap: 12px !important;
            color: #f8fafc !important;
            box-shadow: 0 20px 50px rgba(0, 0, 0, 0.8) !important;
        }
        .aether-sb-modal-input {
            background: rgba(25, 15, 48, 0.9) !important;
            border: 1px solid rgba(168, 85, 247, 0.35) !important;
            border-radius: 7px !important;
            padding: 8px 10px !important;
            font-size: 12.5px !important;
            color: #fff !important;
            outline: none !important;
        }
        .aether-sb-modal-input:focus { border-color: #c084fc !important; }
        .aether-sb-modal-btn {
            background: linear-gradient(135deg, #9333ea, #7c3aed) !important;
            border: none !important;
            border-radius: 7px !important;
            color: #fff !important;
            padding: 8px 14px !important;
            font-size: 12.5px !important;
            cursor: pointer !important;
        }
    `;

    function injectStyle() {
        if (document.getElementById('aether-arc-style')) return;
        const style = document.createElement('style');
        style.id = 'aether-arc-style';
        style.textContent = SIDEBAR_CSS;
        (document.head || document.documentElement).appendChild(style);
    }

    function showToast(text) {
        let toast = document.getElementById('aether-toast');
        if (!toast) {
            toast = document.createElement('div');
            toast.id = 'aether-toast';
            document.documentElement.appendChild(toast);
        }
        toast.textContent = text;
        toast.style.display = 'block';
        toast.style.opacity = '1';
        setTimeout(() => {
            toast.style.opacity = '0';
            setTimeout(() => { toast.style.display = 'none'; }, 250);
        }, 2500);
    }

    function getTabIcon(url, title) {
        const u = (url || '').toLowerCase();
        if (u.includes('youtube.com')) return '<span style="color:#ef4444; font-size:11px;">▶</span>';
        if (u.includes('vk.com')) return '<span style="color:#0077ff; font-weight:bold; font-size:9.5px;">VK</span>';
        if (u.includes('telegram.org')) return '<span style="color:#38bdf8; font-size:10px;">✈</span>';
        if (u.includes('github.com')) return '<span style="color:#f8fafc; font-size:10px;">🐙</span>';
        if (u.includes('start.html') || u.includes('aether://')) return '<span style="color:#c084fc; font-size:10px;">🌌</span>';
        return '<span style="color:#94a3b8; font-size:10px;">🌐</span>';
    }

    // 5. ПОСТРОЕНИЕ САЙДБАРА
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
                        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
                    </button>
                </div>
            </div>

            <div class="aether-sb-nav-row">
                <button id="aether-sb-back" class="aether-sb-nav-btn" title="Назад (Alt + ←)">
                    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
                </button>
                <button id="aether-sb-forward" class="aether-sb-nav-btn" title="Вперед (Alt + →)">
                    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                </button>
                <button id="aether-sb-reload" class="aether-sb-nav-btn" title="Обновить (F5)">
                    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/></svg>
                </button>
                <div class="aether-sb-omnibox-wrap">
                    <span class="aether-sb-omnibox-icon">🔍</span>
                    <input id="aether-sb-omnibox" class="aether-sb-omnibox-input" type="text" placeholder="Поиск / URL..." autocomplete="off" />
                    <div id="aether-sb-ac-dropdown"></div>
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
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 18v-6a9 9 0 0 1 18 0v6"/><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/></svg>
                </button>
                <button id="aether-sb-pip" class="aether-sb-dock-btn" title="Картинка в картинке / PiP (Ctrl + P)">
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><rect x="2" y="3" width="20" height="14" rx="2"/><rect x="12" y="9" width="8" height="6" rx="1"/></svg>
                </button>
                <button id="aether-sb-home" class="aether-sb-dock-btn" title="Стартовая страница Aether (Ctrl + H)">
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
                </button>
            </div>
        `);

        // Рендерим быстрые сервисы
        const favContainer = document.getElementById('aether-sb-favorites');
        const customCards = getCustomCards();

        customCards.forEach((card) => {
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
        addTile.onclick = openSidebarAddModal;
        favContainer.appendChild(addTile);

        // Рендерим вкладки
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

        // Omnibox логика и автодополнение в сайдбаре
        const omnibox = document.getElementById('aether-sb-omnibox');
        const sbAc = document.getElementById('aether-sb-ac-dropdown');

        if (omnibox) {
            const isStart = window.location.href.includes("start.html") || window.location.href.includes("aether://");
            omnibox.value = isStart ? "" : window.location.href;

            omnibox.oninput = () => {
                const q = omnibox.value.trim().toLowerCase();
                sbAc.innerHTML = '';
                if (!q) {
                    sbAc.style.display = 'none';
                    return;
                }

                const matches = [];
                const history = getHistory();
                history.filter(h => h.val.toLowerCase().includes(q)).slice(0, 3).forEach(h => {
                    matches.push({ text: h.val, target: h.val, icon: '🕒' });
                });

                const PRESETS = [
                    { name: 'YouTube', domain: 'youtube.com', url: 'https://www.youtube.com' },
                    { name: 'VK', domain: 'vk.com', url: 'https://vk.com' },
                    { name: 'Telegram', domain: 'web.telegram.org', url: 'https://web.telegram.org' },
                    { name: 'GitHub', domain: 'github.com', url: 'https://github.com' },
                    { name: 'Twitch', domain: 'twitch.tv', url: 'https://www.twitch.tv' },
                    { name: 'Кинопоиск', domain: 'kinopoisk.ru', url: 'https://www.kinopoisk.ru' }
                ];
                PRESETS.filter(p => p.domain.includes(q) || p.name.toLowerCase().includes(q)).slice(0, 3).forEach(p => {
                    matches.push({ text: p.name + ' (' + p.domain + ')', target: p.url, icon: '🌐' });
                });

                matches.push({ text: 'Искать: ' + omnibox.value.trim(), target: omnibox.value.trim(), icon: '🔍' });

                matches.forEach(m => {
                    const el = document.createElement('div');
                    el.className = 'aether-sb-ac-item';
                    el.innerHTML = `<span>${m.icon}</span><span class="aether-sb-ac-text">${m.text}</span>`;
                    el.onclick = () => {
                        handleNavigate(m.target);
                        sbAc.style.display = 'none';
                    };
                    sbAc.appendChild(el);
                });
                sbAc.style.display = 'block';
            };

            omnibox.onkeydown = (e) => {
                if (e.key === 'Enter') {
                    sbAc.style.display = 'none';
                    handleNavigate(omnibox.value.trim());
                } else if (e.key === 'Escape') {
                    sbAc.style.display = 'none';
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

        // Медиа-инструменты в доке
        document.getElementById('aether-sb-audio').onclick = () => toggleAudioMode();
        document.getElementById('aether-sb-pip').onclick = () => togglePiP();
    }

    // 6. МОДАЛЬНОЕ ОКНО ДОБАВЛЕНИЯ В САЙДБАРЕ
    function openSidebarAddModal() {
        let modal = document.getElementById('aether-sb-modal-backdrop');
        if (!modal) {
            modal = document.createElement('div');
            modal.id = 'aether-sb-modal-backdrop';
            safeSetHTML(modal, `
                <div id="aether-sb-modal">
                    <div style="font-weight:600; font-size:15px; display:flex; justify-content:space-between;">
                        <span>Добавить сервис</span>
                        <span id="aether-sb-modal-close" style="cursor:pointer; color:#94a3b8;">✕</span>
                    </div>
                    <input id="aether-sbm-name" class="aether-sb-modal-input" placeholder="Название (например, VK)" autocomplete="off" />
                    <input id="aether-sbm-url" class="aether-sb-modal-input" placeholder="URL адрес (например, vk.ru)" autocomplete="off" />
                    <button id="aether-sbm-submit" class="aether-sb-modal-btn">Добавить сервис</button>
                </div>
            `);
            document.documentElement.appendChild(modal);

            document.getElementById('aether-sb-modal-close').onclick = () => {
                modal.style.display = 'none';
            };
            document.getElementById('aether-sbm-submit').onclick = () => {
                const name = document.getElementById('aether-sbm-name').value.trim();
                let url = document.getElementById('aether-sbm-url').value.trim();
                if (!name || !url) return;
                if (!url.startsWith('http://') && !url.startsWith('https://')) {
                    url = 'https://' + url;
                }
                const cards = getCustomCards();
                cards.push({ id: Date.now().toString(), name, url });
                saveCustomCards(cards);
                modal.style.display = 'none';
            };
        }

        document.getElementById('aether-sbm-name').value = '';
        document.getElementById('aether-sbm-url').value = '';
        modal.style.display = 'flex';
        document.getElementById('aether-sbm-name').focus();
    }

    // 7. МЕНЮ ПКМ ПО ВИДЕО (Контекстное меню yt-dlp, PiP, Аудио)
    function setupVideoContextMenu() {
        let menu = document.getElementById('aether-video-menu');
        if (!menu) {
            menu = document.createElement('div');
            menu.id = 'aether-video-menu';
            safeSetHTML(menu, `
                <div class="aether-vm-header">🎬 Aether Медиа</div>
                <div class="aether-vm-item" id="aether-vmi-download">
                    <span>⬇️</span> <span>Скачать видео (yt-dlp)</span>
                </div>
                <div class="aether-vm-item" id="aether-vmi-pip">
                    <span>🔲</span> <span>Картинка в картинке</span>
                </div>
                <div class="aether-vm-item" id="aether-vmi-audio">
                    <span>🎧</span> <span>Режим «Только аудио»</span>
                </div>
                <div class="aether-vm-item" id="aether-vmi-speed">
                    <span>⚡</span> <span>Скорость 2x / 1x</span>
                </div>
                <div class="aether-vm-item" id="aether-vmi-copy">
                    <span>📋</span> <span>Скопировать ссылку</span>
                </div>
            `);
            document.documentElement.appendChild(menu);

            document.getElementById('aether-vmi-download').onclick = () => {
                menu.style.display = 'none';
                copyYtDlp();
            };
            document.getElementById('aether-vmi-pip').onclick = () => {
                menu.style.display = 'none';
                togglePiP();
            };
            document.getElementById('aether-vmi-audio').onclick = () => {
                menu.style.display = 'none';
                toggleAudioMode();
            };
            document.getElementById('aether-vmi-speed').onclick = () => {
                menu.style.display = 'none';
                toggleVideoSpeed();
            };
            document.getElementById('aether-vmi-copy').onclick = () => {
                menu.style.display = 'none';
                try {
                    navigator.clipboard.writeText(window.location.href);
                    showToast("📋 Ссылка на видео скопирована!");
                } catch (_) {}
            };
        }

        document.addEventListener('contextmenu', (e) => {
            if (e.shiftKey) return; // Shift + ПКМ открывает стандартное контекстное меню
            const isVideo = e.target.closest('video, .html5-video-player, ytd-player, #player, .video-stream');
            if (isVideo) {
                e.preventDefault();
                e.stopPropagation();
                menu.style.display = 'block';
                const x = Math.min(e.clientX, window.innerWidth - 220);
                const y = Math.min(e.clientY, window.innerHeight - 200);
                menu.style.left = x + 'px';
                menu.style.top = y + 'px';
            } else {
                menu.style.display = 'none';
            }
        }, true);

        document.addEventListener('click', (e) => {
            if (!menu.contains(e.target)) {
                menu.style.display = 'none';
            }
        });
    }

    setupVideoContextMenu();

    function toggleVideoSpeed() {
        const video = document.querySelector('video');
        if (video) {
            video.playbackRate = (video.playbackRate === 1.0) ? 2.0 : 1.0;
            showToast(`⚡ Скорость: ${video.playbackRate}x`);
        }
    }

    // 8. НАВИГАЦИЯ, ВКЛАДКИ И ПЕРЕХОД ДОМОЙ ЧЕРЕЗ RUST IPC
    function toggleSidebar(collapsed) {
        const state = getTabsState();
        state.sidebarCollapsed = (typeof collapsed === 'boolean') ? collapsed : !state.sidebarCollapsed;
        saveTabsState(state);
        renderSidebar();
    }

    function goHome() {
        const state = getTabsState();
        const startTab = state.tabs.find(t => t.url && (t.url.includes("start.html") || t.url.includes("aether://")));
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

        if (window.ipc && window.ipc.postMessage) {
            try {
                window.ipc.postMessage("go_home");
                return;
            } catch (_) {}
        }
        window.location.href = getStartPageUrl();
    }

    function createNewTab(url) {
        const state = getTabsState();
        const targetUrl = url || getStartPageUrl();
        const newId = state.nextTabId;
        state.nextTabId++;
        const title = (targetUrl.includes("start.html") || targetUrl.includes("aether://")) ? "Старт" : "Новая вкладка";
        state.tabs.push({ id: newId, title, url: targetUrl });
        state.activeTabId = newId;
        saveTabsState(state);

        if (targetUrl.includes("start.html") || targetUrl.includes("aether://")) {
            goHome();
            return;
        }
        if (window.ipc && window.ipc.postMessage) {
            try {
                window.ipc.postMessage("navigate:" + targetUrl);
                return;
            } catch (_) {}
        }
        window.location.href = targetUrl;
    }

    function switchTab(id) {
        const state = getTabsState();
        const tab = state.tabs.find(t => t.id === id);
        if (tab && tab.url) {
            state.activeTabId = id;
            saveTabsState(state);

            if (tab.url.includes("start.html") || tab.url.includes("aether://")) {
                goHome();
                return;
            }
            if (window.ipc && window.ipc.postMessage) {
                try {
                    window.ipc.postMessage("navigate:" + tab.url);
                    return;
                } catch (_) {}
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
                if (nextTab.url.includes("start.html") || nextTab.url.includes("aether://")) {
                    goHome();
                } else {
                    if (window.ipc && window.ipc.postMessage) {
                        try {
                            window.ipc.postMessage("navigate:" + nextTab.url);
                            return;
                        } catch (_) {}
                    }
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
            if (window.location.href.includes("start.html") || window.location.href.includes("aether://")) {
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
            tab.title = (url.includes("start.html") || url.includes("aether://")) ? "Старт" : "Загрузка...";
        }
        saveTabsState(state);

        if (url.includes("start.html") || url.includes("aether://")) {
            goHome();
        } else {
            if (window.ipc && window.ipc.postMessage) {
                try {
                    window.ipc.postMessage("navigate:" + url);
                    return;
                } catch (_) {}
            }
            window.location.href = url;
        }
    }

    function handleNavigate(input) {
        if (!input) return;
        saveHistoryItem(input, input.startsWith('http') ? 'url' : 'query');
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
        } else if (input.startsWith("http://") || input.startsWith("https://") || input.startsWith("aether://")) {
            targetUrl = input;
        } else if (input.includes(".") && !input.includes(" ") && !input.endsWith(".")) {
            targetUrl = "https://" + input;
        } else {
            targetUrl = "https://www.google.com/search?q=" + encodeURIComponent(input);
        }

        navigateToUrl(targetUrl);
    }

    // 9. МЕДИА-ФУНКЦИИ
    function toggleAudioMode() {
        const video = document.querySelector('video');
        if (video) {
            const isAudioOnly = video.style.opacity === '0.001';
            video.style.opacity = isAudioOnly ? '1' : '0.001';
            video.style.transform = isAudioOnly ? 'scale(1)' : 'scale(0.01)';
            showToast(isAudioOnly ? "📺 Режим видео включен" : "🎧 Режим Только аудио активирован");
        } else {
            showToast("⚠️ Видеоплеер не обнаружен");
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
        } else {
            showToast("⚠️ Видеоплеер не обнаружен");
        }
    }

    function copyYtDlp() {
        const url = window.location.href;
        const cmd = `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best" "${url}"`;
        try {
            navigator.clipboard.writeText(cmd);
            showToast("⬇️ Команда yt-dlp скопирована в буфер!");
        } catch (_) {
            prompt("Скопируйте команду:", cmd);
        }
    }

    // 10. YOUTUBE АВТО-ПРОПУСК РЕКЛАМЫ
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

    // 11. ГОРЯЧИЕ КЛАВИШИ БРАУЗЕРА
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
        } else if (ctrl && (e.key === 'p' || e.key === 'P' || e.key === 'з' || e.key === 'З')) {
            e.preventDefault();
            togglePiP();
        }
    });

    // 12. СЛЕЖЕНИЕ ЗА ИЗМЕНЕНИЕМ ЗАГОЛОВКА
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
