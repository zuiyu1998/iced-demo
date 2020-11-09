use iced::button::State;
use iced::Button;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Text;

struct HelloButton {
    pressed: State,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Sandbox for HelloButton {
    type Message = Message;

    fn new() -> Self {
        HelloButton {
            pressed: State::default(),
        }
    }

    fn title(&self) -> String {
        String::from("登录")
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        Button::new(
            &mut self.pressed,
            Text::new("hello")
                .on_press(Self::Message::ButtonPressed)
                .into(),
        )
        }
    }
}

fn main() {
    HelloButton::run(Settings::default())
}
