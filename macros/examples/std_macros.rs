// Macros from std lib: https://doc.rust-lang.org/std/#macros
// `cargo expand` usage

use std::panic;

fn main() {
    println!("This file is: {}", file!());
    one::two::three::module_path();

    assertions();
    panics();
}

fn assertions() {
    let msg = "Hello";
    assert!(msg.contains("o"));
    assert_eq!(msg, "Hello");
    assert_ne!(msg, "Bye");

    // Debug assertions are only enabled in debug mode (unless `-C debug-assertions` is passed to the compiler for release builds)
    debug_assert!(msg.contains("H"));
    debug_assert_eq!(msg, "Hello");
    debug_assert_ne!(msg, "Bye");
}

fn panics() {
    let _panic = panic::catch_unwind(|| panic!("Panicking!"));
    let _todo = panic::catch_unwind(|| todo!("TODO"));
    let _unimplemented = panic::catch_unwind(|| unimplemented!("Unimplemented"));

    #[cfg(windows)]
    compile_error!("Windows is not supported");
}

mod one {
    pub mod two {
        pub mod three {
            pub fn module_path() {
                println!("The module path is {}", module_path!());
            }
        }
    }
}
