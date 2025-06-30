mod components;
use components::header::Header;
use components::hero::Hero;
use dioxus::prelude::*;

// Contexto para el tema
#[derive(Clone, Copy, PartialEq)]
pub struct ThemeContext {
    pub is_dark: bool,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_CSS: Asset = asset!("/assets/styles/header.css");
const ANIMATION_CSS: Asset = asset!("/assets/styles/animations.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const GITHUB_SVG: Asset = asset!("/assets/github.svg");
const THEME_JS: Asset = asset!("/assets/theme-toggle.js");
pub const MIT_LOGO_LIGHT: Asset = asset!("/assets/Mit-logo-light.png");
pub const MIT_LOGO_DARK: Asset = asset!("/assets/Mit-logo-dark.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme_context = use_signal(|| ThemeContext { is_dark: true });
    
    use_context_provider(|| theme_context);
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: HEADER_CSS }
        document::Link { rel: "stylesheet", href: ANIMATION_CSS }
        document::Script { src: THEME_JS }
        
        // Layout principal
        div { class: "app-layout",
            // Header
            Header {}
            
            // Hero Section (vacío 1/3 + logo 2/3)
            Hero {}
            
            // Cards Section
            div { class: "cards-section",
                GithubCard {}
            }
            
            // Footer (placeholder)
            footer { class: "footer",
                p { "© 2025 MitDevcat" }
            }
        }
    }
}

#[component]
pub fn GithubCard() -> Element {
    rsx! {
        div { class: "github-card",
            a {
                href: "https://github.com/mitdevcat",
                target: "_blank",
                rel: "noopener noreferrer",
                img { 
                    src: GITHUB_SVG,
                    class: "github-icon",
                    alt: "GitHub"
                }
                "GitHub"
            }
        }
    }
}
