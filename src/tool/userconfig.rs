use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum WindowSize {
    Small,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Language {
    Cn,
    En,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub window_size: WindowSize,
    pub language: Language,
}

impl UserConfig {
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 1. 将 self 序列化为 XML 字符串
        let xml = to_string(&self)?; // ? 表示如果出错就向上返回错误
        // 2. 写入文件
        fs::write(path, xml)?;
        Ok(())
    }
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. 读取文件内容
        let xml = fs::read_to_string(path)?;
        // 2. 将 XML 字符串反序列化为 UserConfig
        let config: UserConfig = from_str(&xml)?;
        Ok(config)
    }
    pub fn new() -> Self {
        let exe_path = env::current_exe().expect("无法获取程序路径");
        let exe_dir = exe_path.parent().expect("无法获取程序所在目录");
        let config_path = exe_dir.join("config.xml");
        let path_str = config_path.to_str().expect("路径包含非 UTF-8 字符");
        let config = Self::load(path_str).unwrap_or_else(|_| Self::default());
        config
    }

    pub fn default() -> Self {
        Self {
            window_size: WindowSize::Medium,
            language: Language::Cn,
        }
    }
}
