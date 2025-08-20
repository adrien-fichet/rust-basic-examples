// cargo test --example setup_teardown -- --no-capture
// Note: before_all is called for each test when using nextest, because each test is running in a separate process

use std::sync::OnceLock;
use test_context::{TestContext, test_context};

#[allow(dead_code)]
static BEFORE_ALL: OnceLock<String> = OnceLock::new();

#[allow(dead_code)]
fn before_all() -> String {
    println!("before all");
    "John".to_string()
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum Job {
    Firefighter,
    Engineer,
}

#[allow(dead_code)]
struct Person {
    name: String,
    job: Job,
}

#[allow(dead_code)]
struct MyTestContext {
    main_character: Person,
    secondary_character: Person,
}

impl TestContext for MyTestContext {
    fn setup() -> Self {
        let main_character_name = BEFORE_ALL.get_or_init(before_all);

        println!("before each");
        Self {
            main_character: Person {
                name: main_character_name.clone(),
                job: Job::Firefighter,
            },
            secondary_character: Person {
                name: "John".to_string(),
                job: Job::Engineer,
            },
        }
    }

    fn teardown(self) {
        println!("after each");
    }
}

#[test_context(MyTestContext)]
#[test]
fn test_001(ctx: &mut MyTestContext) {
    println!("test_001");
    assert_eq!(ctx.main_character.job, Job::Firefighter);
}

#[test_context(MyTestContext)]
#[test]
fn test_002(ctx: &mut MyTestContext) {
    println!("test_002");
    assert_eq!(ctx.secondary_character.name, "John".to_string());
}

fn main() {
    todo!();
}
