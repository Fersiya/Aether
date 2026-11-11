//! Встроенный рантайм Aether Browser v0.5.2:
//! - Плавающий Dynamic Island (верхний бар с навигацией, адресом, быстрыми сервисами и медиа-кнопками)
//! - Стильный HUD Омнибокс с префиксами (F6 / Ctrl+L)
//! - Неблокирующие всплывающие уведомления (Toasts)
//! - Умный пропуск рекламы на YouTube (16x skip + авто-клик)
//! - Медиа-контроль: Picture-in-Picture, режим "Только аудио", скачивание через yt-dlp

pub const AETHER_CORE_SCRIPT: &str = r#"
(() => {
    if (window.__AETHER_INJECTED__) return;
    window.__AETHER_INJECTED__ = true;
    console.log("⚡ Aether Engine v0.5.2: Запущен современный Dynamic Island и медиа-контроль");

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
                background: rgba(15, 23, 42, 0.88);
                backdrop-filter: blur(20px) saturate(180%);
                -webkit-backdrop-filter: blur(20px) saturate(180%);
                border: 1px solid rgba(168, 85, 247, 0.35);
                border-radius: 9999px;
                padding: 4px 10px;
                display: flex;
                align-items: center;
                gap: 6px;
                box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5), 0 0 20px rgba(168, 85, 247, 0.2);
                transition: all 0.25s cubic-bezier(0.16, 1, 0.3, 1);
            }

            #aether-island:hover {
                border-color: rgba(192, 132, 252, 0.6);
                box-shadow: 0 12px 35px rgba(0, 0, 0, 0.6), 0 0 25px rgba(168, 85, 247, 0.35);
            }

            /* Кнопки навигации и действий */
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
                border-color: rgba(168, 85, 247, 0.4);
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

            /* Строка URL в острове */
            #aether-url-pill {
                display: flex;
                align-items: center;
                gap: 8px;
                background: rgba(30, 41, 59, 0.7);
                border: 1px solid rgba(168, 85, 247, 0.25);
                border-radius: 9999px;
                padding: 5px 14px;
                max-width: 280px;
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

            /* Разделители */
            .aether-sep {
                width: 1px;
                height: 18px;
                background: rgba(255, 255, 255, 0.12);
                margin: 0 2px;
            }

            /* Быстрые бейджи сервисов */
            .aether-svc-badge {
                padding: 3px 8px;
                border-radius: 9999px;
                font-size: 11px;
                font-weight: 600;
                cursor: pointer;
                transition: all 0.15s ease;
            }
            .aether-svc-yt { background: rgba(239, 68, 68, 0.2); color: #fca5a5; border: 1px solid rgba(239, 68, 68, 0.3); }
            .aether-svc-yt:hover { background: rgba(239, 68, 68, 0.35); transform: translateY(-1px); }
            .aether-svc-vk { background: rgba(59, 130, 246, 0.2); color: #93c5fd; border: 1px solid rgba(59, 130, 246, 0.3); }
            .aether-svc-vk:hover { background: rgba(59, 130, 246, 0.35); transform: translateY(-1px); }
            .aether-svc-tg { background: rgba(56, 189, 248, 0.2); color: #7dd3fc; border: 1px solid rgba(56, 189, 248, 0.3); }
            .aether-svc-tg:hover { background: rgba(56, 189, 248, 0.35); transform: translateY(-1px); }

            /* Режим сворачивания острова */
            #aether-island.aether-collapsed {
                padding: 3px 8px;
            }
            #aether-island.aether-collapsed .aether-collapsible {
                display: none !important;
            }

            /* Всплывающие уведомления (Toasts) */
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

            /* Модальный Омнибокс */
            #aether-omnibox-overlay {
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
            #aether-omnibox-card {
                width: 90%;
                max-width: 660px;
                background: #0f172a;
                border: 1px solid rgba(168, 85, 247, 0.45);
                border-radius: 16px;
                padding: 18px 22px;
                box-shadow: 0 25px 60px rgba(0, 0, 0, 0.7), 0 0 35px rgba(168, 85, 247, 0.25);
                transform: translateY(-12px) scale(0.98);
                transition: transform 0.18s cubic-bezier(0.16, 1, 0.3, 1);
                color: #f8fafc;
                box-sizing: border-box;
            }
            #aether-omnibox-header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                margin-bottom: 14px;
            }
            #aether-omnibox-title {
                display: flex;
                align-items: center;
                gap: 8px;
                font-size: 15px;
                font-weight: 600;
                color: #d8b4fe;
            }
            #aether-omnibox-close-hint {
                font-size: 11px;
                color: #94a3b8;
                background: rgba(255, 255, 255, 0.08);
                padding: 3px 8px;
                border-radius: 6px;
            }
            #aether-omnibox-input {
                width: 100%;
                box-sizing: border-box;
                background: rgba(30, 41, 59, 0.85);
                border: 1.5px solid rgba(168, 85, 247, 0.5);
                border-radius: 12px;
                padding: 14px 18px;
                font-size: 16px;
                color: #ffffff;
                outline: none;
                transition: border-color 0.15s, box-shadow 0.15s;
            }
            #aether-omnibox-input:focus {
                border-color: #c084fc;
                box-shadow: 0 0 16px rgba(192, 132, 252, 0.4);
            }
            #aether-omnibox-tags {
                display: flex;
                flex-wrap: wrap;
                align-items: center;
                gap: 8px;
                margin-top: 14px;
                font-size: 12px;
            }
            .aether-tag {
                padding: 4px 10px;
                border-radius: 6px;
                cursor: pointer;
                transition: opacity 0.15s, transform 0.15s;
            }
            .aether-tag:hover {
                opacity: 0.85;
                transform: translateY(-1px);
            }
            .aether-tag-yt { background: rgba(239, 68, 68, 0.2); color: #fca5a5; border: 1px solid rgba(239, 68, 68, 0.35); }
            .aether-tag-vk { background: rgba(59, 130, 246, 0.2); color: #93c5fd; border: 1px solid rgba(59, 130, 246, 0.35); }
            .aether-tag-gh { background: rgba(148, 163, 184, 0.2); color: #cbd5e1; border: 1px solid rgba(148, 163, 184, 0.35); }
            #aether-omnibox-hint-enter {
                margin-left: auto;
                color: #64748b;
                font-size: 12px;
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
    // 3. СОВРЕМЕННЫЙ DYNAMIC ISLAND
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

                <!-- Быстрые сервисы -->
                <div class="aether-sep aether-collapsible"></div>
                <div class="aether-svc-badge aether-svc-yt aether-collapsible" id="aether-btn-yt" title="YouTube (Ctrl + 2)">YT</div>
                <div class="aether-svc-badge aether-svc-vk aether-collapsible" id="aether-btn-vk" title="ВКонтакте (Ctrl + 3)">VK</div>
                <div class="aether-svc-badge aether-svc-tg aether-collapsible" id="aether-btn-tg" title="Telegram (Ctrl + 4)">TG</div>

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

        // Обработчики кнопок острова
        document.getElementById("aether-nav-back").addEventListener("click", () => window.history.back());
        document.getElementById("aether-nav-forward").addEventListener("click", () => window.history.forward());
        document.getElementById("aether-nav-reload").addEventListener("click", () => window.location.reload());
        document.getElementById("aether-nav-home").addEventListener("click", () => window.location.href = "https://www.google.com");

        document.getElementById("aether-url-pill").addEventListener("click", openOmnibox);

        document.getElementById("aether-btn-yt").addEventListener("click", () => window.location.href = "https://www.youtube.com");
        document.getElementById("aether-btn-vk").addEventListener("click", () => window.location.href = "https://vk.com");
        document.getElementById("aether-btn-tg").addEventListener("click", () => window.location.href = "https://web.telegram.org");

        document.getElementById("aether-tool-audio").addEventListener("click", toggleAudioOnly);
        document.getElementById("aether-tool-pip").addEventListener("click", togglePiP);
        document.getElementById("aether-tool-download").addEventListener("click", downloadMedia);

        // Кнопка сворачивания
        const island = document.getElementById("aether-island");
        const collapseBtn = document.getElementById("aether-toggle-collapse");
        const collapseIcon = document.getElementById("aether-collapse-icon");

        collapseBtn.addEventListener("click", (e) => {
            e.stopPropagation();
            const nowCollapsed = island.classList.toggle("aether-collapsed");
            localStorage.setItem("__aether_island_collapsed", nowCollapsed);
            collapseIcon.innerHTML = `<polyline points="${nowCollapsed ? '6 9 12 15 18 9' : '18 15 12 9 6 15'}"/>`;
        });
    }

    // ==========================================
    // 4. СОВРЕМЕННЫЙ HUD ОМНИБОКС (CTRL+L / F6)
    // ==========================================
    let omniboxOverlay = null;
    let omniboxInput = null;

    function createOmnibox() {
        injectAetherStyles();
        if (document.getElementById("aether-omnibox-overlay")) return;

        omniboxOverlay = document.createElement("div");
        omniboxOverlay.id = "aether-omnibox-overlay";
        omniboxOverlay.innerHTML = `
            <div id="aether-omnibox-card">
                <div id="aether-omnibox-header">
                    <div id="aether-omnibox-title">
                        <span>🌌</span>
                        <span>Aether Omnibox</span>
                    </div>
                    <div id="aether-omnibox-close-hint">Esc — закрыть</div>
                </div>
                <input id="aether-omnibox-input" type="text" autocomplete="off" spellcheck="false" placeholder="Адрес сайта, поисковый запрос или команда (yt:, vk:, gh:)..." />
                <div id="aether-omnibox-tags">
                    <span class="aether-tag aether-tag-yt" data-prefix="yt: ">yt: YouTube</span>
                    <span class="aether-tag aether-tag-vk" data-prefix="vk: ">vk: ВКонтакте</span>
                    <span class="aether-tag aether-tag-gh" data-prefix="gh: ">gh: GitHub</span>
                    <span id="aether-omnibox-hint-enter">↵ Enter для перехода</span>
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
            const card = document.getElementById("aether-omnibox-card");
            if (card) card.style.transform = "translateY(0) scale(1)";
            omniboxInput.focus();
            omniboxInput.select();
        });
    }

    function closeOmnibox() {
        if (!omniboxOverlay) return;
        omniboxOverlay.style.opacity = "0";
        const card = document.getElementById("aether-omnibox-card");
        if (card) card.style.transform = "translateY(-12px) scale(0.98)";
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
    // 5. МЕДИА-КОНТРОЛЬ И СКАЧИВАНИЕ
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
    // 6. ADBLOCK И УСКОРЕНИЕ РЕКЛАМЫ
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
    // 7. ОБРАБОТКА ГОРЯЧИХ КЛАВИШ
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
        else if (ctrlOrCmd && e.key === '1') {
            e.preventDefault();
            window.location.href = "https://www.google.com";
        }
        else if (ctrlOrCmd && e.key === '2') {
            e.preventDefault();
            window.location.href = "https://www.youtube.com";
        }
        else if (ctrlOrCmd && e.key === '3') {
            e.preventDefault();
            window.location.href = "https://vk.com";
        }
        else if (ctrlOrCmd && e.key === '4') {
            e.preventDefault();
            window.location.href = "https://web.telegram.org";
        }
    });

    // Инициализация при загрузке документа
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
