// Usage: cargo test --example testing

// property-based testing
// fuzzing
// mocking
// benchmarks
// profiling
// coverage
// fixtures

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
    fn users() -> Vec<String> {
        ["Alice", "Bob", "Carol"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[rstest]
    fn test_number_of_users(users: Vec<String>) {
        assert_eq!(users.len(), 3);
    }
}

fn main() {}
