use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            h1 { "MitDevcat v2" }
            p { "Sitio en construcción, escrito a mano" }
        }
    }
}
