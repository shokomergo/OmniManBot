use iced::{
    Alignment, Application, Command, Element, Length, Settings,
    widget::{button, column, container, scrollable, text, text_input, Button, Column, Container, Scrollable, Text, TextInput},
};

use rand::seq::SliceRandom;
use rand::thread_rng;

const OMNI_MAN_QUOTES: &[&str] = &[
    "Think, Mark!",
    "You want to die for this planet? Fine. What’s 17 more years? I can always start again… make another kid.",
    "I do love your mother. But she’s more like a… a pet to me.",
    r#"This isn’t your world. It’s theirs. But we can help them. We can stop wars. Eliminate hunger. Give them medical technology centuries ahead of what they have now.
    We’ve already been doing it. If it wasn’t for you and me, this planet would be in flames."#,
    r#"Why did you make me do this? You’re fighting so you can watch everyone around you die! Think, Mark! You’ll outlast every fragile, insignificant being on this planet. 
    You’ll live to see this world crumble to dust and blow away! Everyone and everything you know will be gone!"#,
    "I will burn this planet down. Before I spend another Minute living among these animals!",
    "Are you sure?",
];

#[derive(Clone, Debug)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Clone, Debug)]
enum Role {
    User,
    Bot,
}

#[derive(Clone, Debug)]
enum AppMessage {
    InputChanged(String),
    SendPressed,
}

struct OmniManBot {
    input_value: String,
    messages: Vec<Message>,
}

impl Application for OmniManBot {
    type Message = AppMessage;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            OmniManBot {
                input_value: String::new(),
                messages: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Omni-Man Bot".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::InputChanged(val) => {
                self.input_value = val;
            }
            AppMessage::SendPressed => {
                if !self.input_value.trim().is_empty() {
                    let user_msg = Message {
                        role: Role::User,
                        content: self.input_value.clone(),
                    };
                    self.messages.push(user_msg);

                    let last_bot_msg = self.messages.iter().rev()
                        .find(|m| matches!(m.role, Role::Bot))
                        .map(|m| m.content.as_str())
                        .unwrap_or("");
                    
                    let bot_reply = omni_man_reply(last_bot_msg);
                    let bot_msg = Message {
                        role: Role::Bot,
                        content: bot_reply,
                    };
                    self.messages.push(bot_msg);

                    self.input_value.clear();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut scroll_content = column!().spacing(10);
        for msg in &self.messages {
            let text = match msg.role {
                Role::User => format!("You: {}", msg.content),
                Role::Bot => format!("Omni-Man: {}", msg.content),
            };
            scroll_content = scroll_content.push(Text::new(text));
        }

        let scrollable_content = Container::new(scrollable(scroll_content))
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill);

        let input_row = Column::new()
            .spacing(10)
            .push(
                TextInput::new("Type your message...", &self.input_value)
                    .on_input(AppMessage::InputChanged)
                    .padding(10),
            )
            .push(
                Button::new(Text::new("Send"))
                .on_press(AppMessage::SendPressed)
                .padding(10),
            );

            let content = Column::new()
                .spacing(10)
                .align_items(Alignment::Start)
                .push(scrollable_content)
                .push(input_row);

            Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
                .into()
    }
}

fn omni_man_reply(last_message: &str) -> String {
    let mut rng = thread_rng();
    
    loop {
    let quote = OMNI_MAN_QUOTES.choose(&mut rng).unwrap().to_string();
    if quote != last_message {
        return quote;
        }
    }
}

fn main() -> iced::Result {
    OmniManBot::run(Settings::default())
}