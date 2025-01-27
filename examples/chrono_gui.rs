use iced::widget::{button, column, row, text};
use iced::Element;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    iced_app()?;
    Ok(())
}

fn iced_app() -> iced::Result {
    // todo: configure basic window settings
    iced::run("Chrono", chrono_update, chrono_view)?;
    Ok(())
}

#[derive(Debug, Clone)]
enum ChronoMessage {
    StartStop,
    Pause,
    Reset,
}

fn chrono_update(chrono: &mut Duration, message: ChronoMessage) {
    match message {
        ChronoMessage::StartStop => {
            // todo: start and stop chrono in a thread instead
            *chrono += Duration::from_secs(1);
        }
        ChronoMessage::Pause => *chrono = Duration::from_secs(1),
        ChronoMessage::Reset => *chrono = Duration::from_secs(0),
    }
}

fn chrono_view(chrono: &Duration) -> Element<ChronoMessage> {
    column![
        text(chrono.as_secs()).size(50),
        row![
            button("Start/Stop").on_press(ChronoMessage::StartStop),
            button("Pause").on_press(ChronoMessage::Pause),
            button("Reset").on_press(ChronoMessage::Reset),
        ]
        .spacing(20),
    ]
    .spacing(20)
    .into()
}
