use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, fs};

const DEFAULT_CONFIG_PATH: &str = "config.xml";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "Config", rename_all = "PascalCase")]
pub struct UserConfig {
    pub timer: TimerConfig,
    pub settings: SettingsConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimerConfig {
    pub focus_minutes: u32,
    pub break_minutes: u32,
    pub total_rounds: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SettingsConfig {
    pub language: String,
    pub notification_enabled: bool,
    pub background_mode: bool,
}

#[derive(Debug)]
pub enum UserConfigError {
    Io(std::io::Error),
    XmlDe(quick_xml::DeError),
    XmlSe(quick_xml::SeError),
}

impl fmt::Display for UserConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "config file error: {error}"),
            Self::XmlDe(error) => write!(formatter, "invalid config XML: {error}"),
            Self::XmlSe(error) => write!(formatter, "could not serialize config XML: {error}"),
        }
    }
}

impl Error for UserConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::XmlDe(error) => Some(error),
            Self::XmlSe(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for UserConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<quick_xml::DeError> for UserConfigError {
    fn from(error: quick_xml::DeError) -> Self {
        Self::XmlDe(error)
    }
}

impl From<quick_xml::SeError> for UserConfigError {
    fn from(error: quick_xml::SeError) -> Self {
        Self::XmlSe(error)
    }
}

impl UserConfig {
    pub fn load() -> Result<Self, UserConfigError> {
        match fs::read_to_string(DEFAULT_CONFIG_PATH) {
            Ok(xml) => Ok(from_str(&xml)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let config = Self::default();
                config.save()?;
                Ok(config)
            }
            Err(error) => Err(error.into()),
        }
    }

    pub fn save(&self) -> Result<(), UserConfigError> {
        let xml = to_string(self)?;
        fs::write(
            DEFAULT_CONFIG_PATH,
            format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{xml}\n"),
        )?;
        Ok(())
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            timer: TimerConfig {
                focus_minutes: 30,
                break_minutes: 5,
                total_rounds: 2,
            },
            settings: SettingsConfig {
                language: "zh-CN".to_owned(),
                notification_enabled: true,
                background_mode: true,
            },
        }
    }
}
