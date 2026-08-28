use super::userconfig::{Language, UserConfig, WindowSize};
use iced::window::Settings;
use iced::Size;

pub fn size(window_size: WindowSize) -> Size {
    match window_size {
        WindowSize::Small => Size::new(960.0, 540.0),
        WindowSize::Medium => Size::new(1280.0, 720.0),
        WindowSize::Large => Size::new(1600.0, 900.0),
    }
}

pub fn title(language: Language) -> String {
    match language {
        Language::Cn => "余晖".to_string(),
        Language::En => "After glow".to_string(),
    }
}

pub fn settings(config: &UserConfig) -> Settings {
    Settings {
        size: size(config.window_size),
        ..Settings::default()
    }
}
