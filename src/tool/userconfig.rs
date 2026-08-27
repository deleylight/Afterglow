use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum WindowSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone)]
pub enum ConfigMessage {
    Loaded,
    Saved,
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
        // 2. 写入文件（必要时创建父目录）
        let path = Path::new(path);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        println!("{:?}", xml);
        fs::write(path, xml)?;
        Ok(())
    }
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. 读取文件内容
        let xml = fs::read_to_string(path)?;
        // 2. 将 XML 字符串反序列化为 UserConfig
        let config: UserConfig = from_str(&xml)?;
        println!("{:?}", config);
        Ok(config)
    }
    pub fn config_path() -> String {
        let exe_path = env::current_exe().expect("无法获取程序路径");
        let exe_dir = exe_path.parent().expect("无法获取程序所在目录");
        let config_path = exe_dir.join("config.xml");
        config_path
            .to_str()
            .expect("路径包含非 UTF-8 字符")
            .to_string()
    }

    pub fn new() -> Self {
        let path_str = Self::config_path();
        match Self::load(&path_str) {
            Ok(config) => config,
            Err(_) => {
                let config = Self::default();
                // 文件不存在或损坏时写出默认 XML，整套存储第一次就能落地
                let _ = config.save(&path_str);
                config
            }
        }
    }
    //默认构造
    pub fn default() -> Self {
        Self {
            window_size: WindowSize::Medium,
            language: Language::Cn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn save_load_roundtrip() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("afterglow-config-{stamp}.xml"));
        let path_str = path.to_str().unwrap();
        let config = UserConfig::default();
        config.save(path_str).expect("save xml");
        let loaded = UserConfig::load(path_str).expect("load xml");
        let _ = fs::remove_file(&path);
        assert!(matches!(loaded.window_size, WindowSize::Medium));
        assert!(matches!(loaded.language, Language::Cn));
    }
}
