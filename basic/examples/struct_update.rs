use std::fmt::{Display, Formatter};

fn main() {
    let cat = Cat {
        name: "Riki".to_string(),
        age: 4,
    };

    let cat2 = Cat {
        name: "Rikou".to_string(),
        ..cat.clone()
    };

    println!("{cat2:#?}");
}

#[derive(Clone, Debug)]
struct Cat {
    name: String,
    age: u8,
}

impl Display for Cat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}
