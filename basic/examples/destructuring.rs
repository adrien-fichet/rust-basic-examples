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
}

#[derive(Clone, Debug)]
struct Cat {
    name: String,
    age: u8,
}
