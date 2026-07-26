use crate::MainWindow;
use crate::tool::user_config::*;
use slint::ComponentHandle;
//处理窗口并且运行
pub fn window_run(
    widnow_handle: Result<MainWindow, slint::PlatformError>,
) -> Result<(), slint::PlatformError> {
    let window = widnow_handle?;
    //todo这里需要把config完善,并且user_config并没有写完,先要完善那里
    let _config = UserConfig::load().unwrap_or_else(|error| {
        eprintln!("{error}");
        UserConfig::new()
    });
    println!("{:?}", _config.timer.total_rounds);

    window.run()
}
