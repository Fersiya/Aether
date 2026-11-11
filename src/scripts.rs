//! Встроенный рантайм Aether Browser v0.6.0 (Arc / Zen Style):
//! - Мощный вертикальный сайдбар слева (боковые вкладки, быстрые сервисы, омнибокс)
//! - Чистый полноэкранный веб-контент справа без навязчивых оверлеев
//! - Настоящие независимые вкладки в памяти без перезагрузки страниц

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
        <span class="aether-hint-pill"><b>Ctrl + B</b> Скрыть сайдбар</span>
        <span class="aether-hint-pill"><b>Ctrl + Tab</b> Переключить вкладку</span>
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

pub const SIDEBAR_HTML: &str = r###"<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aether Sidebar</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
        body {
            height: 100vh;
            background: linear-gradient(180deg, #090514 0%, #0d071a 50%, #070310 100%);
            border-right: 1px solid rgba(168, 85, 247, 0.22);
            color: #f8fafc;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            display: flex;
            flex-direction: column;
            overflow: hidden;
            padding: 14px 10px 12px;
        }

        /* 1. Header */
        .aether-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 2px 6px 12px;
        }
        .aether-brand {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 15px;
            font-weight: 700;
            letter-spacing: 2px;
            background: linear-gradient(135deg, #f8fafc 20%, #c084fc 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        .aether-brand-icon {
            font-size: 16px;
            filter: drop-shadow(0 0 10px rgba(168, 85, 247, 0.7));
            -webkit-text-fill-color: initial;
        }
        .aether-toggle-btn {
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(168, 85, 247, 0.25);
            border-radius: 6px;
            width: 26px;
            height: 26px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #c084fc;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .aether-toggle-btn:hover {
            background: rgba(168, 85, 247, 0.25);
            color: #ffffff;
            box-shadow: 0 0 10px rgba(168, 85, 247, 0.4);
        }

        /* 2. Omnibox & Nav */
        .aether-nav-row {
            display: flex;
            align-items: center;
            gap: 4px;
            margin-bottom: 8px;
            padding: 0 2px;
        }
        .aether-nav-btn {
            width: 26px;
            height: 26px;
            border-radius: 6px;
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.06);
            display: flex;
            align-items: center;
            justify-content: center;
            color: #94a3b8;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .aether-nav-btn:hover {
            background: rgba(168, 85, 247, 0.2);
            color: #ffffff;
            border-color: rgba(168, 85, 247, 0.35);
        }
        .aether-omnibox-wrap {
            flex: 1;
            position: relative;
            display: flex;
            align-items: center;
        }
        .aether-omnibox-input {
            width: 100%;
            background: rgba(20, 12, 38, 0.85);
            border: 1px solid rgba(168, 85, 247, 0.35);
            border-radius: 8px;
            padding: 5px 8px 5px 24px;
            font-size: 12px;
            color: #ffffff;
            outline: none;
            transition: all 0.2s ease;
        }
        .aether-omnibox-input:focus {
            border-color: #c084fc;
            box-shadow: 0 0 12px rgba(168, 85, 247, 0.35);
            background: rgba(28, 16, 52, 0.95);
        }
        .aether-omnibox-input::placeholder { color: #64748b; }
        .aether-omnibox-icon {
            position: absolute;
            left: 7px;
            font-size: 11px;
            color: #a855f7;
            pointer-events: none;
        }

        /* 3. Favorites / Pinned Dock */
        .aether-favorites {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 6px;
            margin-bottom: 14px;
            padding: 0 2px;
        }
        .aether-fav-tile {
            height: 40px;
            background: rgba(22, 14, 42, 0.65);
            border: 1px solid rgba(168, 85, 247, 0.22);
            border-radius: 9px;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            transition: all 0.18s cubic-bezier(0.16, 1, 0.3, 1);
            position: relative;
        }
        .aether-fav-tile:hover {
            background: rgba(36, 20, 68, 0.95);
            border-color: rgba(192, 132, 252, 0.6);
            transform: translateY(-2px);
            box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4), 0 0 12px rgba(168, 85, 247, 0.35);
        }
        .aether-fav-active-dot {
            position: absolute;
            bottom: 3px;
            width: 4px;
            height: 4px;
            border-radius: 50%;
            background: #c084fc;
            box-shadow: 0 0 6px #c084fc;
            display: none;
        }

        /* 4. Tabs Section */
        .aether-tabs-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 6px 6px;
            font-size: 11px;
            font-weight: 600;
            color: #94a3b8;
            letter-spacing: 0.8px;
            text-transform: uppercase;
        }
        .aether-new-tab-btn {
            background: rgba(147, 51, 234, 0.25);
            border: 1px solid rgba(168, 85, 247, 0.35);
            border-radius: 5px;
            width: 20px;
            height: 20px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #d8b4fe;
            font-size: 13px;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .aether-new-tab-btn:hover {
            background: rgba(168, 85, 247, 0.5);
            color: #ffffff;
            box-shadow: 0 0 8px rgba(168, 85, 247, 0.5);
            transform: scale(1.08);
        }

        .aether-tabs-list {
            flex: 1;
            overflow-y: auto;
            overflow-x: hidden;
            display: flex;
            flex-direction: column;
            gap: 4px;
            padding: 2px;
        }
        .aether-tabs-list::-webkit-scrollbar { width: 3px; }
        .aether-tabs-list::-webkit-scrollbar-thumb { background: rgba(168, 85, 247, 0.3); border-radius: 3px; }

        .aether-tab-item {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 6px 9px;
            border-radius: 8px;
            font-size: 12.5px;
            color: #cbd5e1;
            cursor: pointer;
            border: 1px solid transparent;
            transition: all 0.15s ease;
        }
        .aether-tab-item:hover {
            background: rgba(255, 255, 255, 0.05);
            color: #ffffff;
        }
        .aether-tab-item.active {
            background: linear-gradient(135deg, rgba(147, 51, 234, 0.35), rgba(124, 58, 237, 0.2));
            border-color: rgba(192, 132, 252, 0.55);
            color: #ffffff;
            font-weight: 500;
            box-shadow: 0 4px 14px rgba(0, 0, 0, 0.3), 0 0 10px rgba(168, 85, 247, 0.2);
        }
        .aether-tab-badge {
            width: 16px;
            height: 16px;
            flex-shrink: 0;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
        }
        .aether-tab-title {
            flex: 1;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .aether-tab-close {
            opacity: 0;
            font-size: 12px;
            color: #94a3b8;
            padding: 2px 4px;
            border-radius: 4px;
            transition: all 0.12s;
        }
        .aether-tab-item:hover .aether-tab-close {
            opacity: 0.75;
        }
        .aether-tab-close:hover {
            opacity: 1 !important;
            color: #fca5a5;
            background: rgba(239, 68, 68, 0.25);
        }

        /* 5. Bottom Utilities Dock */
        .aether-dock {
            margin-top: auto;
            padding-top: 10px;
            border-top: 1px solid rgba(255, 255, 255, 0.08);
            display: flex;
            align-items: center;
            justify-content: space-around;
        }
        .aether-dock-btn {
            width: 32px;
            height: 32px;
            border-radius: 8px;
            background: rgba(255, 255, 255, 0.04);
            border: 1px solid rgba(255, 255, 255, 0.06);
            display: flex;
            align-items: center;
            justify-content: center;
            color: #94a3b8;
            cursor: pointer;
            transition: all 0.15s ease;
        }
        .aether-dock-btn:hover {
            background: rgba(168, 85, 247, 0.25);
            color: #d8b4fe;
            border-color: rgba(168, 85, 247, 0.45);
            box-shadow: 0 0 10px rgba(168, 85, 247, 0.3);
            transform: translateY(-1px);
        }
    </style>
</head>
<body>
    <!-- 1. Header -->
    <div class="aether-header">
        <div class="aether-brand">
            <span class="aether-brand-icon">🌌</span>
            <span>AETHER</span>
        </div>
        <button id="btn-toggle-sidebar" class="aether-toggle-btn" title="Свернуть / развернуть (Ctrl + B)">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="9" y1="3" x2="9" y2="21"/></svg>
        </button>
    </div>

    <!-- 2. Omnibox & Navigation -->
    <div class="aether-nav-row">
        <button id="btn-back" class="aether-nav-btn" title="Назад (Alt + ←)">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
        </button>
        <button id="btn-forward" class="aether-nav-btn" title="Вперед (Alt + →)">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
        </button>
        <button id="btn-reload" class="aether-nav-btn" title="Обновить (F5)">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/></svg>
        </button>
        <div class="aether-omnibox-wrap">
            <span class="aether-omnibox-icon">🔍</span>
            <input id="omnibox-input" class="aether-omnibox-input" type="text" placeholder="Поиск или URL..." autocomplete="off" />
        </div>
    </div>

    <!-- 3. Favorites (Pinned Apps) -->
    <div class="aether-favorites">
        <div class="aether-fav-tile" title="YouTube" onclick="openFavorite('https://www.youtube.com')">
            <svg viewBox="0 0 24 24" width="20" height="20" fill="#ff0000"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>
            <div id="dot-yt" class="aether-fav-active-dot"></div>
        </div>
        <div class="aether-fav-tile" title="ВКонтакте" onclick="openFavorite('https://vk.com')">
            <svg viewBox="0 0 24 24" width="20" height="20" fill="#0077ff"><path d="M15.684 0H8.316C2.992 0 0 2.992 0 8.316v7.368C0 21.008 2.992 24 8.316 24h7.368C21.008 24 24 21.008 24 15.684V8.316C24 2.992 21.008 0 15.684 0zm3.692 17.123h-1.744c-.66 0-.862-.525-2.05-1.714-1.036-1.01-1.496-1.144-1.75-1.144-.356 0-.457.102-.457.593v1.642c0 .423-.135.686-1.253.686-1.846 0-3.896-1.12-5.336-3.201-2.17-3.074-2.778-5.386-2.778-5.86 0-.254.102-.491.593-.491h1.745c.44 0 .61.203.78.677.847 2.49 2.27 4.675 2.862 4.675.22 0 .322-.102.322-.66V11.23c-.068-1.185-.695-1.287-.695-1.71 0-.203.17-.406.44-.406h2.744c.373 0 .508.186.508.626v3.354c0 .355.153.474.254.474.22 0 .407-.119.814-.525 1.27-1.423 2.185-3.642 2.185-3.642.119-.254.339-.491.78-.491h1.744c.525 0 .644.27.525.643-.22.983-2.287 3.913-2.388 4.082-.204.322-.288.474 0 .864.203.27 1.05 1.016 1.592 1.643.999 1.152 1.762 2.117 1.965 2.778.084.271-.102.66-.542.66z"/></svg>
            <div id="dot-vk" class="aether-fav-active-dot"></div>
        </div>
        <div class="aether-fav-tile" title="Telegram" onclick="openFavorite('https://web.telegram.org')">
            <svg viewBox="0 0 24 24" width="20" height="20" fill="#29b6f6"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm5.894 8.221l-1.97 9.28c-.145.658-.537.818-1.084.508l-3-2.21-1.446 1.394c-.16.16-.295.295-.605.295l.213-3.053 5.56-5.023c.242-.213-.054-.333-.373-.121l-6.871 4.326-2.962-.924c-.643-.204-.657-.643.136-.953l11.57-4.458c.538-.196 1.006.128.832.935z"/></svg>
            <div id="dot-tg" class="aether-fav-active-dot"></div>
        </div>
        <div class="aether-fav-tile" title="GitHub" onclick="openFavorite('https://github.com')">
            <svg viewBox="0 0 24 24" width="20" height="20" fill="#ffffff"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/></svg>
            <div id="dot-gh" class="aether-fav-active-dot"></div>
        </div>
    </div>

    <!-- 4. Tabs List -->
    <div class="aether-tabs-header">
        <span id="tabs-counter">Вкладки</span>
        <button id="btn-new-tab" class="aether-new-tab-btn" title="Новая вкладка (Ctrl + T)">+</button>
    </div>
    <div id="tabs-list" class="aether-tabs-list"></div>

    <!-- 5. Bottom Utilities Dock -->
    <div class="aether-dock">
        <button id="btn-audio" class="aether-dock-btn" title="Режим 'Только аудио' (Ctrl + M) — экономия GPU">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 18v-6a9 9 0 0 1 18 0v6"/><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/></svg>
        </button>
        <button id="btn-pip" class="aether-dock-btn" title="Картинка в картинке / PiP (Ctrl + P)">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><rect x="2" y="3" width="20" height="14" rx="2"/><rect x="12" y="9" width="8" height="6" rx="1"/></svg>
        </button>
        <button id="btn-download" class="aether-dock-btn" title="Скачать видео / yt-dlp (Ctrl + D)">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
        </button>
        <button id="btn-home" class="aether-dock-btn" title="Домашняя страница Aether">
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
        </button>
    </div>

    <script>
        function sendIpc(msg) {
            if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(msg);
            }
        }

        // Navigation
        document.getElementById('btn-back').onclick = () => sendIpc('back');
        document.getElementById('btn-forward').onclick = () => sendIpc('forward');
        document.getElementById('btn-reload').onclick = () => sendIpc('reload');
        document.getElementById('btn-new-tab').onclick = () => sendIpc('new_tab:');
        document.getElementById('btn-home').onclick = () => sendIpc('go_home');
        document.getElementById('btn-toggle-sidebar').onclick = () => sendIpc('toggle_sidebar');

        // Dock tools
        document.getElementById('btn-audio').onclick = () => sendIpc('toggle_audio');
        document.getElementById('btn-pip').onclick = () => sendIpc('toggle_pip');
        document.getElementById('btn-download').onclick = () => sendIpc('download');

        // Omnibox
        const omnibox = document.getElementById('omnibox-input');
        omnibox.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const val = omnibox.value.trim();
                if (!val) return;
                sendIpc('navigate:' + val);
            }
        });

        // Open Favorite (open or switch)
        function openFavorite(url) {
            sendIpc('open_favorite:' + url);
        }

        function getTabIcon(url, title) {
            const u = (url || '').toLowerCase();
            if (u.includes('youtube.com')) return '<span style="color:#ef4444; font-size:13px;">▶</span>';
            if (u.includes('vk.com')) return '<span style="color:#0077ff; font-weight:bold; font-size:11px;">VK</span>';
            if (u.includes('telegram.org')) return '<span style="color:#38bdf8; font-size:12px;">✈</span>';
            if (u.includes('github.com')) return '<span style="color:#f8fafc; font-size:12px;">🐙</span>';
            if (u.includes('start.html')) return '<span style="color:#c084fc; font-size:12px;">🌌</span>';
            return '<span style="color:#94a3b8; font-size:12px;">🌐</span>';
        }

        // Called from Rust via evaluate_script
        window.updateTabs = function(tabs, activeId) {
            const list = document.getElementById('tabs-list');
            list.innerHTML = '';
            document.getElementById('tabs-counter').textContent = 'Вкладки (' + tabs.length + ')';

            // Check favorite dots
            let hasYt = false, hasVk = false, hasTg = false, hasGh = false;

            tabs.forEach(tab => {
                const u = (tab.url || '').toLowerCase();
                if (u.includes('youtube.com')) hasYt = true;
                if (u.includes('vk.com')) hasVk = true;
                if (u.includes('telegram.org')) hasTg = true;
                if (u.includes('github.com')) hasGh = true;

                const item = document.createElement('div');
                item.className = 'aether-tab-item' + (tab.id === activeId ? ' active' : '');
                
                const icon = document.createElement('div');
                icon.className = 'aether-tab-badge';
                icon.innerHTML = getTabIcon(tab.url, tab.title);

                const title = document.createElement('div');
                title.className = 'aether-tab-title';
                title.textContent = tab.title || 'Новая вкладка';

                const close = document.createElement('div');
                close.className = 'aether-tab-close';
                close.textContent = '✕';
                close.onclick = (e) => {
                    e.stopPropagation();
                    sendIpc('close_tab:' + tab.id);
                };

                item.onclick = () => {
                    if (tab.id !== activeId) {
                        sendIpc('switch_tab:' + tab.id);
                    }
                };

                item.appendChild(icon);
                item.appendChild(title);
                item.appendChild(close);
                list.appendChild(item);

                if (tab.id === activeId) {
                    const isStart = (tab.url || '').includes('start.html');
                    omnibox.value = isStart ? '' : (tab.url || '');
                }
            });

            document.getElementById('dot-yt').style.display = hasYt ? 'block' : 'none';
            document.getElementById('dot-vk').style.display = hasVk ? 'block' : 'none';
            document.getElementById('dot-tg').style.display = hasTg ? 'block' : 'none';
            document.getElementById('dot-gh').style.display = hasGh ? 'block' : 'none';
        };

        // Keyboard navigation inside sidebar
        window.addEventListener('keydown', (e) => {
            const ctrl = e.ctrlKey || e.metaKey;
            if (ctrl && (e.key === 't' || e.key === 'T' || e.key === 'е' || e.key === 'Е')) {
                e.preventDefault();
                sendIpc('new_tab:');
            } else if (ctrl && (e.key === 'w' || e.key === 'W' || e.key === 'ц' || e.key === 'Ц')) {
                e.preventDefault();
                sendIpc('close_active_tab');
            } else if (ctrl && (e.key === 'b' || e.key === 'B' || e.key === 'и' || e.key === 'И')) {
                e.preventDefault();
                sendIpc('toggle_sidebar');
            } else if (ctrl && (e.key === 'l' || e.key === 'L' || e.key === 'д' || e.key === 'Д') || e.key === 'F6') {
                e.preventDefault();
                omnibox.focus();
                omnibox.select();
            }
        });
    </script>
</body>
</html>
"###;

pub const CONTENT_SCRIPT: &str = r###"
(() => {
    // 1. Уведомление об изменении URL или заголовка
    function notify() {
        try {
            if (window.ipc && window.ipc.postMessage) {
                const title = document.title || window.location.hostname || "Вкладка";
                window.ipc.postMessage("tab_navigated:" + title + "|||" + window.location.href);
            }
        } catch (_) {}
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', notify);
    } else {
        notify();
    }
    window.addEventListener('load', notify);
    window.addEventListener('yt-navigate-finish', notify);
    window.addEventListener('popstate', notify);

    let lastTitle = "";
    let lastUrl = "";
    setInterval(() => {
        if (document.title !== lastTitle || window.location.href !== lastUrl) {
            lastTitle = document.title;
            lastUrl = window.location.href;
            notify();
        }
    }, 400);

    // 2. Горячие клавиши браузера
    window.addEventListener('keydown', (e) => {
        const ctrl = e.ctrlKey || e.metaKey;
        if (ctrl && (e.key === 't' || e.key === 'T' || e.key === 'е' || e.key === 'Е')) {
            e.preventDefault();
            if (window.ipc) window.ipc.postMessage("new_tab:");
        } else if (ctrl && (e.key === 'w' || e.key === 'W' || e.key === 'ц' || e.key === 'Ц')) {
            e.preventDefault();
            if (window.ipc) window.ipc.postMessage("close_active_tab");
        } else if (ctrl && (e.key === 'b' || e.key === 'B' || e.key === 'и' || e.key === 'И')) {
            e.preventDefault();
            if (window.ipc) window.ipc.postMessage("toggle_sidebar");
        } else if (ctrl && e.key === 'Tab') {
            e.preventDefault();
            if (window.ipc) window.ipc.postMessage(e.shiftKey ? "prev_tab" : "next_tab");
        }
    });

    // 3. Авто-перехват ссылок target="_blank"
    document.addEventListener('click', (e) => {
        try {
            const link = e.target && e.target.closest ? e.target.closest('a') : null;
            if (link && link.target === '_blank' && link.href && !link.href.startsWith('javascript:')) {
                e.preventDefault();
                if (window.ipc) window.ipc.postMessage("new_tab:" + link.href);
            }
        } catch (_) {}
    }, true);

    // 4. YouTube AdBlocker
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
})();
"###;
