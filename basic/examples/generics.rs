use serde::Serialize;

#[derive(Serialize)]
struct Cat {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct Plant {
    name: String,
    color: String,
}

fn use_something<T>(obj: T)
where
    T: Serialize,
{
    dbg!(&serde_json::to_string(&obj).unwrap());
}

fn main() {
    let riki = Cat {
        name: "Riki".to_string(),
        age: 4,
    };
    use_something(&riki);

    let cactus = Plant {
        name: "Cactus".to_string(),
        color: "green".to_string(),
    };
    use_something(&cactus);
}
