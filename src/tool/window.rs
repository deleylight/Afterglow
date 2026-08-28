use super::userconfig::{Language, UserConfig, WindowSize};
use iced::window::Settings;
use iced::Size;
use std::env;
use std::path::PathBuf;

pub fn size(window_size: WindowSize) -> Size {
    match window_size {
        WindowSize::Small => Size::new(960.0, 540.0),
        WindowSize::Medium => Size::new(1280.0, 720.0),
        WindowSize::Large | WindowSize::Fullscreen => Size::new(1600.0, 900.0),
    }
}

pub fn title(language: Language) -> String {
    match language {
        Language::Cn => "余晖".to_string(),
        Language::En => "After glow".to_string(),
    }
}

pub fn background(window_size: WindowSize) -> PathBuf {
    let file = match window_size {
        WindowSize::Small => "960x540.jpg",
        WindowSize::Medium => "1280x720.jpg",
        WindowSize::Large | WindowSize::Fullscreen => "1600x900.jpg",
    };

    let mut relative = PathBuf::from("img");
    relative.push("bg");
    relative.push(file);

    if let Ok(cwd) = env::current_dir() {
        let path = cwd.join(&relative);
        if path.exists() {
            return path;
        }
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            let path = dir.join(&relative);
            if path.exists() {
                return path;
            }
        }
    }

    relative
}

pub fn settings(config: &UserConfig) -> Settings {
    let size = size(config.window_size);
    let fullscreen = matches!(config.window_size, WindowSize::Fullscreen);

    Settings {
        size,
        resizable: false,
        maximized: false,
        fullscreen,
        min_size: (!fullscreen).then_some(size),
        max_size: (!fullscreen).then_some(size),
        ..Settings::default()
    }
}
