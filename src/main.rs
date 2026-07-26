use afterglow::MainWindow;
use afterglow::window;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new();
    window::manager::window_run(main_window)
}
