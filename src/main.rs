use afterglow::tool::userconfig::*;
use afterglow::tool::window;
use iced::widget::{button, column};

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
enum Message {
    Config(ConfigMessage),
}

impl Aftergolw {
    fn new(user_config: UserConfig) -> Self {
        Self { user_config }
    }

    fn title(&self) -> String {
        window::title(self.user_config.language)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Config(msg) => match msg {
                ConfigMessage::Loaded => match UserConfig::load(&UserConfig::config_path()) {
                    Ok(config) => self.user_config = config,

                    Err(e) => eprintln!("加载配置失败: {e}"),
                },
                ConfigMessage::Saved => {
                    if let Err(e) = self.user_config.save(&UserConfig::config_path()) {
                        eprintln!("保存配置失败: {e}");
                    }
                }
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            button("Save").on_press(Message::Config(ConfigMessage::Saved)),
            button("Load").on_press(Message::Config(ConfigMessage::Loaded)),
        ]
        .spacing(10)
        .into()
    }
}
