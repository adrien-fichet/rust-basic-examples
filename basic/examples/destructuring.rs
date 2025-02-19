fn main() {
    let cat = Cat {
        name: "Riki".to_string(),
        age: 4,
    };

    let Cat { name, age } = cat.clone(); // same variable names as the struct fields
    println!("{} is {} years old", name, age);

    let Cat {
        name: cat_name,
        age: cat_age,
    } = cat; // new variable names
    println!("{} is {} years old", cat_name, cat_age);
}

#[derive(Clone)]
struct Cat {
    name: String,
    age: u8,
}
