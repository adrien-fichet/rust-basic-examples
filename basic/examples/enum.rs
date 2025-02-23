// https://doc.rust-lang.org/stable/std/keyword.enum.html

trait Log {
    fn log(&self);
}

enum Event {
    Click { x: i32, y: i32 },
    Paste(String),
}

impl Log for Event {
    fn log(&self) {
        match self {
            Event::Click { x, y } => {
                println!("Click at ({}, {})", x, y);
            }
            Event::Paste(s) => println!("Pasted \"{}\"", s),
        }
    }
}

fn main() {
    let events = [Event::Click { x: 10, y: 50 }, Event::Paste(String::from("Hello"))];

    events.iter().for_each(|event| event.log());

    let event = Event::Click { x: 10, y: 50 };
    if let Event::Click { x, y } = event {
        println!("Click at ({}, {})", x, y);
    }
}
