use iced::progress_bar::ProgressBar;
use iced::Element;
use iced::Sandbox;
use iced::Settings;

struct HelloProgressBar {
    value: f32,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for HelloProgressBar {
    type Message = Message;

    fn new() -> Self {
        HelloProgressBar { value: 50.0 }
    }

    fn title(&self) -> String {
        String::from("HelloProgressBar")
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        ProgressBar::new(0.0..=100.0, self.value).into()
    }
}

fn main() {
    HelloProgressBar::run(Settings::default());
}
