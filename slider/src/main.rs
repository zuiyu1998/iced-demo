use iced::slider;
use iced::Element;
use iced::Sandbox;
use iced::Settings;

struct HelloSlider {
    state: slider::State,
    value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    SliderChanged(f32),
}

impl Sandbox for HelloSlider {
    type Message = Message;

    fn new() -> Self {
        HelloSlider {
            state: slider::State::default(),
            value: 0.0,
        }
    }

    fn title(&self) -> String {
        String::from("HelloSlider")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SliderChanged(f) => {
                self.value = f;
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        slider::Slider::new(
            &mut self.state,
            0.0..=100.0,
            self.value,
            Message::SliderChanged,
        )
        .into()
    }
}

fn main() {
    HelloSlider::run(Settings::default());
}
