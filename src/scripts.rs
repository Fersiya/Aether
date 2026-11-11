//! Встроенный рантайм Aether Browser v0.5.0:
//! Полная обработка горячих клавиш (Ctrl+L, Ctrl+P, Ctrl+M, Ctrl+Space, Ctrl+1..4, Ctrl+D),
//! автоматический пропуск рекламы на YouTube, Picture-in-Picture и извлечение медиа.

pub const AETHER_CORE_SCRIPT: &str = r#"
(() => {
    if (window.__AETHER_INJECTED__) return;
    window.__AETHER_INJECTED__ = true;
    console.log("⚡ Aether Engine v0.5.0: Ядро управления и защиты успешно подключено");

    // 1. Автоматический пропуск рекламы на YouTube и скрытие баннеров
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

    // 2. Очистка баннеров на обычных сайтах
    function setupGenericCleaners() {
        const adSelectors = ['.adsbygoogle', 'div[id^="ad_"]', 'div[class*="banner-ad"]', '#cookie-banner', '.cookie-notice'];
        try {
            document.querySelectorAll(adSelectors.join(', ')).forEach(el => el.style.display = 'none');
        } catch (_) {}
    }

    // 3. Форсирование высокого качества 1080p+ на YouTube
    function forceHighQuality() {
        if (!location.hostname.includes("youtube.com")) return;
        const player = document.getElementById('movie_player');
        if (player && typeof player.setPlaybackQualityRange === 'function') {
            player.setPlaybackQualityRange('hd1080', 'highres');
        }
    }

    // 4. Умный омнибокс Aether
    function openOmnibox() {
        const input = prompt(
            "Aether Omnibox (Префиксы: 'yt: запрос', 'vk: запрос', 'gh: запрос' или обычный поиск в Google):",
            window.location.href
        );
        if (!input) return;

        const trimmed = input.trim();
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

    // 5. Медиа функции: Play/Pause, PiP, Audio-Only, Скачивание
    function togglePlayPause() {
        if (window.vk && window.vk.audio) {
            const playBtn = document.querySelector('.top_audio_player_play, .AudioPlayer__play');
            if (playBtn) { playBtn.click(); return; }
        }
        const media = document.querySelector('video, audio');
        if (media) {
            if (media.paused) { media.play(); } else { media.pause(); }
        }
    }

    function togglePiP() {
        const video = document.querySelector('video');
        if (!video) {
            alert("Видео на странице не найдено.");
            return;
        }
        if (document.pictureInPictureElement) {
            document.exitPictureInPicture().catch(console.error);
        } else if (document.pictureInPictureEnabled) {
            video.requestPictureInPicture().catch(console.error);
        }
    }

    function toggleAudioOnly() {
        const video = document.querySelector('video');
        if (!video) {
            alert("Видео на странице не найдено.");
            return;
        }
        if (video.style.opacity === '0.001') {
            video.style.opacity = '1';
            video.style.visibility = 'visible';
            console.log("Aether: Обычный режим видео");
        } else {
            video.style.opacity = '0.001';
            console.log("Aether: Режим 'Только аудио' включен (GPU разгружен)");
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
            return;
        }
        const cmd = `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best" "${curUrl}"`;
        navigator.clipboard.writeText(cmd).then(() => {
            alert(`Ссылка скопирована!\n\nКоманда для скачивания через yt-dlp:\n${cmd}`);
        }).catch(() => {
            alert(`Ссылка на видео:\n${curUrl}`);
        });
    }

    // 6. Кроссплатформенная обработка горячих клавиш (Windows, Linux, macOS)
    window.addEventListener('keydown', (e) => {
        const ctrlOrCmd = e.ctrlKey || e.metaKey;

        // F6 или Ctrl+L — Омнибокс
        if (e.key === 'F6' || (ctrlOrCmd && (e.key === 'l' || e.key === 'L' || e.key === 'д' || e.key === 'Д'))) {
            e.preventDefault();
            openOmnibox();
        }
        // Ctrl+Space — Пауза / Воспроизведение
        else if (ctrlOrCmd && e.code === 'Space') {
            e.preventDefault();
            togglePlayPause();
        }
        // Ctrl+P — Picture-in-Picture
        else if (ctrlOrCmd && (e.key === 'p' || e.key === 'P' || e.key === 'з' || e.key === 'З')) {
            e.preventDefault();
            togglePiP();
        }
        // Ctrl+M — Режим 'Только аудио'
        else if (ctrlOrCmd && (e.key === 'm' || e.key === 'M' || e.key === 'ь' || e.key === 'Ь')) {
            e.preventDefault();
            toggleAudioOnly();
        }
        // Ctrl+D — Скачать видео
        else if (ctrlOrCmd && (e.key === 'd' || e.key === 'D' || e.key === 'в' || e.key === 'В')) {
            e.preventDefault();
            downloadMedia();
        }
        // Ctrl+1 — Google
        else if (ctrlOrCmd && e.key === '1') {
            e.preventDefault();
            window.location.href = "https://www.google.com";
        }
        // Ctrl+2 — YouTube
        else if (ctrlOrCmd && e.key === '2') {
            e.preventDefault();
            window.location.href = "https://www.youtube.com";
        }
        // Ctrl+3 — ВКонтакте
        else if (ctrlOrCmd && e.key === '3') {
            e.preventDefault();
            window.location.href = "https://vk.com";
        }
        // Ctrl+4 — Telegram Web
        else if (ctrlOrCmd && e.key === '4') {
            e.preventDefault();
            window.location.href = "https://web.telegram.org";
        }
    });

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
