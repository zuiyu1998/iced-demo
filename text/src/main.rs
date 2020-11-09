use iced::widget::Text;
use iced::window;
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
        Text::new("世界，你好?")
            .size(40)
            .color([0.0, 0.0, 1.0])
            .into()
    }
}

fn main() {
    let setting = Settings {
        window: window::Settings {
            size: (800, 600),
            resizable: true,
            decorations: true,
        },
        flags: (),
        default_font: Some(include_bytes!("C:\\Windows\\Fonts\\simfang.ttf")),
        antialiasing: true,
    };
    HelloText::run(setting);
}
