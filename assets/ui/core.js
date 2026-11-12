(() => {
    try {
        if (window.top !== window.self) return;
    } catch (_) {
        return;
    }

    if (window.__AETHER_CORE_ATTACHED__) return;
    window.__AETHER_CORE_ATTACHED__ = true;

    // Отправка сообщения в Rust. Состояние вкладок хранит Rust, страница лишь шлёт намерения.
    function post(msg) {
        try { window.ipc.postMessage(msg); return true; } catch (_) { return false; }
    }

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

    // 2. СОСТОЯНИЕ ВКЛАДОК
    // Источник истины — Rust: вкладки, быстрые карточки и счётчики визитов приходят готовым снимком
    // (window.__AETHER_STATE__ при создании страницы, __aetherApplyState — при изменениях).
    function getStartPageUrl() {
        return window.__AETHER_GLOBAL_START_URL__ || "http://aether.localhost/";
    }

    function isStartUrl(u) {
        return !!u && (u.includes("aether.localhost") || u.includes("start.html"));
    }

    function getTabsState() {
        const s = window.__AETHER_STATE__;
        return (s && Array.isArray(s.tabs)) ? s : { activeTabId: 0, tabs: [], cards: [], visits: {} };
    }

    function getCustomCards() {
        const s = getTabsState();
        return Array.isArray(s.cards) ? s.cards : [];
    }

    let sidebarDirty = false;
    let lastSig = "";
    function stateSig(s) {
        try { return JSON.stringify([s.activeTabId, s.tabs.map(t => [t.id, t.title, t.url]), s.cards]); } catch (_) { return ""; }
    }

    // Rust присылает свежее состояние. Сайдбар перерисовывается, только если он открыт, данные
    // реально изменились и пользователь не печатает в омнибоксе; иначе — при следующем открытии.
    window.__aetherApplyState = function (state) {
        window.__AETHER_STATE__ = state;
        if (stateSig(state) === lastSig) return;
        const ob = document.getElementById('aether-sb-omnibox');
        if (!isSidebarOpen || (ob && document.activeElement === ob)) {
            sidebarDirty = true;
            return;
        }
        renderSidebar();
    };

    // 3. ПЛАВАЮЩИЙ САЙДБАР ARC
    const SIDEBAR_CSS = `
        #aether-progress-bar {
            position: fixed !important;
            top: 0 !important;
            left: 0 !important;
            width: 0% !important;
            height: 3px !important;
            background: linear-gradient(90deg, #9333ea, #a855f7, #38bdf8, #c084fc) !important;
            box-shadow: 0 0 10px rgba(192, 132, 252, 0.8), 0 0 20px rgba(147, 51, 234, 0.5) !important;
            z-index: 2147483647 !important;
            pointer-events: none !important;
            transition: width 0.25s cubic-bezier(0.1, 0.9, 0.2, 1), opacity 0.2s ease !important;
            opacity: 0;
        }

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

        #aether-hover-edge {
            position: fixed !important;
            top: 0 !important;
            left: 0 !important;
            width: 8px !important;
            height: 100vh !important;
            z-index: 2147483644 !important;
            background: transparent !important;
        }

        #aether-arc-sidebar {
            position: fixed !important;
            top: 0 !important;
            left: 0 !important;
            width: 185px !important;
            height: 100vh !important;
            background: rgba(12, 7, 24, 0.96) !important;
            backdrop-filter: blur(24px) !important;
            border-right: 1px solid rgba(168, 85, 247, 0.35) !important;
            box-shadow: none !important;
            visibility: hidden !important;
            pointer-events: none !important;
            z-index: 2147483646 !important;
            display: flex !important;
            flex-direction: column !important;
            padding: 10px 8px 8px !important;
            box-sizing: border-box !important;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
            user-select: none !important;
            color: #f8fafc !important;
            transform: translateX(-100%) !important;
            transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1), visibility 0.22s !important;
        }
        #aether-arc-sidebar.open {
            transform: translateX(0) !important;
            visibility: visible !important;
            pointer-events: auto !important;
            box-shadow: 4px 0 16px rgba(0, 0, 0, 0.35) !important;
        }

        :fullscreen #aether-trigger-pill,
        :-webkit-full-screen #aether-trigger-pill,
        :fullscreen #aether-hover-edge,
        :-webkit-full-screen #aether-hover-edge,
        :fullscreen #aether-arc-sidebar,
        :-webkit-full-screen #aether-arc-sidebar {
            display: none !important;
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
            font-size: 14px !important;
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
        if (sidebarDirty) renderSidebar();
        const sidebar = document.getElementById('aether-arc-sidebar');
        if (sidebar) sidebar.classList.add('open');
        // адрес мог измениться без перезагрузки страницы (SPA)
        const ob = document.getElementById('aether-sb-omnibox');
        if (ob && document.activeElement !== ob) ob.value = isStartUrl(location.href) ? "" : location.href;
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

        let triggerPill = document.getElementById('aether-trigger-pill');
        if (!triggerPill) {
            triggerPill = document.createElement('div');
            triggerPill.id = 'aether-trigger-pill';
            triggerPill.title = 'Открыть меню Aether (Ctrl + B)';
            triggerPill.innerHTML = `
                <svg width="18" height="18" viewBox="0 0 100 100" fill="none">
                    <rect width="100" height="100" rx="26" fill="#4c1d95"/>
                    <ellipse cx="50" cy="50" rx="38" ry="18" stroke="#38bdf8" stroke-width="4" transform="rotate(-25 50 50)"/>
                    <path d="M38 72L50 28L62 72M42 58H58" stroke="#fff" stroke-width="6" stroke-linecap="round"/>
                </svg>
            `;
            triggerPill.onclick = (e) => {
                e.stopPropagation();
                toggleSidebar();
            };
            document.documentElement.appendChild(triggerPill);
        }

        let hoverEdge = document.getElementById('aether-hover-edge');
        if (!hoverEdge) {
            hoverEdge = document.createElement('div');
            hoverEdge.id = 'aether-hover-edge';
            hoverEdge.addEventListener('mouseenter', openSidebar);
            document.documentElement.appendChild(hoverEdge);
        }

        let sidebar = document.getElementById('aether-arc-sidebar');
        if (!sidebar) {
            sidebar = document.createElement('div');
            sidebar.id = 'aether-arc-sidebar';
            sidebar.addEventListener('mouseleave', closeSidebar);
            document.documentElement.appendChild(sidebar);
        }

        const state = getTabsState();
        lastSig = stateSig(state);
        sidebarDirty = false;

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

        // Быстрые карточки
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

    // 5. КОНТЕКСТНОЕ МЕНЮ ПКМ ПО ВИДЕО (Создается ТОЛЬКО при реальном ПКМ по видео!)
    function showVideoMenu(x, y) {
        let menu = document.getElementById('aether-video-menu');
        if (!menu) {
            menu = document.createElement('div');
            menu.id = 'aether-video-menu';
            menu.style.cssText = "display:none; position:fixed; z-index:2147483647; background:rgba(15,9,30,0.96); backdrop-filter:blur(16px); border:1px solid rgba(168,85,247,0.45); border-radius:12px; box-shadow:0 15px 40px rgba(0,0,0,0.85); padding:6px 0; min-width:210px; font-family:sans-serif; color:#f1f5f9; user-select:none;";
            safeSetHTML(menu, `
                <div style="padding:6px 14px 4px; font-size:10.5px; font-weight:600; color:#a78bfa; text-transform:uppercase; border-bottom:1px solid rgba(255,255,255,0.06); margin-bottom:4px;">🎬 Aether Медиа</div>
                <div id="aether-vmi-download" style="padding:8px 14px; font-size:12.5px; display:flex; align-items:center; gap:9px; cursor:pointer;"><span>⬇️</span> <span>Скачать видео (yt-dlp)</span></div>
                <div id="aether-vmi-pip" style="padding:8px 14px; font-size:12.5px; display:flex; align-items:center; gap:9px; cursor:pointer;"><span>🔲</span> <span>Картинка в картинке</span></div>
                <div id="aether-vmi-audio" style="padding:8px 14px; font-size:12.5px; display:flex; align-items:center; gap:9px; cursor:pointer;"><span>🎧</span> <span>Режим «Только аудио»</span></div>
                <div id="aether-vmi-speed" style="padding:8px 14px; font-size:12.5px; display:flex; align-items:center; gap:9px; cursor:pointer;"><span>⚡</span> <span>Скорость 2x / 1x</span></div>
                <div id="aether-vmi-copy" style="padding:8px 14px; font-size:12.5px; display:flex; align-items:center; gap:9px; cursor:pointer;"><span>📋</span> <span>Скопировать ссылку</span></div>
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

        const menuX = Math.min(x, window.innerWidth - 220);
        const menuY = Math.min(y, window.innerHeight - 200);
        menu.style.left = menuX + 'px';
        menu.style.top = menuY + 'px';
        menu.style.display = 'block';
    }

    document.addEventListener('contextmenu', (e) => {
        if (e.shiftKey) return;
        const isVideo = e.target.closest('video, .html5-video-player, ytd-player, #player, .video-stream');
        if (isVideo) {
            e.preventDefault();
            e.stopPropagation();
            showVideoMenu(e.clientX, e.clientY);
        } else {
            const menu = document.getElementById('aether-video-menu');
            if (menu) menu.style.display = 'none';
        }
    }, true);

    document.addEventListener('click', (e) => {
        const menu = document.getElementById('aether-video-menu');
        if (menu && !menu.contains(e.target)) menu.style.display = 'none';
    });

    function toggleVideoSpeed() {
        const video = document.querySelector('video');
        if (video) {
            video.playbackRate = (video.playbackRate === 1.0) ? 2.0 : 1.0;
            showToast(`⚡ Скорость: ${video.playbackRate}x`);
        }
    }

    // Нейро-индикатор мгновенной загрузки страницы
    function triggerLoadProgress() {
        let bar = document.getElementById('aether-progress-bar');
        if (!bar) {
            bar = document.createElement('div');
            bar.id = 'aether-progress-bar';
            document.documentElement.appendChild(bar);
        }
        bar.style.opacity = '1';
        bar.style.width = '35%';
        setTimeout(() => {
            if (bar && bar.style.opacity === '1') bar.style.width = '75%';
        }, 120);
    }

    function completeLoadProgress() {
        scheduleHideLoader();
        const bar = document.getElementById('aether-progress-bar');
        if (bar) {
            bar.style.width = '100%';
            setTimeout(() => {
                bar.style.opacity = '0';
                setTimeout(() => { bar.style.width = '0%'; }, 200);
            }, 150);
        }
    }

    // Фирменный экран загрузки Aether. Показывается при переходах, запущенных самим браузером
    // (омнибокс, карточки, вкладки, «домой»), чтобы вместо белой вспышки был тёмный экран в стиле Aether.
    // Стили только через CSSOM и Web Animations API — работает даже на сайтах со строгим CSP.
    const LOADER_ID = 'aether-loader';
    const IS_START_HOST = /(^|\.)aether\.localhost$/.test(location.hostname);
    let loaderEl = null;
    let loaderHideTimer = 0;
    let loaderSafetyTimer = 0;

    function loaderHostLabel(target) {
        try {
            if (!target) return "";
            const u = new URL(target, location.href);
            return u.hostname.replace(/^www\./, "") || String(target);
        } catch (_) {
            return String(target || "");
        }
    }

    function buildLoader(label) {
        const root = document.createElement('div');
        root.id = LOADER_ID;
        root.style.cssText = 'position:fixed;top:0;left:0;right:0;bottom:0;z-index:2147483646;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:20px;'
            + 'background:radial-gradient(ellipse at 50% 32%,#2a1150 0%,#120a22 48%,#07040d 100%);opacity:1;transition:opacity .22s ease;'
            + 'font-family:"Segoe UI",system-ui,sans-serif;margin:0;padding:0;border:0;pointer-events:none;';

        const orb = document.createElement('div');
        orb.style.cssText = 'position:relative;width:96px;height:96px;display:flex;align-items:center;justify-content:center;';

        const ring = document.createElement('div');
        ring.style.cssText = 'position:absolute;top:0;left:0;right:0;bottom:0;border-radius:50%;border:2px solid transparent;border-top-color:#c084fc;border-right-color:#38bdf8;box-sizing:border-box;';

        const logo = document.createElement('div');
        logo.style.cssText = 'width:60px;height:60px;border-radius:16px;box-shadow:0 0 26px rgba(168,85,247,.55);';
        safeSetHTML(logo,
            '<svg viewBox="0 0 100 100" width="60" height="60" fill="none" xmlns="http://www.w3.org/2000/svg">'
            + '<rect width="100" height="100" rx="26" fill="url(#aeLoadBg)"/>'
            + '<rect x="2" y="2" width="96" height="96" rx="24" stroke="url(#aeLoadGlow)" stroke-width="3"/>'
            + '<ellipse cx="50" cy="50" rx="38" ry="18" stroke="#38bdf8" stroke-width="2.5" opacity="0.85" transform="rotate(-25 50 50)"/>'
            + '<path d="M38 72L50 28L62 72M42 58H58" stroke="#f8fafc" stroke-width="4.5" stroke-linecap="round" stroke-linejoin="round"/>'
            + '<circle cx="72" cy="28" r="3.5" fill="#ffffff"/>'
            + '<defs><linearGradient id="aeLoadBg" x1="0" y1="0" x2="100" y2="100" gradientUnits="userSpaceOnUse"><stop stop-color="#180730"/><stop offset="0.5" stop-color="#4c1d95"/><stop offset="1" stop-color="#7c3aed"/></linearGradient>'
            + '<linearGradient id="aeLoadGlow" x1="0" y1="0" x2="100" y2="100" gradientUnits="userSpaceOnUse"><stop stop-color="#c084fc"/><stop offset="1" stop-color="#38bdf8"/></linearGradient></defs></svg>');

        const title = document.createElement('div');
        title.textContent = 'AETHER';
        title.style.cssText = 'font-size:15px;font-weight:700;letter-spacing:.5em;text-indent:.5em;color:#d8b4fe;';

        const track = document.createElement('div');
        track.style.cssText = 'position:relative;width:150px;height:3px;border-radius:3px;background:rgba(168,85,247,.18);overflow:hidden;';
        const slider = document.createElement('div');
        slider.style.cssText = 'position:absolute;top:0;left:0;width:45%;height:100%;border-radius:3px;background:linear-gradient(90deg,#9333ea,#a855f7,#38bdf8);box-shadow:0 0 10px rgba(192,132,252,.8);';
        track.appendChild(slider);

        const sub = document.createElement('div');
        sub.textContent = label ? ('Загрузка · ' + label) : 'Загрузка…';
        sub.style.cssText = 'font-size:12px;color:#8b7bb0;max-width:70vw;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;';

        orb.appendChild(ring);
        orb.appendChild(logo);
        root.appendChild(orb);
        root.appendChild(title);
        root.appendChild(track);
        root.appendChild(sub);

        try {
            if (typeof ring.animate === 'function') {
                ring.animate([{ transform: 'rotate(0deg)' }, { transform: 'rotate(360deg)' }], { duration: 1100, iterations: Infinity, easing: 'linear' });
                logo.animate([{ transform: 'scale(1)' }, { transform: 'scale(1.07)' }, { transform: 'scale(1)' }], { duration: 1600, iterations: Infinity, easing: 'ease-in-out' });
                slider.animate([{ transform: 'translateX(-100%)' }, { transform: 'translateX(320%)' }], { duration: 1300, iterations: Infinity, easing: 'ease-in-out' });
            }
        } catch (_) {}
        return root;
    }

    function attachLoader() {
        const host = document.documentElement;
        if (!host || !loaderEl) return false;
        if (!loaderEl.isConnected) host.appendChild(loaderEl);
        return true;
    }

    function removeLoaderNow() {
        clearTimeout(loaderHideTimer);
        clearTimeout(loaderSafetyTimer);
        loaderHideTimer = 0;
        loaderSafetyTimer = 0;
        const el = loaderEl;
        loaderEl = null;
        if (el && el.isConnected) {
            el.style.opacity = '0';
            el.style.pointerEvents = 'none';
            setTimeout(() => { try { el.remove(); } catch (_) {} }, 260);
        }
    }

    let loaderShownAt = 0;

    // cap — максимум, сколько экран может висеть, если страница так и не сообщила о готовности.
    // Для старой страницы (пока идёт сетевой запрос) он длиннее, для новой — короткий.
    function showLoader(target, cap) {
        try {
            clearTimeout(loaderHideTimer);
            loaderHideTimer = 0;
            if (!loaderEl) {
                loaderEl = buildLoader(loaderHostLabel(target));
                loaderShownAt = Date.now();
            }
            if (!attachLoader()) {
                const mo = new MutationObserver(() => { if (attachLoader()) mo.disconnect(); });
                mo.observe(document, { childList: true });
            }
            clearTimeout(loaderSafetyTimer);
            loaderSafetyTimer = setTimeout(removeLoaderNow, cap || 8000);
        } catch (_) {}
    }

    function scheduleHideLoader() {
        if (!loaderEl || loaderHideTimer) return;
        // Минимум 220 мс на экране, чтобы не мигал, и сразу прячем — без дополнительных пауз
        loaderHideTimer = setTimeout(removeLoaderNow, Math.max(0, 220 - (Date.now() - loaderShownAt)));
    }

    window.__aetherShowLoader = showLoader;

    // Новый документ открыт переходом, который запустил Aether (нет referrer, это не перезагрузка и не стартовая):
    // показываем экран загрузки сразу, до первой отрисовки страницы.
    (() => {
        try {
            if (IS_START_HOST) return;
            let navType = 'navigate';
            try { navType = performance.getEntriesByType('navigation')[0].type || 'navigate'; } catch (_) {}
            if (navType !== 'navigate' || document.referrer) return;
            showLoader(location.href, 1500);
            // Прячем при готовности DOM либо через 0.7 с после появления <body> —
            // тяжёлые сайты (YouTube) начинают рисоваться раньше DOMContentLoaded.
            document.addEventListener('DOMContentLoaded', scheduleHideLoader, { once: true });
            const armBody = () => {
                if (!document.body) return false;
                setTimeout(scheduleHideLoader, 700);
                return true;
            };
            if (!armBody()) {
                const mo = new MutationObserver(() => { if (armBody()) mo.disconnect(); });
                mo.observe(document, { childList: true, subtree: true });
            }
        } catch (_) {}
    })();

    // Обработка полноэкранного режима HTML5 видео и скрытия панели задач Windows
    function handleFullscreenChange() {
        const isFs = !!(document.fullscreenElement || document.webkitFullscreenElement || document.mozFullScreenElement || document.msFullscreenElement);
        const pill = document.getElementById('aether-trigger-pill');
        const edge = document.getElementById('aether-hover-edge');
        const sb = document.getElementById('aether-arc-sidebar');
        if (isFs) {
            if (pill) pill.style.display = 'none';
            if (edge) edge.style.display = 'none';
            if (sb) { sb.style.display = 'none'; sb.classList.remove('open'); }
        } else {
            if (pill) pill.style.display = 'flex';
            if (edge) edge.style.display = 'block';
            if (sb) sb.style.display = 'flex';
        }
        if (window.ipc && window.ipc.postMessage) {
            try {
                window.ipc.postMessage("set_fullscreen:" + (isFs ? "true" : "false"));
            } catch (_) {}
        }
    }
    document.addEventListener('fullscreenchange', handleFullscreenChange);
    document.addEventListener('webkitfullscreenchange', handleFullscreenChange);
    document.addEventListener('mozfullscreenchange', handleFullscreenChange);

    // 6. НАВИГАЦИЯ И ВКЛАДКИ
    // Страница лишь отправляет намерения — вкладками, переходами и их сохранением управляет Rust.
    function goHome() {
        if (isStartUrl(location.href)) return;
        // без экрана загрузки: стартовая страница отдаётся локально, а Rust может просто переключить вкладку
        if (!post("go_home")) window.location.href = getStartPageUrl();
    }

    function createNewTab(url) {
        if (!post("tab_new:" + (url || ""))) window.location.href = url || getStartPageUrl();
    }

    function switchTab(id) {
        post("tab_switch:" + id);
    }

    function closeTab(id) {
        post("tab_close:" + id);
    }

    // Клик колёсиком мыши по ссылке — новая вкладка
    document.addEventListener('auxclick', (e) => {
        if (e.button !== 1) return;
        const anchor = e.target.closest && e.target.closest('a[href]');
        if (anchor && anchor.href && !anchor.href.startsWith('javascript:')) {
            e.preventDefault();
            e.stopPropagation();
            createNewTab(anchor.href);
        }
    }, true);

    // Ссылки target="_blank" открываются во вкладках Aether
    document.addEventListener('click', (e) => {
        if (e.button !== 0) return;
        const anchor = e.target.closest && e.target.closest('a[href]');
        if (anchor && anchor.target === '_blank' && anchor.href && !anchor.href.startsWith('javascript:')) {
            e.preventDefault();
            e.stopPropagation();
            createNewTab(anchor.href);
        }
    }, true);

    // window.open тоже открывает вкладку
    const _origWindowOpen = window.open;
    window.open = function (url) {
        if (url && typeof url === 'string') {
            createNewTab(url);
            return window;
        }
        return _origWindowOpen.apply(this, arguments);
    };

    function openService(url) {
        const domain = url.replace("https://", "").replace("http://", "").replace("www.", "").split('/')[0];
        const existing = getTabsState().tabs.find(t => t.url && t.url.includes(domain));
        if (existing) {
            switchTab(existing.id);
        } else if (isStartUrl(location.href)) {
            navigateToUrl(url);
        } else {
            createNewTab(url);
        }
    }

    function navigateToUrl(url) {
        if (isStartUrl(url)) {
            goHome();
            return;
        }
        triggerLoadProgress();
        showLoader(url);
        if (!post("navigate:" + url)) window.location.href = url;
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
            if (document.hidden) return; // фоновая вкладка не опрашивается
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
        document.addEventListener('DOMContentLoaded', () => {
            renderSidebar();
            completeLoadProgress();
        });
    } else {
        renderSidebar();
        completeLoadProgress();
    }
    // Медиа: сообщаем Rust, играет ли во вкладке звук/видео — такие вкладки не выгружаются из памяти
    let mediaPlaying = 0;
    function reportMedia(flag) {
        if (flag !== mediaPlaying) {
            mediaPlaying = flag;
            post("tab_media:" + flag);
        }
    }
    function recheckMedia() {
        let any = 0;
        document.querySelectorAll('video,audio').forEach(m => { if (!m.paused && !m.ended) any = 1; });
        reportMedia(any);
    }
    document.addEventListener('playing', () => reportMedia(1), true);
    document.addEventListener('pause', recheckMedia, true);
    document.addEventListener('ended', recheckMedia, true);

    function onDomReady() {
        try { renderSidebar(); } finally { completeLoadProgress(); }
    }
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', onDomReady, { once: true });
    } else {
        onDomReady();
    }
    window.addEventListener('load', completeLoadProgress);
    window.addEventListener('yt-navigate-finish', completeLoadProgress);
})();
