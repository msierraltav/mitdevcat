use dioxus::prelude::*;
use web_sys::window;

const LOGO: Asset = asset!("/assets/Mit-logo-light.png");
const ANIMATION_CSS: Asset = asset!("/assets/styles/animations.css");

#[component]
pub fn Hero() -> Element {
    use_effect(|| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                let body = document.body().unwrap();

                if document.get_element_by_id("page-overlay").is_none() {
                    let overlay = document.create_element("div").unwrap();
                    overlay.set_id("page-overlay");
                    overlay.set_class_name("page-overlay");
                    body.append_child(&overlay).unwrap();
                }
            }
        }
    });

    rsx! {
        document::Link{ rel: "stylesheet", href: ANIMATION_CSS}
        div {
            id: "hero",
            class: "hero-container",
            img {
                src: LOGO,
                id: "hero-logo",
                class: "hero-img-visible",
                height: "300px",
                width: "300px"
            }
        }
    }
}
