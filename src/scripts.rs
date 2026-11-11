//! Скрипты Aether Browser v0.5.0:
//! Блокировка рекламы, медиа-контроль, PiP, режим 'Только аудио' и скачивание медиа.

pub const AETHER_CORE_SCRIPT: &str = r#"
(() => {
    if (window.__AETHER_INJECTED__) return;
    window.__AETHER_INJECTED__ = true;
    console.log("⚡ Aether Engine v0.5.0: Ядро защиты и оптимизации активно");

    // 1. Автоматический пропуск рекламы на YouTube и скрытие промо-блоков
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

    // 2. Очистка баннеров на остальных сайтах
    function setupGenericCleaners() {
        const adSelectors = ['.adsbygoogle', 'div[id^="ad_"]', 'div[class*="banner-ad"]', '#cookie-banner', '.cookie-notice'];
        try {
            document.querySelectorAll(adSelectors.join(', ')).forEach(el => el.style.display = 'none');
        } catch (_) {}
    }

    // 3. Форсирование высокого качества видео (1080p+)
    function forceHighQuality() {
        if (!location.hostname.includes("youtube.com")) return;
        const player = document.getElementById('movie_player');
        if (player && typeof player.setPlaybackQualityRange === 'function') {
            player.setPlaybackQualityRange('hd1080', 'highres');
        }
    }

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

/// Воспроизведение / Пауза (YouTube, ВК, универсальный HTML5)
pub const TOGGLE_PLAY_PAUSE_SCRIPT: &str = r#"
(() => {
    if (window.vk && window.vk.audio) {
        const playBtn = document.querySelector('.top_audio_player_play, .AudioPlayer__play');
        if (playBtn) { playBtn.click(); return; }
    }
    const media = document.querySelector('video, audio');
    if (media) {
        if (media.paused) { media.play(); } else { media.pause(); }
    }
})();
"#;

/// Режим 'Только аудио'
pub const TOGGLE_AUDIO_ONLY_SCRIPT: &str = r#"
(() => {
    const video = document.querySelector('video');
    if (!video) {
        alert("Видео на странице не обнаружено.");
        return;
    }
    if (video.style.opacity === '0.001') {
        video.style.opacity = '1';
        video.style.visibility = 'visible';
    } else {
        video.style.opacity = '0.001';
    }
})();
"#;

/// Режим Picture-in-Picture
pub const TOGGLE_PIP_SCRIPT: &str = r#"
(() => {
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
})();
"#;

/// Быстрое скачивание / получение ссылки на медиапоток (Ctrl + D)
pub const DOWNLOAD_MEDIA_SCRIPT: &str = r#"
(() => {
    const curUrl = window.location.href;
    const video = document.querySelector('video');
    
    // Если на странице есть прямой видеофайл
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

    // Для YouTube и ВК — копируем команду скачивания в буфер обмена и показываем подсказку
    const cmd = `yt-dlp -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best" "${curUrl}"`;
    navigator.clipboard.writeText(cmd).then(() => {
        alert(`Ссылка на видео скопирована в буфер!\n\nГотовая команда для мгновенной загрузки:\n${cmd}`);
    }).catch(() => {
        alert(`Ссылка на страницу для загрузки:\n${curUrl}`);
    });
})();
"#;
