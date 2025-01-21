fn main() {
    // I want to call the eat() method on a collection of different types implementing the Hungry trait
    let entities: Vec<Box<dyn Hungry>> = vec![
        Box::new(Cat {
            name: "Riki".to_string(),
        }),
        Box::new(Dog {
            name: "Fooki".to_string(),
        }),
    ];
    for entity in entities {
        entity.eat(Food::Apple);
        entity.eat(Food::Banana);
    }
}

#[derive(Debug)]
enum Food {
    Apple,
    Banana,
}

trait Hungry {
    fn eat(&self, food: Food);
}

struct Cat {
    name: String,
}

impl Hungry for Cat {
    fn eat(&self, food: Food) {
        println!("The cat named {} eats a(n) {:?}, meow!", self.name, food);
    }
}

struct Dog {
    name: String,
}

impl Hungry for Dog {
    fn eat(&self, food: Food) {
        println!("The dog named {} eats a(n) {:?}, woof!", self.name, food);
    }
}
