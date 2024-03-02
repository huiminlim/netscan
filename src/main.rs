use iced::widget::{container, text};
use iced::executor;
use iced::{Application, Element, Settings, Theme, Command, window};

#[derive(Default)]
struct IcedTwentyOne;

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Application for IcedTwentyOne {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (IcedTwentyOne, Command<Self::Message>) {
        (IcedTwentyOne::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Iced Twenty-One")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let hello = text("Hello, Iced! (with application trait)");
        container(hello).into()
    }
}

pub fn main() -> iced::Result {
    let settings = Settings {
            window: window::Settings {
                size: iced::Size{
                    width: 300.0,
                    height: 200.0
                },
                position: iced::window::Position::Default,
                resizable: true,
                decorations: true,
            ..Default::default()
            },
            ..Default::default()
        };
    IcedTwentyOne::run(settings)
}