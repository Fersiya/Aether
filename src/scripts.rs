//! Встроенный рантайм Aether Browser v0.5.3:
//! - Настраиваемые быстрые сервисы (пользователь сам добавляет, удаляет и редактирует закладки)
//! - Быстрый переход по Ctrl+1..9 к своим сервисам
//! - Плавающий Dynamic Island с навигацией и медиа-панелью
//! - Стильный HUD Омнибокс с префиксами (F6 / Ctrl+L)
//! - Неблокирующие всплывающие уведомления (Toasts)
//! - YouTube AdBlock (16x skip + авто-клик)
//! - Медиа-контроль: PiP, режим "Только аудио", скачивание через yt-dlp

pub const AETHER_CORE_SCRIPT: &str = r#"
(() => {
    if (window.__AETHER_INJECTED__) return;
    window.__AETHER_INJECTED__ = true;
    console.log("⚡ Aether Engine v0.5.3: Модуль кастомных быстрых сервисов и Dynamic Island запущен");

    // ==========================================
    // 1. СТИЛИ ИНТЕРФЕЙСА AETHER
    // ==========================================
    function injectAetherStyles() {
        if (document.getElementById('aether-injected-styles')) return;
        const style = document.createElement('style');
        style.id = 'aether-injected-styles';
        style.textContent = `
            :fullscreen #aether-island-root,
            :-webkit-full-screen #aether-island-root {
                display: none !important;
            }

            #aether-island-root {
                position: fixed;
                top: 10px;
                left: 50%;
                transform: translateX(-50%);
                z-index: 2147483640;
                user-select: none;
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                pointer-events: none;
            }

            #aether-island {
                pointer-events: auto;
                background: rgba(15, 23, 42, 0.90);
                backdrop-filter: blur(20px) saturate(180%);
                -webkit-backdrop-filter: blur(20px) saturate(180%);
                border: 1px solid rgba(168, 85, 247, 0.4);
                border-radius: 9999px;
                padding: 4px 10px;
                display: flex;
                align-items: center;
                gap: 6px;
                box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5), 0 0 20px rgba(168, 85, 247, 0.2);
                transition: all 0.25s cubic-bezier(0.16, 1, 0.3, 1);
            }

            #aether-island:hover {
                border-color: rgba(192, 132, 252, 0.65);
                box-shadow: 0 12px 35px rgba(0, 0, 0, 0.6), 0 0 25px rgba(168, 85, 247, 0.35);
            }

            .aether-btn {
                width: 30px;
                height: 30px;
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
            .aether-btn:active {
                transform: scale(0.95);
            }
            .aether-btn.aether-active {
                background: rgba(168, 85, 247, 0.6) !important;
                color: #ffffff !important;
                box-shadow: 0 0 12px rgba(168, 85, 247, 0.6);
            }

            #aether-url-pill {
                display: flex;
                align-items: center;
                gap: 8px;
                background: rgba(30, 41, 59, 0.7);
                border: 1px solid rgba(168, 85, 247, 0.3);
                border-radius: 9999px;
                padding: 5px 14px;
                max-width: 260px;
                cursor: pointer;
                transition: all 0.2s ease;
            }
            #aether-url-pill:hover {
                background: rgba(30, 41, 59, 0.95);
                border-color: rgba(192, 132, 252, 0.6);
                box-shadow: 0 0 12px rgba(168, 85, 247, 0.3);
            }
            #aether-url-text {
                font-size: 13px;
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

            /* Кастомные сервисы */
            #aether-services-container {
                display: flex;
                align-items: center;
                gap: 5px;
            }
            .aether-svc-badge {
                padding: 4px 9px;
                border-radius: 9999px;
                font-size: 11.5px;
                font-weight: 600;
                cursor: pointer;
                border: 1px solid rgba(168, 85, 247, 0.35);
                background: rgba(168, 85, 247, 0.18);
                color: #e9d5ff;
                transition: all 0.18s ease;
            }
            .aether-svc-badge:hover {
                transform: translateY(-1px) scale(1.05);
                border-color: #c084fc;
                box-shadow: 0 0 10px rgba(192, 132, 252, 0.4);
            }

            .aether-btn-add {
                width: 24px;
                height: 24px;
                border-radius: 50%;
                background: rgba(255, 255, 255, 0.08);
                border: 1px dashed rgba(168, 85, 247, 0.5);
                color: #d8b4fe;
                font-size: 14px;
                font-weight: bold;
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: all 0.15s ease;
                padding: 0;
            }
            .aether-btn-add:hover {
                background: rgba(168, 85, 247, 0.3);
                border-style: solid;
                color: #fff;
                transform: scale(1.1);
            }

            /* Сворачивание */
            #aether-island.aether-collapsed {
                padding: 3px 8px;
            }
            #aether-island.aether-collapsed .aether-collapsible {
                display: none !important;
            }

            /* Toasts */
            #aether-toast-container {
                position: fixed;
                bottom: 24px;
                right: 24px;
                z-index: 2147483647;
                display: flex;
                flex-direction: column;
                gap: 10px;
                pointer-events: none;
            }
            .aether-toast {
                display: flex;
                align-items: center;
                gap: 10px;
                background: rgba(15, 23, 42, 0.94);
                backdrop-filter: blur(14px);
                -webkit-backdrop-filter: blur(14px);
                border: 1px solid rgba(168, 85, 247, 0.5);
                box-shadow: 0 12px 30px rgba(0, 0, 0, 0.6), 0 0 18px rgba(168, 85, 247, 0.25);
                border-radius: 12px;
                padding: 12px 18px;
                color: #f8fafc;
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                font-size: 13.5px;
                max-width: 420px;
                opacity: 0;
                transform: translateY(12px);
                transition: opacity 0.22s ease-out, transform 0.22s ease-out;
            }

            /* Оверлей добавления сервиса и Омнибокс */
            .aether-modal-overlay {
                position: fixed;
                inset: 0;
                z-index: 2147483646;
                background: rgba(0, 0, 0, 0.65);
                backdrop-filter: blur(14px);
                -webkit-backdrop-filter: blur(14px);
                display: none;
                align-items: flex-start;
                justify-content: center;
                padding-top: 14vh;
                opacity: 0;
                transition: opacity 0.18s ease-out;
            }
            .aether-modal-card {
                width: 90%;
                max-width: 560px;
                background: #0f172a;
                border: 1px solid rgba(168, 85, 247, 0.45);
                border-radius: 16px;
                padding: 20px 24px;
                box-shadow: 0 25px 60px rgba(0, 0, 0, 0.7), 0 0 35px rgba(168, 85, 247, 0.25);
                transform: translateY(-12px) scale(0.98);
                transition: transform 0.18s cubic-bezier(0.16, 1, 0.3, 1);
                color: #f8fafc;
                box-sizing: border-box;
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            }
            .aether-modal-card h3 {
                margin: 0 0 16px 0;
                font-size: 17px;
                color: #d8b4fe;
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .aether-form-group {
                margin-bottom: 14px;
            }
            .aether-form-group label {
                display: block;
                font-size: 12.5px;
                color: #94a3b8;
                margin-bottom: 6px;
            }
            .aether-form-input {
                width: 100%;
                box-sizing: border-box;
                background: rgba(30, 41, 59, 0.85);
                border: 1.5px solid rgba(168, 85, 247, 0.4);
                border-radius: 10px;
                padding: 10px 14px;
                font-size: 14.5px;
                color: #ffffff;
                outline: none;
                transition: border-color 0.15s, box-shadow 0.15s;
            }
            .aether-form-input:focus {
                border-color: #c084fc;
                box-shadow: 0 0 14px rgba(192, 132, 252, 0.35);
            }
            .aether-btn-row {
                display: flex;
                justify-content: flex-end;
                gap: 10px;
                margin-top: 18px;
            }
            .aether-submit-btn {
                background: #9333ea;
                color: #ffffff;
                border: none;
                border-radius: 8px;
                padding: 8px 18px;
                font-size: 13.5px;
                font-weight: 600;
                cursor: pointer;
                transition: background 0.15s, transform 0.15s;
            }
            .aether-submit-btn:hover {
                background: #a855f7;
                transform: translateY(-1px);
            }
            .aether-cancel-btn {
                background: rgba(255, 255, 255, 0.08);
                color: #94a3b8;
                border: none;
                border-radius: 8px;
                padding: 8px 16px;
                font-size: 13.5px;
                cursor: pointer;
            }
            .aether-cancel-btn:hover {
                background: rgba(255, 255, 255, 0.15);
                color: #ffffff;
            }
        `;
        (document.head || document.documentElement).appendChild(style);
    }

    // ==========================================
    // 2. ВСПЛЫВАЮЩИЕ УВЕДОМЛЕНИЯ (TOASTS)
    // ==========================================
    function showToast(message, icon = "🌌") {
        injectAetherStyles();
        let container = document.getElementById("aether-toast-container");
        if (!container) {
            container = document.createElement("div");
            container.id = "aether-toast-container";
            document.documentElement.appendChild(container);
        }

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
    // 3. МЕНЕДЖЕР КАСТОМНЫХ БЫСТРЫХ СЕРВИСОВ
    // ==========================================
    const STORAGE_KEY = "__aether_custom_services";
    const COLOR_PALETTE = [
        { bg: "rgba(239, 68, 68, 0.22)", border: "rgba(239, 68, 68, 0.45)", text: "#fca5a5" },
        { bg: "rgba(59, 130, 246, 0.22)", border: "rgba(59, 130, 246, 0.45)", text: "#93c5fd" },
        { bg: "rgba(168, 85, 247, 0.22)", border: "rgba(168, 85, 247, 0.45)", text: "#d8b4fe" },
        { bg: "rgba(16, 185, 129, 0.22)", border: "rgba(16, 185, 129, 0.45)", text: "#6ee7b7" },
        { bg: "rgba(245, 158, 11, 0.22)", border: "rgba(245, 158, 11, 0.45)", text: "#fcd34d" },
        { bg: "rgba(236, 72, 153, 0.22)", border: "rgba(236, 72, 153, 0.45)", text: "#f472b6" },
        { bg: "rgba(20, 184, 166, 0.22)", border: "rgba(20, 184, 166, 0.45)", text: "#5eead4" },
    ];

    function getSavedServices() {
        try {
            const raw = localStorage.getItem(STORAGE_KEY);
            if (raw !== null) {
                return JSON.parse(raw);
            }
        } catch (_) {}
        // Дефолтные сервисы при самом первом запуске:
        return [
            { title: "YouTube", badge: "YT", url: "https://www.youtube.com" },
            { title: "ВКонтакте", badge: "VK", url: "https://vk.com" },
            { title: "Telegram", badge: "TG", url: "https://web.telegram.org" }
        ];
    }

    function saveServices(services) {
        try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(services));
        } catch (_) {}
    }

    function renderQuickServices() {
        const container = document.getElementById("aether-services-container");
        if (!container) return;
        container.innerHTML = "";

        const services = getSavedServices();
        services.forEach((svc, index) => {
            const palette = COLOR_PALETTE[index % COLOR_PALETTE.length];
            const badge = document.createElement("div");
            badge.className = "aether-svc-badge aether-collapsible";
            badge.textContent = svc.badge || svc.title.substring(0, 2).toUpperCase();
            badge.title = `${svc.title} (Ctrl+${index + 1})\n${svc.url}\n\n• Левый клик — открыть\n• Правый клик — удалить`;
            badge.style.background = palette.bg;
            badge.style.borderColor = palette.border;
            badge.style.color = palette.text;

            // Левый клик — переход
            badge.addEventListener("click", (e) => {
                e.stopPropagation();
                window.location.href = svc.url;
            });

            // Правый клик — удаление
            badge.addEventListener("contextmenu", (e) => {
                e.preventDefault();
                e.stopPropagation();
                if (confirm(`Удалить «${svc.title}» (${svc.url}) из быстрых сервисов?`)) {
                    services.splice(index, 1);
                    saveServices(services);
                    renderQuickServices();
                    showToast(`Сервис «${svc.title}» удален`, "🗑️");
                }
            });

            container.appendChild(badge);
        });
    }

    // Модальное окно добавления нового сервиса
    function openAddServiceModal() {
        injectAetherStyles();
        let overlay = document.getElementById("aether-add-svc-overlay");
        if (!overlay) {
            overlay = document.createElement("div");
            overlay.id = "aether-add-svc-overlay";
            overlay.className = "aether-modal-overlay";
            overlay.innerHTML = `
                <div class="aether-modal-card">
                    <h3><span>⭐</span> <span>Добавить быстрый сервис</span></h3>
                    <div class="aether-form-group">
                        <label>Название сервиса:</label>
                        <input id="aether-form-title" class="aether-form-input" type="text" placeholder="Например: GitHub, Twitch, Kinopoisk..." />
                    </div>
                    <div class="aether-form-group">
                        <label>Короткий бейдж (2–3 буквы):</label>
                        <input id="aether-form-badge" class="aether-form-input" type="text" maxlength="4" placeholder="GH, TW, KP..." />
                    </div>
                    <div class="aether-form-group">
                        <label>URL адрес:</label>
                        <input id="aether-form-url" class="aether-form-input" type="text" placeholder="https://..." />
                    </div>
                    <div class="aether-btn-row">
                        <button id="aether-form-cancel" class="aether-cancel-btn">Отмена</button>
                        <button id="aether-form-save" class="aether-submit-btn">Сохранить</button>
                    </div>
                </div>
            `;
            document.documentElement.appendChild(overlay);

            document.getElementById("aether-form-cancel").addEventListener("click", closeAddServiceModal);
            overlay.addEventListener("click", (e) => {
                if (e.target === overlay) closeAddServiceModal();
            });

            document.getElementById("aether-form-save").addEventListener("click", submitAddService);
        }

        // Автозаполнение текущими данными сайта
        const host = window.location.hostname.replace(/^www\./i, "");
        const title = document.title || host;
        const autoBadge = host.split(".")[0].substring(0, 3).toUpperCase() || "WEB";

        document.getElementById("aether-form-title").value = title.substring(0, 25);
        document.getElementById("aether-form-badge").value = autoBadge;
        document.getElementById("aether-form-url").value = window.location.href;

        overlay.style.display = "flex";
        requestAnimationFrame(() => {
            overlay.style.opacity = "1";
            document.getElementById("aether-form-title").focus();
        });
    }

    function closeAddServiceModal() {
        const overlay = document.getElementById("aether-add-svc-overlay");
        if (!overlay) return;
        overlay.style.opacity = "0";
        setTimeout(() => {
            overlay.style.display = "none";
        }, 180);
    }

    function submitAddService() {
        const titleInput = document.getElementById("aether-form-title");
        const badgeInput = document.getElementById("aether-form-badge");
        const urlInput = document.getElementById("aether-form-url");

        let title = titleInput.value.trim();
        let badge = badgeInput.value.trim();
        let url = urlInput.value.trim();

        if (!title) title = "Сайт";
        if (!badge) badge = title.substring(0, 2).toUpperCase();
        if (!url) url = window.location.href;
        if (!url.startsWith("http://") && !url.startsWith("https://")) {
            url = "https://" + url;
        }

        const services = getSavedServices();
        services.push({ title, badge, url });
        saveServices(services);
        renderQuickServices();

        closeAddServiceModal();
        showToast(`Сервис «${title}» добавлен!`, "✨");
    }

    // ==========================================
    // 4. СОВРЕМЕННЫЙ DYNAMIC ISLAND
    // ==========================================
    function buildDynamicIsland() {
        injectAetherStyles();
        if (document.getElementById("aether-island-root")) return;

        const isHttps = window.location.protocol === "https:";
        const host = window.location.hostname || "google.com";
        const isCollapsed = localStorage.getItem("__aether_island_collapsed") === "true";

        const root = document.createElement("div");
        root.id = "aether-island-root";
        root.innerHTML = `
            <div id="aether-island" class="${isCollapsed ? "aether-collapsed" : ""}">
                <!-- Навигация -->
                <button id="aether-nav-back" class="aether-btn aether-collapsible" title="Назад (Alt + ←)">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
                </button>
                <button id="aether-nav-forward" class="aether-btn aether-collapsible" title="Вперед (Alt + →)">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                </button>
                <button id="aether-nav-reload" class="aether-btn aether-collapsible" title="Обновить (F5)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/></svg>
                </button>
                <button id="aether-nav-home" class="aether-btn aether-collapsible" title="Главная Google (Alt + Home)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
                </button>

                <!-- Адресная капсула -->
                <div id="aether-url-pill" title="Нажмите для поиска или ввода адреса (Ctrl + L / F6)">
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="${isHttps ? '#c084fc' : '#94a3b8'}" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                        <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                    </svg>
                    <span id="aether-url-text">${host}</span>
                </div>

                <!-- Быстрые сервисы пользователя -->
                <div class="aether-sep aether-collapsible"></div>
                <div id="aether-services-container"></div>
                <button id="aether-btn-add-svc" class="aether-btn-add aether-collapsible" title="Добавить текущий сайт в быстрые сервисы">+</button>

                <!-- Медиа-инструменты -->
                <div class="aether-sep aether-collapsible"></div>
                <button id="aether-tool-audio" class="aether-btn aether-collapsible" title="Режим 'Только аудио' (Ctrl + M) — отключить видеоряд для разгрузки GPU">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 18v-6a9 9 0 0 1 18 0v6"/><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"/></svg>
                </button>
                <button id="aether-tool-pip" class="aether-btn aether-collapsible" title="Картинка в картинке / PiP (Ctrl + P)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2"/><rect x="12" y="9" width="8" height="6" rx="1"/></svg>
                </button>
                <button id="aether-tool-download" class="aether-btn aether-collapsible" title="Скачать медиа / yt-dlp (Ctrl + D)">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                </button>

                <!-- Свернуть / Развернуть остров -->
                <button id="aether-toggle-collapse" class="aether-btn" title="Свернуть / развернуть панель">
                    <svg id="aether-collapse-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="${isCollapsed ? '6 9 12 15 18 9' : '18 15 12 9 6 15'}"/></svg>
                </button>
            </div>
        `;

        document.documentElement.appendChild(root);

        // Привязка обработчиков
        document.getElementById("aether-nav-back").addEventListener("click", () => window.history.back());
        document.getElementById("aether-nav-forward").addEventListener("click", () => window.history.forward());
        document.getElementById("aether-nav-reload").addEventListener("click", () => window.location.reload());
        document.getElementById("aether-nav-home").addEventListener("click", () => window.location.href = "https://www.google.com");

        document.getElementById("aether-url-pill").addEventListener("click", openOmnibox);
        document.getElementById("aether-btn-add-svc").addEventListener("click", openAddServiceModal);

        document.getElementById("aether-tool-audio").addEventListener("click", toggleAudioOnly);
        document.getElementById("aether-tool-pip").addEventListener("click", togglePiP);
        document.getElementById("aether-tool-download").addEventListener("click", downloadMedia);

        const island = document.getElementById("aether-island");
        const collapseBtn = document.getElementById("aether-toggle-collapse");
        const collapseIcon = document.getElementById("aether-collapse-icon");

        collapseBtn.addEventListener("click", (e) => {
            e.stopPropagation();
            const nowCollapsed = island.classList.toggle("aether-collapsed");
            localStorage.setItem("__aether_island_collapsed", nowCollapsed);
            collapseIcon.innerHTML = `<polyline points="${nowCollapsed ? '6 9 12 15 18 9' : '18 15 12 9 6 15'}"/>`;
        });

        // Отрисовываем кастомные сервисы пользователя
        renderQuickServices();
    }

    // ==========================================
    // 5. СОВРЕМЕННЫЙ HUD ОМНИБОКС (CTRL+L / F6)
    // ==========================================
    let omniboxOverlay = null;
    let omniboxInput = null;

    function createOmnibox() {
        injectAetherStyles();
        if (document.getElementById("aether-omnibox-overlay")) return;

        omniboxOverlay = document.createElement("div");
        omniboxOverlay.id = "aether-omnibox-overlay";
        omniboxOverlay.className = "aether-modal-overlay";
        omniboxOverlay.innerHTML = `
            <div id="aether-omnibox-card" class="aether-modal-card">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px;">
                    <div style="display: flex; align-items: center; gap: 8px; font-size: 15px; font-weight: 600; color: #d8b4fe;">
                        <span>🌌</span>
                        <span>Aether Omnibox</span>
                    </div>
                    <div style="font-size: 11px; color: #94a3b8; background: rgba(255, 255, 255, 0.08); padding: 3px 8px; border-radius: 6px;">Esc — закрыть</div>
                </div>
                <input id="aether-omnibox-input" class="aether-form-input" style="font-size: 16px; padding: 13px 16px;" type="text" autocomplete="off" spellcheck="false" placeholder="Адрес сайта, поисковый запрос или команда (yt:, vk:, gh:)..." />
                <div style="display: flex; flex-wrap: wrap; align-items: center; gap: 8px; margin-top: 14px; font-size: 12px;">
                    <span class="aether-tag aether-tag-yt" data-prefix="yt: ">yt: YouTube</span>
                    <span class="aether-tag aether-tag-vk" data-prefix="vk: ">vk: ВКонтакте</span>
                    <span class="aether-tag aether-tag-gh" data-prefix="gh: ">gh: GitHub</span>
                    <span style="margin-left: auto; color: #64748b; font-size: 12px;">↵ Enter для перехода</span>
                </div>
            </div>
        `;

        document.documentElement.appendChild(omniboxOverlay);
        omniboxInput = document.getElementById("aether-omnibox-input");

        omniboxOverlay.querySelectorAll(".aether-tag").forEach(tag => {
            tag.addEventListener("click", (e) => {
                e.stopPropagation();
                const prefix = tag.getAttribute("data-prefix");
                omniboxInput.value = prefix;
                omniboxInput.focus();
            });
        });

        omniboxOverlay.addEventListener("click", (e) => {
            if (e.target === omniboxOverlay) {
                closeOmnibox();
            }
        });

        omniboxInput.addEventListener("keydown", (e) => {
            e.stopPropagation();

            if (e.key === "Escape") {
                e.preventDefault();
                closeOmnibox();
            } else if (e.key === "Enter") {
                e.preventDefault();
                navigateFromInput(omniboxInput.value);
            }
        });
    }

    function openOmnibox() {
        createOmnibox();
        if (!omniboxOverlay || !omniboxInput) return;

        omniboxInput.value = window.location.href;
        omniboxOverlay.style.display = "flex";

        requestAnimationFrame(() => {
            omniboxOverlay.style.opacity = "1";
            omniboxInput.focus();
            omniboxInput.select();
        });
    }

    function closeOmnibox() {
        if (!omniboxOverlay) return;
        omniboxOverlay.style.opacity = "0";
        setTimeout(() => {
            omniboxOverlay.style.display = "none";
        }, 180);
    }

    function navigateFromInput(rawInput) {
        if (!rawInput) return;
        const trimmed = rawInput.trim();
        if (!trimmed) return;

        closeOmnibox();

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

        const isUrl = trimmed.startsWith("http://") || 
                      trimmed.startsWith("https://") || 
                      (trimmed.includes(".") && !trimmed.includes(" ") && !trimmed.endsWith("."));

        if (isUrl) {
            if (trimmed.startsWith("http://") || trimmed.startsWith("https://")) {
                window.location.href = trimmed;
            } else {
                window.location.href = "https://" + trimmed;
            }
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
                showToast("ВК Музыка переключена", "🎵");
                return; 
            }
        }
        const media = document.querySelector('video, audio');
        if (media) {
            if (media.paused) { 
                media.play(); 
                showToast("Воспроизведение", "▶️");
            } else { 
                media.pause(); 
                showToast("Пауза", "⏸️");
            }
        }
    }

    function togglePiP() {
        const video = document.querySelector('video');
        if (!video) {
            showToast("Видео на странице не найдено", "⚠️");
            return;
        }
        if (document.pictureInPictureElement) {
            document.exitPictureInPicture().then(() => {
                showToast("PiP выключен", "📺");
            }).catch(console.error);
        } else if (document.pictureInPictureEnabled) {
            video.requestPictureInPicture().then(() => {
                showToast("Картинка в картинке (PiP)", "📺");
            }).catch(console.error);
        }
    }

    function toggleAudioOnly() {
        const video = document.querySelector('video');
        const audioBtn = document.getElementById("aether-tool-audio");

        if (!video) {
            showToast("Видео на странице не найдено", "⚠️");
            return;
        }
        if (video.style.opacity === '0.001') {
            video.style.opacity = '1';
            video.style.visibility = 'visible';
            if (audioBtn) audioBtn.classList.remove("aether-active");
            showToast("Обычный режим видео включен", "🎬");
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
            showToast("Начато прямое скачивание файла", "📥");
            return;
        }

        const cmd = `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best" "${curUrl}"`;
        navigator.clipboard.writeText(cmd).then(() => {
            showToast("Команда yt-dlp скопирована в буфер обмена!", "📋");
        }).catch(() => {
            showToast(`Ссылка: ${curUrl}`, "🔗");
        });
    }

    // ==========================================
    // 7. ADBLOCK И УСКОРЕНИЕ РЕКЛАМЫ
    // ==========================================
    function setupYouTubeAdBlock() {
        if (!location.hostname.includes("youtube.com")) return;

        setInterval(() => {
            const skipBtn = document.querySelector('.ytp-skip-ad-button, .ytp-ad-skip-button, .ytp-ad-skip-button-modern');
            if (skipBtn) {
                skipBtn.click();
            }

            const video = document.querySelector('video');
            const adShowing = document.querySelector('.ad-showing, .ad-interrupting');
            if (video && adShowing) {
                video.playbackRate = 16.0;
                video.muted = true;
                if (isFinite(video.duration) && video.duration > 0) {
                    video.currentTime = video.duration;
                }
            }

            const adOverlays = document.querySelectorAll('ytd-ad-slot-renderer, #player-ads, ytd-banner-promo-renderer, ytd-in-feed-ad-layout-renderer');
            adOverlays.forEach(el => el.style.display = 'none');
        }, 300);
    }

    function setupGenericCleaners() {
        const adSelectors = ['.adsbygoogle', 'div[id^="ad_"]', 'div[class*="banner-ad"]', '#cookie-banner', '.cookie-notice'];
        try {
            document.querySelectorAll(adSelectors.join(', ')).forEach(el => el.style.display = 'none');
        } catch (_) {}
    }

    function forceHighQuality() {
        if (!location.hostname.includes("youtube.com")) return;
        const player = document.getElementById('movie_player');
        if (player && typeof player.setPlaybackQualityRange === 'function') {
            player.setPlaybackQualityRange('hd1080', 'highres');
        }
    }

    // ==========================================
    // 8. ОБРАБОТКА ГОРЯЧИХ КЛАВИШ
    // ==========================================
    window.addEventListener('keydown', (e) => {
        const ctrlOrCmd = e.ctrlKey || e.metaKey;
        const activeEl = document.activeElement;
        const isTyping = activeEl && (
            ['INPUT', 'TEXTAREA'].includes(activeEl.tagName) || 
            activeEl.isContentEditable
        );

        if (e.key === 'F6' || (ctrlOrCmd && (e.key === 'l' || e.key === 'L' || e.key === 'д' || e.key === 'Д'))) {
            e.preventDefault();
            openOmnibox();
            return;
        }

        if (isTyping) return;

        // Переход по быстрым сервисам пользователя: Ctrl + 1..9
        if (ctrlOrCmd && e.key >= '1' && e.key <= '9') {
            const index = parseInt(e.key) - 1;
            const services = getSavedServices();
            if (services && services[index]) {
                e.preventDefault();
                showToast(`Переход: ${services[index].title}`, "🚀");
                window.location.href = services[index].url;
                return;
            }
        }

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

    function init() {
        buildDynamicIsland();
        setupYouTubeAdBlock();
        setupGenericCleaners();
        setTimeout(forceHighQuality, 1500);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
"#;
