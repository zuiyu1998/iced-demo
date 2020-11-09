use iced::text_input::State;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::TextInput;

struct HelloTextInput {
    state: State,
    value: String,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl Sandbox for HelloTextInput {
    type Message = Message;

    fn new() -> Self {
        HelloTextInput {
            state: State::new(),
            value: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("HelloTextInput")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextInputChanged(s) => {
                self.value = s;
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        TextInput::new(&mut self.state, "hello", &mut self.value, |s| {
            Message::TextInputChanged(s)
        })
        .padding(20)
        .width(iced::Length::Units(100))
        .password()
        .into()
    }
}

fn main() {
    HelloTextInput::run(Settings::default())
}
