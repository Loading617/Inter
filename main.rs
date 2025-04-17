use iced::{
    executor, theme, widget::{button, column, container, row, scrollable, text, text_input},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

pub fn main() -> iced::Result {
    Messenger::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    SendPressed,
}

struct Messenger {
    messages: Vec<String>,
    input_value: String,
    scroll: iced::widget::scrollable::Id,
}

impl Application for Messenger {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Messenger {
                messages: Vec::new(),
                input_value: String::new(),
                scroll: iced::widget::scrollable::Id::unique(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Inter")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::SendPressed => {
                if !self.input_value.trim().is_empty() {
                    self.messages.push(self.input_value.trim().to_string());
                    self.input_value.clear();
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let messages_view = self
            .messages
            .iter()
            .cloned()
            .fold(column!().spacing(10), |col, msg| {
                col.push(text(msg))
            });

        let scroll = scrollable(messages_view)
            .id(self.scroll.clone())
            .height(Length::Fill)
            .padding(10);

        let input_row = row![
            text_input("Type your message...", &self.input_value, Message::InputChanged)
                .padding(10)
                .size(16)
                .width(Length::Fill),
            button("Send")
                .on_press(Message::SendPressed)
                .padding(10)
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let content = column![scroll, input_row]
            .spacing(20)
            .padding(20)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}
