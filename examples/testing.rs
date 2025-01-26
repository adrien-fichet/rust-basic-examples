// Usage: cargo test --example testing -- --test-threads=5

// property-based testing
// fuzzing
// mocking
// benchmarks: timeit, criterion
// profiling
// coverage
// fixtures
// async tests

use rstest::{fixture, rstest};

fn adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        assert_eq!(adder(1, 3), 4);
    }

    #[fixture]
    fn users() -> [&'static str; 4] {
        ["Alice", "Bob", "Carol", "Dave"]
    }

    #[rstest]
    fn test_number_of_users(users: [&'static str; 4]) {
        assert_eq!(users.len(), 4);
    }
}

fn main() {}
