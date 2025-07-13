fn main() {
    let cat = Cat {
        name: "Riki".to_string(),
        age: 4,
    };

    let Cat { name, age } = cat.clone(); // same variable names as the struct fields
    println!("{name} is {age} years old");

    let Cat {
        name: cat_name,
        age: cat_age,
    } = cat; // new variable names
    println!("{cat_name} is {cat_age} years old");

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
