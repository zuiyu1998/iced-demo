use iced::button;
use iced::Align;
use iced::Button;
use iced::Column;
use iced::Element;
use iced::Row;
use iced::Sandbox;
use iced::Settings;
use iced::Text;

#[derive(Default, Clone)]
struct Counter {
    value: i32,
    increment: button::State,
    decrement: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("lw-Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        //Column的文档在iced-native
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}

#[derive(Default)]
struct ManyCounters {
    counters: Vec<Counter>,
}

#[derive(Debug, Clone, Copy)]
enum ManyMessage {
    Counter(usize, Message),
}

impl Sandbox for ManyCounters {
    type Message = ManyMessage;

    fn new() -> Self {
        ManyCounters {
            counters: vec![Counter::default(); 4],
        }
    }

    fn title(&self) -> String {
        String::from("lw-Counter")
    }

    fn update(&mut self, message: ManyMessage) {
        match message {
            ManyMessage::Counter(index, counter_msg) => {
                if let Some(counter) = self.counters.get_mut(index) {
                    counter.update(counter_msg);
                }
            }
        }
    }

    fn view(&mut self) -> Element<ManyMessage> {
        self.counters
            .iter_mut()
            .enumerate()
            .fold(Row::new().spacing(20), |row, (index, counter)| {
                let element: Element<Message> = counter.view().into();
                row.push(element.map(move |message| ManyMessage::Counter(index, message)))
            })
            .into()
    }
}

pub fn main() {
    ManyCounters::run(Settings::default())
}
