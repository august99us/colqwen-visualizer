use std::fs;

use iced::{Element, Length};
use iced::widget::{button, column, image, scrollable, text, Column};

pub fn main() -> iced::Result {
    iced::run("A cool counter", update, view)
}

fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
        Message::Decrement => *counter -= 1,
    }
}

fn view(counter: &u64) -> Element<Message> {
    let children = fs::read_dir("test_imgs").unwrap().map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            image(path)
                .into()
        } else {
            text("Not a file").into()
        }
    });

    // scrollable(column![
    //         button(text("Increment"))
    //             .on_press(Message::Increment),
    //         button(text("Decrement"))
    //             .on_press(Message::Decrement),
    //         text(format!("Counter: {}", counter)),
    // ]);
    scrollable(Column::with_children(children))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}