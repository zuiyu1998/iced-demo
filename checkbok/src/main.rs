use iced::checkbox;
use iced::Element;
use iced::Sandbox;
use iced::Settings;

struct HelloCheckBox {
    state: bool,
}

#[derive(Debug, Clone)]
enum Message {
    CheckboxToggled(bool),
}

impl Sandbox for HelloCheckBox {
    type Message = Message;

    fn new() -> Self {
        HelloCheckBox { state: false }
    }

    fn title(&self) -> String {
        String::from("CheckBox")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::CheckboxToggled(s) => self.state = s,
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        checkbox::Checkbox::new(self.state, "toggle me", Message::CheckboxToggled)
            .size(50)
            .into()
    }
}

fn main() {
    HelloCheckBox::run(Settings::default())
}
