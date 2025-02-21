use colored::{Colorize, CustomColor};

fn main() {
    println!("The following text is red: {}", "<red text>".red());
    println!("{}", "Cyan underline".underline().cyan());
    println!("{}", "Yellow bold with Purple background".bold().on_purple().yellow());
    println!("No color");

    let style = |s: &str| s.white().on_black().italic();
    println!("{}", style("With a predefined style"));

    let steel_blue = CustomColor::new(70, 130, 180);
    println!("{}", "Steel blue".custom_color(steel_blue));
}
