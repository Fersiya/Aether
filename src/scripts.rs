//! Встроенный рантайм Aether Browser v0.6.4 (Arc / Zen Style):
//! - Чистый веб-контент: нулевые модификации html/body/элементов сайтов — YouTube и VK работают на 100% без сдвигов и багов
//! - Плавающий авто-скрывающийся сайдбар Arc Style: открывается при наведении на левый край экрана, клике на значок или Ctrl+B
//! - Отдельное окно "Настройки" с полной историей поиска и переходов, очисткой и переключением режимов
//! - Меню ПКМ по видео: скачивание через yt-dlp, PiP, режим Только аудио, копирование ссылки
//! - Кнопка Домика гарантированно ведет на http://aether.localhost/ через Rust IPC
//! - Автоматический сбор логов в aether_debug.log на Рабочем столе для полной диагностики

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

        /* Кнопка настроек в правом верхнем углу */
        #aether-top-settings {
            position: absolute;
            top: 20px;
            right: 24px;
            display: flex;
            align-items: center;
            gap: 8px;
            background: rgba(22, 14, 42, 0.65);
            border: 1px solid rgba(168, 85, 247, 0.3);
            border-radius: 10px;
            padding: 8px 14px;
            color: #c084fc;
            font-size: 13px;
            cursor: pointer;
            transition: all 0.2s;
            backdrop-filter: blur(10px);
        }
        #aether-top-settings:hover {
            background: rgba(30, 18, 58, 0.95);
            border-color: #c084fc;
            color: #ffffff;
            box-shadow: 0 0 15px rgba(168, 85, 247, 0.4);
            transform: translateY(-1px);
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

        /* Поисковая строка */
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

        /* Выпадающий список популярных интернет-сервисов и быстрого поиска */
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
            gap: 12px;
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
        .aether-ac-icon { font-size: 16px; color: #c084fc; width: 22px; text-align: center; }
        .aether-ac-text { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1; }

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

        /* Модальное окно добавления карточки */
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
        }
        .aether-presets { display: flex; flex-wrap: wrap; gap: 6px; }
        .aether-preset-chip {
            background: rgba(168, 85, 247, 0.12);
            border: 1px solid rgba(168, 85, 247, 0.25);
            border-radius: 6px;
            padding: 4px 9px;
            font-size: 11.5px;
            color: #d8b4fe;
            cursor: pointer;
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
        }
        .aether-btn-submit {
            background: linear-gradient(135deg, #9333ea, #7c3aed);
            border: none;
            color: #ffffff;
            padding: 8px 18px;
            border-radius: 8px;
            font-size: 13px;
            font-weight: 500;
            cursor: pointer;
        }

        /* Окно НАСТРОЕК с Историей */
        #aether-settings-modal {
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: rgba(0, 0, 0, 0.75);
            backdrop-filter: blur(10px);
            z-index: 2147483647;
            display: none;
            align-items: center;
            justify-content: center;
        }
        #aether-settings-box {
            background: #0f081e;
            border: 1px solid rgba(168, 85, 247, 0.45);
            box-shadow: 0 25px 70px rgba(0, 0, 0, 0.85), 0 0 35px rgba(168, 85, 247, 0.3);
            border-radius: 20px;
            width: 100%;
            max-width: 600px;
            height: 520px;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
        .aether-settings-header {
            padding: 16px 22px;
            background: rgba(20, 12, 38, 0.8);
            border-bottom: 1px solid rgba(168, 85, 247, 0.25);
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .aether-settings-nav {
            display: flex;
            gap: 12px;
            padding: 12px 22px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.06);
        }
        .aether-s-tab {
            padding: 6px 14px;
            border-radius: 8px;
            font-size: 13px;
            cursor: pointer;
            color: #cbd5e1;
            transition: all 0.15s;
        }
        .aether-s-tab.active {
            background: rgba(147, 51, 234, 0.35);
            border: 1px solid rgba(168, 85, 247, 0.4);
            color: #ffffff;
            font-weight: 500;
        }
        .aether-settings-content {
            flex: 1;
            overflow-y: auto;
            padding: 18px 22px;
        }
        .aether-history-item {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 9px 12px;
            border-radius: 8px;
            background: rgba(255, 255, 255, 0.03);
            border: 1px solid rgba(255, 255, 255, 0.05);
            margin-bottom: 6px;
            font-size: 13px;
            cursor: pointer;
            transition: background 0.15s;
        }
        .aether-history-item:hover {
            background: rgba(147, 51, 234, 0.25);
        }
        .aether-history-del {
            color: #94a3b8;
            padding: 2px 6px;
            border-radius: 4px;
        }
        .aether-history-del:hover { color: #fca5a5; background: rgba(239, 68, 68, 0.25); }
    </style>
</head>
<body>
    <button id="aether-top-settings" onclick="openSettings('history')">
        <span>⚙️</span> <span>Настройки и История</span>
    </button>

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

    <!-- Окно НАСТРОЕК И ИСТОРИИ -->
    <div id="aether-settings-modal">
        <div id="aether-settings-box">
            <div class="aether-settings-header">
                <div style="font-weight:600; font-size:16px; display:flex; align-items:center; gap:8px;">
                    <span>⚙️</span> <span>Настройки Aether Browser</span>
                </div>
                <span onclick="closeSettings()" style="cursor:pointer; color:#94a3b8; font-size:16px;">✕</span>
            </div>
            <div class="aether-settings-nav">
                <span class="aether-s-tab active" id="tab-btn-history" onclick="switchSettingsTab('history')">🕒 История</span>
                <span class="aether-s-tab" id="tab-btn-ui" onclick="switchSettingsTab('ui')">🎛️ Боковая панель</span>
                <span class="aether-s-tab" id="tab-btn-diag" onclick="switchSettingsTab('diag')">📁 Логи и Диагностика</span>
            </div>
            <div class="aether-settings-content" id="settings-content-area"></div>
        </div>
    </div>

    <script>
        function log(msg) {
            if (window.ipc && window.ipc.postMessage) {
                try { window.ipc.postMessage("log:" + msg); } catch (_) {}
            }
        }
        log("Стартовая страница загружена (http://aether.localhost/)");

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
            { name: 'ChatGPT', domain: 'chatgpt.com', url: 'https://chatgpt.com', icon: '💬' }
        ];

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
                if (list.length > 100) list.pop();
                localStorage.setItem('aether_search_history', JSON.stringify(list));
            } catch (_) {}
        }

        function deleteHistoryItem(val) {
            try {
                let list = getHistory();
                list = list.filter(item => item.val.toLowerCase() !== val.toLowerCase());
                localStorage.setItem('aether_search_history', JSON.stringify(list));
                renderHistoryTab();
            } catch (_) {}
        }

        function clearAllHistory() {
            localStorage.removeItem('aether_search_history');
            renderHistoryTab();
        }

        function navigateTo(target) {
            if (!target) return;
            let url = target.trim();
            saveHistoryItem(url, url.startsWith('http') ? 'url' : 'query');

            let dest = url;
            if (url.toLowerCase().startsWith("yt:") || url.toLowerCase().startsWith("ютуб:")) {
                dest = "https://www.youtube.com/results?search_query=" + encodeURIComponent(url.replace(/^(yt:|ютуб:)/i, "").trim());
            } else if (url.toLowerCase().startsWith("vk:") || url.toLowerCase().startsWith("вк:")) {
                dest = "https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D=" + encodeURIComponent(url.replace(/^(vk:|вк:)/i, "").trim());
            } else if (url.toLowerCase().startsWith("gh:")) {
                dest = "https://github.com/search?q=" + encodeURIComponent(url.replace(/^gh:/i, "").trim());
            } else {
                const isUrl = url.startsWith("http://") || url.startsWith("https://") || (url.includes(".") && !url.includes(" ") && !url.endsWith("."));
                if (isUrl) {
                    dest = (url.startsWith("http://") || url.startsWith("https://")) ? url : "https://" + url;
                } else {
                    dest = "https://www.google.com/search?q=" + encodeURIComponent(url);
                }
            }

            log("Стартовая страница навигация к: " + dest);
            if (window.ipc && window.ipc.postMessage) {
                try {
                    window.ipc.postMessage("navigate:" + dest);
                    return;
                } catch (_) {}
            }
            window.location.href = dest;
        }

        function handleSearchSubmit(e) {
            e.preventDefault();
            navigateTo(document.getElementById('aether-search-input').value);
        }

        // Автодополнение популярных сайтов
        const searchInput = document.getElementById('aether-search-input');
        const acDropdown = document.getElementById('aether-ac-dropdown');

        searchInput.addEventListener('input', () => {
            const val = searchInput.value.trim().toLowerCase();
            acDropdown.innerHTML = '';
            if (!val) {
                acDropdown.style.display = 'none';
                return;
            }

            const items = [];
            POPULAR_SITES.filter(s => s.domain.includes(val) || s.name.toLowerCase().includes(val)).slice(0, 4).forEach(s => {
                items.push({ text: s.name + ' (' + s.domain + ')', target: s.url, icon: s.icon });
            });
            items.push({ text: `Искать в Google: "${searchInput.value.trim()}"`, target: searchInput.value.trim(), icon: '🔍' });
            items.push({ text: `Искать на YouTube: "${searchInput.value.trim()}"`, target: "yt: " + searchInput.value.trim(), icon: '▶' });

            items.forEach(item => {
                const el = document.createElement('div');
                el.className = 'aether-ac-item';
                el.innerHTML = `<span class="aether-ac-icon">${item.icon}</span><span class="aether-ac-text">${item.text}</span>`;
                el.onclick = () => { navigateTo(item.target); };
                acDropdown.appendChild(el);
            });
            acDropdown.style.display = 'block';
        });

        document.addEventListener('click', (e) => {
            if (!document.getElementById('aether-search-container').contains(e.target)) {
                acDropdown.style.display = 'none';
            }
        });

        // --- Управление карточками ---
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
            return [];
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
                try { window.ipc.postMessage("save_cards:" + JSON.stringify(cards)); } catch (_) {}
            }
            renderGrid();
        }

        function renderGrid() {
            const grid = document.getElementById('aether-grid');
            grid.innerHTML = '';
            const cards = getCustomCards();

            // Кнопка добавления: всегда слева от добавленных карточек
            const addBtn = document.createElement('div');
            addBtn.className = 'aether-card-add';
            addBtn.onclick = openAddModal;
            addBtn.title = 'Добавить быстрый сервис';
            addBtn.innerHTML = `
                <div class="aether-card-add-icon">+</div>
                <div class="aether-card-add-name">Добавить сервис</div>
            `;
            grid.appendChild(addBtn);

            cards.forEach((card, index) => {
                const el = document.createElement('div');
                el.className = 'aether-card';
                el.onclick = () => { navigateTo(card.url); };

                const icon = document.createElement('div');
                icon.className = 'aether-card-icon';
                icon.textContent = (card.name || 'S').trim().charAt(0).toUpperCase();

                const name = document.createElement('div');
                name.className = 'aether-card-name';
                name.textContent = card.name;

                const del = document.createElement('div');
                del.className = 'aether-card-del';
                del.textContent = '✕';
                del.title = 'Удалить';
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
        function closeAddModal() { document.getElementById('aether-modal-backdrop').style.display = 'none'; }
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

        // --- Окно настроек и истории ---
        function openSettings(tab = 'history') {
            document.getElementById('aether-settings-modal').style.display = 'flex';
            switchSettingsTab(tab);
        }
        function closeSettings() {
            document.getElementById('aether-settings-modal').style.display = 'none';
        }

        function switchSettingsTab(tab) {
            document.querySelectorAll('.aether-s-tab').forEach(el => el.classList.remove('active'));
            const tabBtn = document.getElementById('tab-btn-' + tab);
            if (tabBtn) tabBtn.classList.add('active');

            if (tab === 'history') renderHistoryTab();
            else if (tab === 'ui') renderUiTab();
            else if (tab === 'diag') renderDiagTab();
        }

        function renderHistoryTab() {
            const area = document.getElementById('settings-content-area');
            const history = getHistory();

            if (history.length === 0) {
                area.innerHTML = `
                    <div style="text-align:center; padding: 40px; color:#94a3b8;">
                        <div style="font-size:32px; margin-bottom:10px;">🕒</div>
                        <div>История поиска и переходов пуста</div>
                    </div>
                `;
                return;
            }

            let html = `
                <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:14px;">
                    <span style="font-size:13px; color:#cbd5e1;">Всего записей: ${history.length}</span>
                    <button onclick="clearAllHistory()" style="background:rgba(239,68,68,0.2); border:1px solid rgba(239,68,68,0.4); color:#fca5a5; padding:4px 10px; border-radius:6px; cursor:pointer; font-size:12px;">🗑️ Очистить всю историю</button>
                </div>
            `;

            history.forEach(item => {
                const date = new Date(item.ts).toLocaleString('ru-RU', { hour: '2-digit', minute: '2-digit', day: 'numeric', month: 'short' });
                html += `
                    <div class="aether-history-item" onclick="navigateTo('${item.val.replace(/'/g, "\\'")}')">
                        <div style="display:flex; align-items:center; gap:8px; overflow:hidden;">
                            <span>${item.type === 'url' ? '🌐' : '🔍'}</span>
                            <span style="overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">${item.val}</span>
                        </div>
                        <div style="display:flex; align-items:center; gap:10px; flex-shrink:0;">
                            <span style="font-size:11px; color:#94a3b8;">${date}</span>
                            <span class="aether-history-del" onclick="event.stopPropagation(); deleteHistoryItem('${item.val.replace(/'/g, "\\'")}')">✕</span>
                        </div>
                    </div>
                `;
            });
            area.innerHTML = html;
        }

        function renderUiTab() {
            const area = document.getElementById('settings-content-area');
            const currentMode = localStorage.getItem('aether_sidebar_mode') || 'autohide';
            area.innerHTML = `
                <div style="display:flex; flex-direction:column; gap:16px;">
                    <div>
                        <div style="font-weight:600; font-size:14px; margin-bottom:6px;">Режим боковой панели (Arc Style)</div>
                        <div style="font-size:12.5px; color:#94a3b8; margin-bottom:12px;">Выберите, как боковая панель ведёт себя на открытых сайтах:</div>
                    </div>
                    <label style="display:flex; align-items:flex-start; gap:10px; cursor:pointer; background:rgba(255,255,255,0.03); padding:12px; border-radius:8px; border:1px solid rgba(255,255,255,0.08);">
                        <input type="radio" name="sb_mode" value="autohide" ${currentMode === 'autohide' ? 'checked' : ''} onchange="setSidebarMode('autohide')" />
                        <div>
                            <div style="font-weight:500; color:#fff;">✨ Авто-скрытие (Floating / Arc Style) — Рекомендуется</div>
                            <div style="font-size:12px; color:#94a3b8; margin-top:2px;">Сайт отображается на все 100% экрана, ничего не сдвигается и не перекрывается. Панель плавно выезжает поверх экрана при наведении на левый край или по Ctrl+B.</div>
                        </div>
                    </label>
                    <label style="display:flex; align-items:flex-start; gap:10px; cursor:pointer; background:rgba(255,255,255,0.03); padding:12px; border-radius:8px; border:1px solid rgba(255,255,255,0.08);">
                        <input type="radio" name="sb_mode" value="pinned" ${currentMode === 'pinned' ? 'checked' : ''} onchange="setSidebarMode('pinned')" />
                        <div>
                            <div style="font-weight:500; color:#fff;">📌 Закрепленная панель (Pinned)</div>
                            <div style="font-size:12px; color:#94a3b8; margin-top:2px;">Панель всегда видна слева на экране.</div>
                        </div>
                    </label>
                </div>
            `;
        }

        function setSidebarMode(mode) {
            localStorage.setItem('aether_sidebar_mode', mode);
            log("Изменен режим сайдбара: " + mode);
        }

        function renderDiagTab() {
            const area = document.getElementById('settings-content-area');
            area.innerHTML = `
                <div style="display:flex; flex-direction:column; gap:14px;">
                    <div style="font-weight:600; font-size:14px;">📁 Диагностика и логирование</div>
                    <div style="font-size:13px; color:#cbd5e1; line-height:1.5;">
                        Все события, переходы, клики по кнопке «Домой» и ошибки автоматически записываются в реальном времени в файл лога на вашем Рабочем столе:
                    </div>
                    <div style="background:rgba(20,12,38,0.9); border:1px solid rgba(168,85,247,0.3); border-radius:8px; padding:10px 14px; font-family:monospace; font-size:12px; color:#c084fc;">
                        %USERPROFILE%\\Desktop\\aether_debug.log
                    </div>
                    <div style="font-size:12px; color:#94a3b8;">
                        Если вы сталкиваетесь с проблемой, вы можете просто открыть этот файл с Рабочего стола и отправить его текст.
                    </div>
                </div>
            `;
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

    function logToRust(msg) {
        if (window.ipc && window.ipc.postMessage) {
            try { window.ipc.postMessage("log:" + msg); } catch (_) {}
        }
    }
    logToRust("Core script запущен на: " + window.location.href);

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
        return "http://aether.localhost/";
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
        return [];
    }

    function saveCustomCards(cards) {
        try { localStorage.setItem('aether_custom_cards', JSON.stringify(cards)); } catch (_) {}
        const state = getTabsState();
        state.cards = cards;
        saveTabsState(state);
        if (window.ipc && window.ipc.postMessage) {
            try { window.ipc.postMessage("save_cards:" + JSON.stringify(cards)); } catch (_) {}
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
        const initialTitle = (startUrl.includes("aether.localhost") || startUrl.includes("start.html")) ? "Старт" : (document.title || "Вкладка");
        return {
            activeTabId: 1,
            nextTabId: 2,
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
        const curTitle = (curUrl.includes("aether.localhost") || curUrl.includes("start.html")) ? "Старт" : (document.title || (location.hostname.replace("www.", "") || "Вкладка"));

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

    // 3. ПЛАВАЮЩИЙ САЙДБАР ARC: 100% ЧИСТЫЙ САЙТ БЕЗ ИЗМЕНЕНИЙ ВЕРСТКИ
    // Важно: Мы НЕ МЕНЯЕМ html { margin-left }, #guide, #masthead-container или другие элементы сайта!
    const SIDEBAR_CSS = `
        /* Кнопка-триггер в левом верхнем углу сайта */
        #aether-trigger-pill {
            position: fixed !important;
            top: 10px !important;
            left: 8px !important;
            width: 32px !important;
            height: 32px !important;
            border-radius: 9px !important;
            background: rgba(18, 12, 33, 0.88) !important;
            backdrop-filter: blur(12px) !important;
            border: 1px solid rgba(168, 85, 247, 0.45) !important;
            color: #c084fc !important;
            display: flex !important;
            align-items: center !important;
            justify-content: center !important;
            cursor: pointer !important;
            z-index: 2147483645 !important;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6), 0 0 10px rgba(168, 85, 247, 0.3) !important;
            transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1) !important;
            font-size: 15px !important;
            user-select: none !important;
        }
        #aether-trigger-pill:hover {
            background: rgba(32, 18, 62, 0.98) !important;
            color: #ffffff !important;
            border-color: #c084fc !important;
            transform: scale(1.08) !important;
            box-shadow: 0 6px 20px rgba(168, 85, 247, 0.5) !important;
        }

        /* Невидимая полоса детекции наведения у левого края (8px) */
        #aether-hover-edge {
            position: fixed !important;
            top: 0 !important;
            left: 0 !important;
            width: 8px !important;
            height: 100vh !important;
            z-index: 2147483644 !important;
            background: transparent !important;
        }

        /* Сам плавающий сайдбар */
        #aether-arc-sidebar {
            position: fixed !important;
            top: 0 !important;
            left: 0 !important;
            width: 185px !important;
            height: 100vh !important;
            background: rgba(12, 7, 24, 0.96) !important;
            backdrop-filter: blur(24px) !important;
            border-right: 1px solid rgba(168, 85, 247, 0.35) !important;
            box-shadow: 6px 0 32px rgba(0, 0, 0, 0.8), 0 0 20px rgba(168, 85, 247, 0.25) !important;
            z-index: 2147483646 !important;
            display: flex !important;
            flex-direction: column !important;
            padding: 10px 8px 8px !important;
            box-sizing: border-box !important;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
            user-select: none !important;
            color: #f8fafc !important;
            transform: translateX(-100%) !important;
            transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1) !important;
        }
        #aether-arc-sidebar.open {
            transform: translateX(0) !important;
        }

        .aether-sb-header {
            display: flex !important;
            align-items: center !important;
            justify-content: space-between !important;
            padding: 2px 3px 8px !important;
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
        .aether-sb-btn-box {
            display: flex !important;
            align-items: center !important;
            gap: 4px !important;
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
        }

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

        /* Меню ПКМ по видео */
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
        if (u.includes('aether.localhost') || u.includes('start.html')) return '<span style="color:#c084fc; font-size:10px;">🌌</span>';
        return '<span style="color:#94a3b8; font-size:10px;">🌐</span>';
    }

    // 4. ПОСТРОЕНИЕ САЙДБАРА
    let isSidebarOpen = false;

    function openSidebar() {
        isSidebarOpen = true;
        const sidebar = document.getElementById('aether-arc-sidebar');
        if (sidebar) sidebar.classList.add('open');
    }

    function closeSidebar() {
        isSidebarOpen = false;
        const sidebar = document.getElementById('aether-arc-sidebar');
        if (sidebar) sidebar.classList.remove('open');
    }

    function toggleSidebar() {
        if (isSidebarOpen) closeSidebar();
        else openSidebar();
    }

    function renderSidebar() {
        injectStyle();

        // 1. Кнопка-триггер в левом верхнем углу
        let triggerPill = document.getElementById('aether-trigger-pill');
        if (!triggerPill) {
            triggerPill = document.createElement('div');
            triggerPill.id = 'aether-trigger-pill';
            triggerPill.title = 'Открыть меню Aether (Ctrl + B)';
            triggerPill.textContent = '🌌';
            triggerPill.onclick = (e) => {
                e.stopPropagation();
                toggleSidebar();
            };
            document.documentElement.appendChild(triggerPill);
        }

        // 2. Детектор наведения мыши у левого края
        let hoverEdge = document.getElementById('aether-hover-edge');
        if (!hoverEdge) {
            hoverEdge = document.createElement('div');
            hoverEdge.id = 'aether-hover-edge';
            hoverEdge.addEventListener('mouseenter', openSidebar);
            document.documentElement.appendChild(hoverEdge);
        }

        // 3. Плавающий сайдбар
        let sidebar = document.getElementById('aether-arc-sidebar');
        if (!sidebar) {
            sidebar = document.createElement('div');
            sidebar.id = 'aether-arc-sidebar';
            sidebar.addEventListener('mouseleave', () => {
                closeSidebar();
            });
            document.documentElement.appendChild(sidebar);
        }

        const state = getTabsState();

        safeSetHTML(sidebar, `
            <div class="aether-sb-header">
                <div class="aether-sb-brand">
                    <span class="aether-sb-brand-icon">🌌</span>
                    <span>AETHER</span>
                </div>
                <div class="aether-sb-btn-box">
                    <button id="aether-sb-close-btn" class="aether-sb-tool-btn" title="Закрыть сайдбар (Ctrl + B)">
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
                </div>
            </div>

            <div class="aether-sb-favorites" id="aether-sb-favorites"></div>

            <div class="aether-sb-tabs-header">
                <span id="aether-sb-tabs-counter">Вкладки (${state.tabs.length})</span>
                <button id="aether-sb-new-tab" class="aether-sb-new-tab-btn" title="Новая вкладка (Ctrl + T)">+</button>
            </div>

            <div id="aether-sb-tabs-list" class="aether-sb-tabs-list"></div>

            <div class="aether-sb-dock">
                <button id="aether-sb-audio" class="aether-sb-dock-btn" title="Только аудио (Ctrl + M)">
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

        // Рендерим быстрые карточки в сайдбаре
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

            tile.onclick = () => {
                closeSidebar();
                openService(card.url);
            };
            favContainer.appendChild(tile);
        });

        // Вкладки
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
                closeSidebar();
                if (tab.id !== state.activeTabId) {
                    switchTab(tab.id);
                }
            };

            item.appendChild(badge);
            item.appendChild(title);
            item.appendChild(closeBtn);
            tabsContainer.appendChild(item);
        });

        const omnibox = document.getElementById('aether-sb-omnibox');
        if (omnibox) {
            const isStart = window.location.href.includes("aether.localhost") || window.location.href.includes("start.html");
            omnibox.value = isStart ? "" : window.location.href;
            omnibox.onkeydown = (e) => {
                if (e.key === 'Enter') {
                    closeSidebar();
                    handleNavigate(omnibox.value.trim());
                } else if (e.key === 'Escape') {
                    closeSidebar();
                }
            };
        }

        document.getElementById('aether-sb-back').onclick = () => window.history.back();
        document.getElementById('aether-sb-forward').onclick = () => window.history.forward();
        document.getElementById('aether-sb-reload').onclick = () => window.location.reload();
        document.getElementById('aether-sb-close-btn').onclick = closeSidebar;
        document.getElementById('aether-sb-new-tab').onclick = () => { closeSidebar(); createNewTab(); };
        document.getElementById('aether-sb-home').onclick = () => { closeSidebar(); goHome(); };
        document.getElementById('aether-sb-audio').onclick = toggleAudioMode;
        document.getElementById('aether-sb-pip').onclick = togglePiP;
    }

    // 5. КОНТЕКСТНОЕ МЕНЮ ПКМ ПО ВИДЕО
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

            document.getElementById('aether-vmi-download').onclick = () => { menu.style.display = 'none'; copyYtDlp(); };
            document.getElementById('aether-vmi-pip').onclick = () => { menu.style.display = 'none'; togglePiP(); };
            document.getElementById('aether-vmi-audio').onclick = () => { menu.style.display = 'none'; toggleAudioMode(); };
            document.getElementById('aether-vmi-speed').onclick = () => { menu.style.display = 'none'; toggleVideoSpeed(); };
            document.getElementById('aether-vmi-copy').onclick = () => {
                menu.style.display = 'none';
                try {
                    navigator.clipboard.writeText(window.location.href);
                    showToast("📋 Ссылка на видео скопирована!");
                } catch (_) {}
            };
        }

        document.addEventListener('contextmenu', (e) => {
            if (e.shiftKey) return;
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
            if (!menu.contains(e.target)) menu.style.display = 'none';
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

    // 6. НАВИГАЦИЯ, ВКЛАДКИ И ПЕРЕХОД ДОМОЙ
    function goHome() {
        logToRust("goHome() вызван. Текущий адрес: " + window.location.href);
        const state = getTabsState();
        const startTab = state.tabs.find(t => t.url && (t.url.includes("aether.localhost") || t.url.includes("start.html")));
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
            } catch (err) {
                logToRust("IPC go_home ошибка: " + err);
            }
        }
        window.location.href = getStartPageUrl();
    }

    function createNewTab(url) {
        const state = getTabsState();
        const targetUrl = url || getStartPageUrl();
        const newId = state.nextTabId;
        state.nextTabId++;
        const title = (targetUrl.includes("aether.localhost") || targetUrl.includes("start.html")) ? "Старт" : "Новая вкладка";
        state.tabs.push({ id: newId, title, url: targetUrl });
        state.activeTabId = newId;
        saveTabsState(state);

        if (targetUrl.includes("aether.localhost") || targetUrl.includes("start.html")) {
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

            if (tab.url.includes("aether.localhost") || tab.url.includes("start.html")) {
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
                if (nextTab.url.includes("aether.localhost") || nextTab.url.includes("start.html")) {
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
            if (window.location.href.includes("aether.localhost") || window.location.href.includes("start.html")) {
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
            tab.title = (url.includes("aether.localhost") || url.includes("start.html")) ? "Старт" : "Загрузка...";
        }
        saveTabsState(state);

        if (url.includes("aether.localhost") || url.includes("start.html")) {
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
        let targetUrl = "";
        const lower = input.toLowerCase();

        if (lower.startsWith("yt:") || lower.startsWith("ютуб:")) {
            targetUrl = "https://www.youtube.com/results?search_query=" + encodeURIComponent(input.replace(/^(yt:|ютуб:)/i, "").trim());
        } else if (lower.startsWith("vk:") || lower.startsWith("вк:")) {
            targetUrl = "https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D=" + encodeURIComponent(input.replace(/^(vk:|вк:)/i, "").trim());
        } else if (lower.startsWith("gh:")) {
            targetUrl = "https://github.com/search?q=" + encodeURIComponent(input.replace(/^gh:/i, "").trim());
        } else if (input.startsWith("http://") || input.startsWith("https://") || input.startsWith("aether://")) {
            targetUrl = input;
        } else if (input.includes(".") && !input.includes(" ") && !input.endsWith(".")) {
            targetUrl = "https://" + input;
        } else {
            targetUrl = "https://www.google.com/search?q=" + encodeURIComponent(input);
        }

        navigateToUrl(targetUrl);
    }

    // 7. МЕДИА-ФУНКЦИИ
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

    // 8. YOUTUBE АВТО-ПРОПУСК РЕКЛАМЫ
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

    // 9. ГОРЯЧИЕ КЛАВИШИ БРАУЗЕРА
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
            openSidebar();
            const omnibox = document.getElementById('aether-sb-omnibox');
            if (omnibox) {
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
