use iced::widget::Text;
use iced::Element;
use iced::Sandbox;
use iced::Settings;

struct HelloText;

impl Sandbox for HelloText {
    type Message = ();

    fn new() -> Self {
        HelloText
    }

    fn title(&self) -> String {
        String::from("HelloText")
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}

fn main() {
    HelloText::run(Settings::default());
}
