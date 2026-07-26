use std::fs::OpenOptions;
use std::io;
pub struct UserConfig {}
impl UserConfig {
    pub fn load() -> Self {
        UserConfig::new()
    }
    pub fn save(&self) { /* ... */
    }
    pub fn new() -> Self {
        open_or_create("config.json").unwrap();
        UserConfig {}
    }
}
fn open_or_create(path: &str) -> io::Result<std::fs::File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}
