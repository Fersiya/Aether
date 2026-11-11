//! Встроенный рантайм Aether Browser v0.5.1:
//! - Современный плавающий омнибокс (HUD Omnibox)
//! - Неблокирующие всплывающие уведомления (Toasts)
//! - Умный пропуск и ускорение рекламы на YouTube (16x skip)
//! - Медиа-контроль: Picture-in-Picture, "Только аудио", универсальная пауза/плей
//! - Быстрое скачивание / копирование команды yt-dlp
//! - Горячие клавиши с поддержкой латинской и русской раскладок

pub const AETHER_CORE_SCRIPT: &str = r#"
(() => {
    if (window.__AETHER_INJECTED__) return;
    window.__AETHER_INJECTED__ = true;
    console.log("⚡ Aether Engine v0.5.1: Ядро управления и защиты успешно подключено");

    // ==========================================
    // 1. СТИЛИ ДЛЯ HUD ОМНИБОКСА И УВЕДОМЛЕНИЙ
    // ==========================================
    function injectAetherStyles() {
        if (document.getElementById('aether-injected-styles')) return;
        const style = document.createElement('style');
        style.id = 'aether-injected-styles';
        style.textContent = `
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
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
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
                user-select: none;
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
                user-select: none;
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
                user-select: none;
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
                backdrop-filter: blur(12px);
                -webkit-backdrop-filter: blur(12px);
                border: 1px solid rgba(168, 85, 247, 0.5);
                box-shadow: 0 12px 30px rgba(0, 0, 0, 0.55), 0 0 15px rgba(168, 85, 247, 0.2);
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
    // 3. СОВРЕМЕННЫЙ HUD ОМНИБОКС (CTRL+L / F6)
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

        // Клик по тегу префикса
        omniboxOverlay.querySelectorAll(".aether-tag").forEach(tag => {
            tag.addEventListener("click", (e) => {
                e.stopPropagation();
                const prefix = tag.getAttribute("data-prefix");
                omniboxInput.value = prefix;
                omniboxInput.focus();
            });
        });

        // Клик снаружи карточки закрывает омнибокс
        omniboxOverlay.addEventListener("click", (e) => {
            if (e.target === omniboxOverlay) {
                closeOmnibox();
            }
        });

        // Клавиатура внутри инпута
        omniboxInput.addEventListener("keydown", (e) => {
            e.stopPropagation(); // Не пропускаем клавиши на саму страницу

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

        // 1. YouTube префиксы
        if (trimmed.toLowerCase().startsWith("yt:") || trimmed.toLowerCase().startsWith("ютуб:")) {
            const q = trimmed.replace(/^(yt:|ютуб:)/i, "").trim();
            window.location.href = "https://www.youtube.com/results?search_query=" + encodeURIComponent(q);
            return;
        }

        // 2. ВКонтакте префиксы
        if (trimmed.toLowerCase().startsWith("vk:") || trimmed.toLowerCase().startsWith("вк:")) {
            const q = trimmed.replace(/^(vk:|вк:)/i, "").trim();
            window.location.href = "https://vk.com/search?c%5Bsection%5D=auto&c%5Bq%5D=" + encodeURIComponent(q);
            return;
        }

        // 3. GitHub префикс
        if (trimmed.toLowerCase().startsWith("gh:")) {
            const q = trimmed.replace(/^gh:/i, "").trim();
            window.location.href = "https://github.com/search?q=" + encodeURIComponent(q);
            return;
        }

        // 4. Проверка на URL
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
            // 5. Поиск по умолчанию в Google
            window.location.href = "https://www.google.com/search?q=" + encodeURIComponent(trimmed);
        }
    }

    // ==========================================
    // 4. МЕДИА-КОНТРОЛЬ И СКАЧИВАНИЕ
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
                showToast("Режим Картинка в картинке (PiP)", "📺");
            }).catch(console.error);
        }
    }

    function toggleAudioOnly() {
        const video = document.querySelector('video');
        if (!video) {
            showToast("Видео на странице не найдено", "⚠️");
            return;
        }
        if (video.style.opacity === '0.001') {
            video.style.opacity = '1';
            video.style.visibility = 'visible';
            showToast("Обычный режим видео включен", "🎬");
        } else {
            video.style.opacity = '0.001';
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
    // 5. ADBLOCK И ОЧИСТКА РЕКЛАМЫ
    // ==========================================
    function setupYouTubeAdBlock() {
        if (!location.hostname.includes("youtube.com")) return;

        setInterval(() => {
            // Клик по кнопке пропуска рекламы
            const skipBtn = document.querySelector('.ytp-skip-ad-button, .ytp-ad-skip-button, .ytp-ad-skip-button-modern');
            if (skipBtn) {
                skipBtn.click();
            }

            // Мгновенный пропуск рекламы на 16x скорости
            const video = document.querySelector('video');
            const adShowing = document.querySelector('.ad-showing, .ad-interrupting');
            if (video && adShowing) {
                video.playbackRate = 16.0;
                video.muted = true;
                if (isFinite(video.duration) && video.duration > 0) {
                    video.currentTime = video.duration;
                }
            }

            // Скрытие баннеров
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
    // 6. ОБРАБОТКА ГОРЯЧИХ КЛАВИШ
    // ==========================================
    window.addEventListener('keydown', (e) => {
        const ctrlOrCmd = e.ctrlKey || e.metaKey;
        const activeEl = document.activeElement;
        const isTyping = activeEl && (
            ['INPUT', 'TEXTAREA'].includes(activeEl.tagName) || 
            activeEl.isContentEditable
        );

        // F6 или Ctrl+L (Д) — Открыть Омнибокс (работает всегда)
        if (e.key === 'F6' || (ctrlOrCmd && (e.key === 'l' || e.key === 'L' || e.key === 'д' || e.key === 'Д'))) {
            e.preventDefault();
            openOmnibox();
            return;
        }

        // Если пользователь печатает в поле ввода на странице — не перехватываем остальные хоткеи
        if (isTyping) return;

        // Ctrl+Space — Пауза / Воспроизведение
        if (ctrlOrCmd && e.code === 'Space') {
            e.preventDefault();
            togglePlayPause();
        }
        // Ctrl+P (З) — Picture-in-Picture
        else if (ctrlOrCmd && (e.key === 'p' || e.key === 'P' || e.key === 'з' || e.key === 'З')) {
            e.preventDefault();
            togglePiP();
        }
        // Ctrl+M (Ь) — Режим 'Только аудио'
        else if (ctrlOrCmd && (e.key === 'm' || e.key === 'M' || e.key === 'ь' || e.key === 'Ь')) {
            e.preventDefault();
            toggleAudioOnly();
        }
        // Ctrl+D (В) — Скачать медиа
        else if (ctrlOrCmd && (e.key === 'd' || e.key === 'D' || e.key === 'в' || e.key === 'В')) {
            e.preventDefault();
            downloadMedia();
        }
        // Навигация: Назад / Вперед
        else if (e.altKey && e.key === 'ArrowLeft') {
            e.preventDefault();
            window.history.back();
        }
        else if (e.altKey && e.key === 'ArrowRight') {
            e.preventDefault();
            window.history.forward();
        }
        // Обновить страницу (F5 или Ctrl+R / К)
        else if (e.key === 'F5' || (ctrlOrCmd && (e.key === 'r' || e.key === 'R' || e.key === 'к' || e.key === 'К'))) {
            e.preventDefault();
            window.location.reload();
        }
        // Быстрые сервисы: Ctrl + 1..4
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
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            setupYouTubeAdBlock();
            setupGenericCleaners();
            setTimeout(forceHighQuality, 1500);
        });
    } else {
        setupYouTubeAdBlock();
        setupGenericCleaners();
        setTimeout(forceHighQuality, 1500);
    }
})();
"#;
