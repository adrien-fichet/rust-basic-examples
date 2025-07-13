use std::fmt::Display;

fn main() {
    let cat = Cat::new("Riki".to_string(), 4)
        .with_main_color(Color::White)
        .with_secondary_color(Color::Black)
        .lazy(false);
    println!("{cat}");
}

#[derive(Default, Debug)]
enum Color {
    #[default]
    Black,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Black => "Black",
                Color::White => "White",
            }
        )
    }
}

#[derive(Default, Debug)]
struct Cat {
    name: String,
    age: u8,
    main_color: Color,
    secondary_color: Color,
    is_lazy: bool,
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "--- Cat infos ---\nName: {}\nAge: {}\nMain Color: {}\nSecondary Color: {}\nLazy: {}",
            self.name, self.age, self.main_color, self.secondary_color, self.is_lazy
        )
    }
}

impl Cat {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
            ..Default::default()
        }
    }

    fn with_main_color(mut self, color: Color) -> Self {
        self.main_color = color;
        self
    }

    fn with_secondary_color(mut self, color: Color) -> Self {
        self.secondary_color = color;
        self
    }

    fn lazy(mut self, is_lazy: bool) -> Self {
        self.is_lazy = is_lazy;
        self
    }
}
