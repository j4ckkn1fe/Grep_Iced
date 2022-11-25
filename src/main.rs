use iced::{Settings, Application, window};

mod main_window;

fn main() -> iced::Result {
    main_window::MainWindow::run(Settings {
        window: window::Settings {
            size: (600, 600),
            position: (iced::window::Position::Centered),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
