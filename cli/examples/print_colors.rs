// https://docs.rs/ansi_term/latest/ansi_term/

use ansi_term::{Colour, Style};

fn main() {
    println!("The following text is red: {}", Colour::Red.paint("<red text>"));

    println!("{}", Colour::Cyan.underline().paint("Cyan underline"));

    println!(
        "{}",
        Colour::Yellow
            .bold()
            .on(Colour::Purple)
            .paint("Yellow bold with Purple background")
    );

    let style = Style::new().fg(Colour::White).on(Colour::Black).italic();
    println!("{}", style.paint("With a predefined style"));

    let steel_blue = Colour::RGB(70, 130, 180);
    println!("{}", steel_blue.paint("Steel blue"));
}
