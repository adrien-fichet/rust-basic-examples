// https://doc.rust-lang.org/reference/macros.html

#[macro_export]
macro_rules! print_args {
    ( $( $x: expr),* ) => { // any number of args separated by commas
        $( // for each arg stored in $x
            println!("{}", $x);
        )*
    }
}

fn main() {
    print_args!();
    print_args!(1);
    print_args!(2, 3);

    let cat = Cat {
        name: "Riki".to_string(),
    };
    print_args!(cat); // won't compile if Cat does not implement Display
}

struct Cat {
    name: String,
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A cat named {}", self.name)
    }
}
