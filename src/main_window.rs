use iced::executor;
use iced::{Element, Command, Application, Length, Theme};
use iced::widget::{Button, Column, Row, Text, TextInput, Container, scrollable, container};
use native_dialog::FileDialog;
use std::path::PathBuf;
use std::fs;

pub struct MainWindow {
    file_path: PathBuf,
    file_path_buf: String,
    file_output_text: String,
    search_word: String,
    search_word_output: String,
}

struct SearchContainerStyle;

impl container::StyleSheet for SearchContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let mut temp_return = container::Appearance::default();
        temp_return.border_width = 1.0;
        temp_return.border_color = iced::Color::WHITE;

        temp_return
    }

}

#[derive(Debug, Clone)]
pub enum Message {
    UserFilePathUpdate(String),
    UserSearchWordUpdate(String),
    OpenDialog,
    BtnFileSearch,
}

impl MainWindow {
    fn file_search(&self) -> String {
        self.file_output_text
            .lines()
            .filter(|line| line.contains(&self.search_word))
            .collect()
    }
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            MainWindow {
                file_path: PathBuf::new(),
                file_path_buf: String::new(),
                file_output_text: String::new(),
                search_word: String::new(),
                search_word_output: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Grep-Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UserFilePathUpdate(x) => {
                self.file_path_buf = x;
                Command::none()
            }

            Message::UserSearchWordUpdate(x) => {
                self.search_word = x;
                Command::none()
            }

            Message::OpenDialog => {
                self.file_path = match FileDialog::new().set_location("~/").show_open_single_file().unwrap() {
                    Some(path) => path,
                    None => PathBuf::new(),
                };
                self.file_path_buf = self.file_path.to_string_lossy().into_owned();

                Command::none()
            }

            Message::BtnFileSearch => {
                match fs::read_to_string(&self.file_path) {
                    Ok(x) => {
                        self.file_output_text = x;
                        self.search_word_output = self.file_search();
                    }
                    Err(e) => self.file_output_text = format!("Error opening file: {}", e),
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let file_path_input = TextInput::new("File Path:", &self.file_path_buf, Message::UserFilePathUpdate)
            .size(25)
            .width(iced::Length::Units(400))
            .on_submit(Message::BtnFileSearch);
        let file_dialog_button = Button::new("...")
            .on_press(Message::OpenDialog)
            .height(iced::Length::Units(28))
            .width(iced::Length::Units(50));
        let user_main_row = Row::new()
            .push(file_path_input)
            .push(file_dialog_button)
            .width(iced::Length::Fill);

        let file_search_input = TextInput::new("Search word:", &self.search_word, Message::UserSearchWordUpdate)
            .size(25)
            .width(iced::Length::Units(400))
            .on_submit(Message::BtnFileSearch);
        let file_search_button = Button::new("Search")
            .on_press(Message::BtnFileSearch)
            .height(iced::Length::Units(28))
            .width(iced::Length::Units(60));
        let user_search_row = Row::new()
            .push(file_search_input)
            .push(file_search_button);

        let output_text = Text::new(&self.file_output_text).width(iced::Length::Fill);
        let output_text_scroll = scrollable(output_text).height(Length::Units(250));

        let output_text_lbl = Text::new("File:");

        let search_output_text = Text::new(&self.search_word_output).width(iced::Length::Fill);
        let search_output_scroll = scrollable(search_output_text).height(Length::Units(250));

        let output_search_lbl = Text::new("Search:");

        let output_container = Container::new(output_text_scroll)
            .style(iced::theme::Container::Custom(Box::new(SearchContainerStyle)));
        let search_container = Container::new(search_output_scroll)
            .style(iced::theme::Container::Custom(Box::new(SearchContainerStyle)));

        let main_column = Column::new()
            .push(user_main_row)
            .push(user_search_row)
            .push(output_text_lbl)
            .push(output_container)
            .push(output_search_lbl)
            .push(search_container)
            .spacing(15)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill);

        let main_layout = Container::new(main_column);
        main_layout.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
