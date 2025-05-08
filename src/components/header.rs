use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "header",
            p { "MitDevcat 🫠"}
          }
    }
}
