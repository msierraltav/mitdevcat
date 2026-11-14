mod components;

use components::hero::Hero;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Hero {}
    }
}
