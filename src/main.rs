slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // include_modules!() 已经把 MainWindow 生成并引入进来了
    let main_window = MainWindow::new()?;

    // 运行窗口事件循环
    main_window.run()
}
