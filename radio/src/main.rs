use iced::Element;
use iced::Radio;
use iced::Sandbox;
use iced::Settings;

struct HelloRadio {
    value: Option<Choice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    RadioSelected(Choice),
}

impl Sandbox for HelloRadio {
    type Message = Message;

    fn new() -> Self {
        HelloRadio { value: None }
    }

    fn title(&self) -> String {
        String::from("HelloRadio")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::RadioSelected(v) => {
                if self.value == Some(v) {
                    self.value = None;
                } else {
                    self.value = Some(v);
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Radio::new(Choice::A, "A", self.value, Message::RadioSelected).into()
    }
}

fn main() {
    HelloRadio::run(Settings::default());
}
