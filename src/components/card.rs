use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div{
            class:"card",
            p { "card"}
        }
    }
}
