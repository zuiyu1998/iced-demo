use iced::Column;
use iced::Element;
use iced::Sandbox;
use iced::Settings;
use iced::Text;

struct HelloColumn;

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for HelloColumn {
    type Message = Message;

    fn new() -> Self {
        HelloColumn
    }

    fn title(&self) -> String {
        String::from("Column")
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("hello"))
            .push(Text::new("hello"))
            .spacing(20)
            .into()
    }
}

fn main() {
    HelloColumn::run(Settings::default());
}
