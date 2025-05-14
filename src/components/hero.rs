use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/Mit-logo-light.png");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: LOGO, id: "hero-logo", height: "300px", width: "300px"}
        }
    }
}
