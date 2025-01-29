use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, row, text};
use iced::Subscription;
use iced::{Element, Length, Size};
use std::time::Duration;

const TICK_INTERVAL: Duration = Duration::from_millis(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    iced_app()?;
    Ok(())
}

fn iced_app() -> iced::Result {
    iced::application("Chrono", update, view)
        .subscription(tick)
        .window_size(Size::new(400.0, 200.0))
        .run()
}

fn tick(_state: &State) -> Subscription<Message> {
    iced::time::every(TICK_INTERVAL).map(|_| Message::Tick)
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Start,
    Pause,
    Resume,
    Reset,
}

struct State {
    chrono: Duration,
    started_at: Option<std::time::Instant>,
    paused: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            chrono: Duration::ZERO,
            started_at: None,
            paused: true, // chrono should not start automatically
        }
    }
}

impl State {
    fn start(&mut self) {
        self.chrono = Duration::ZERO;
        self.started_at = Some(std::time::Instant::now());
        self.paused = false;
    }

    fn resume(&mut self) {
        self.paused = false;
    }

    fn pause(&mut self) {
        self.paused = true;
    }

    fn reset(&mut self) {
        self.chrono = Duration::ZERO;
        self.started_at = None;
        self.paused = true;
    }
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Tick => {
            if !state.paused {
                state.chrono += TICK_INTERVAL;
            }
        }
        Message::Start => state.start(),
        Message::Pause => state.pause(),
        Message::Resume => state.resume(),
        Message::Reset => state.reset(),
    }
}

fn view(state: &State) -> Element<Message> {
    let mut start_button = button("Start");
    if state.paused && state.started_at.is_none() {
        start_button = start_button.on_press(Message::Start);
    }

    let mut pause_button = button("Pause");
    if !state.paused {
        pause_button = pause_button.on_press(Message::Pause);
    }

    let mut resume_button = button("Resume");
    if state.started_at.is_some() && state.paused {
        resume_button = resume_button.on_press(Message::Resume);
    }

    let mut reset_button = button("Reset");
    if state.started_at.is_some() {
        reset_button = reset_button.on_press(Message::Reset);
    }

    row![column![
        text(format!("{}ms", state.chrono.as_millis()))
            .size(60)
            .width(Length::Fill)
            .center(),
        row![start_button, pause_button, resume_button, reset_button,].spacing(10)
    ]
    .spacing(20)
    .align_x(Horizontal::Center)]
    .align_y(Vertical::Center)
    .height(Length::Fill)
    .into()
}
