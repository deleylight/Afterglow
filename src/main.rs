use afterglow::tool::userconfig::{self, UserConfig};
fn main() -> iced::Result {
    iced::application(Aftergolw::new, update, view)
        .title("After glow")
        .run()
}

struct Aftergolw {
    user_config: UserConfig,
}

impl Aftergolw {
    fn new() {
        UserConfig::new();
    }
}
