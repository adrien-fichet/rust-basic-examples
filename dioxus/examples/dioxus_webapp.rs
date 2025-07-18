// cd dioxus
// dx serve --example dioxus_webapp

use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("./assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        "Dioxus webapp!"
    }
}
