use iced::widget::{button, column, container, text};
use iced::Element;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    using_iced()?;
    //using_egui()?;
    Ok(())
}

fn using_egui() {
    todo!();
}

fn using_iced() -> iced::Result {
    iced::run("Chrono", iced_update, iced_view)?;
    Ok(())
}

#[derive(Debug, Clone)]
enum IcedMessage {
    Increment,
}

fn iced_update(counter: &mut u64, message: IcedMessage) {
    match message {
        IcedMessage::Increment => *counter += 1,
    }
}

fn iced_view(counter: &u64) -> Element<IcedMessage> {
    column![
        text(counter).size(20),
        button("Increment").on_press(IcedMessage::Increment),
    ]
    .spacing(10)
    .into()
}
