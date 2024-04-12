#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

use iced::widget::{button, column, text, Column};

impl Counter {
    pub fn view(&self) -> Column<Message> {

        // The increment button. We tell it to produce an
        // `Increment` message when pressed
        let plus = button("+").on_press(Message::Increment);

        // We show the value of the counter here
        let txt = text(self.value).size(50);

        // The decrement button. We tell it to produce a
        // `Decrement` message when pressed
        let minus = button("-").on_press(Message::Decrement);
        // We use a column: a simple vertical layout
        let col: Column<Message> = column!(plus, txt, minus);
        col
    }
}

impl Counter {
    // ...

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}