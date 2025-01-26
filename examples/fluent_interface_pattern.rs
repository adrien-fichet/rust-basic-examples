#[derive(Default, Debug)]
enum Color {
    #[default]
    Black,
    White,
    Brown,
}

#[derive(Default, Debug)]
struct Cat {
    name: String,
    age: u8,
    main_color: Color,
    secondary_color: Color,
    is_lazy: bool,
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

fn main() {
    let cat = Cat::new("Riki".to_string(), 4)
        .with_main_color(Color::White)
        .with_secondary_color(Color::Black)
        .lazy(false);
    println!("{:?}", cat);
}
