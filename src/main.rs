use afterglow::tool::userconfig::*;
use afterglow::tool::window;
use iced::widget::image;
use iced::{ContentFit, Fill};

fn main() -> iced::Result {
    let user_config = UserConfig::new();

    iced::application(
        move || Aftergolw::new(user_config),
        Aftergolw::update,
        Aftergolw::view,
    )
    .title(Aftergolw::title)
    .window(window::settings(&user_config))
    .run()
}

struct Aftergolw {
    user_config: UserConfig,
}

#[derive(Debug, Clone)]
enum Message {}

impl Aftergolw {
    fn new(user_config: UserConfig) -> Self {
        Self { user_config }
    }

    fn title(&self) -> String {
        window::title(self.user_config.language)
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<'_, Message> {
        let fit = match self.user_config.window_size {
            WindowSize::Fullscreen => ContentFit::Cover,
            _ => ContentFit::Fill,
        };

        image(window::background(self.user_config.window_size))
            .width(Fill)
            .height(Fill)
            .content_fit(fit)
            .into()
    }
}
