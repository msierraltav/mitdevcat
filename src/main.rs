mod components;
use components::header::Header;
use components::hero::Hero;
use components::sections::{AboutSection, ProjectsSection, ContactSection};
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t, unic_langid::langid};

// Contexto para el tema
#[derive(Clone, Copy, PartialEq)]
pub struct ThemeContext {
    pub is_dark: bool,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_CSS: Asset = asset!("/assets/styles/header.css");
const HERO_CSS: Asset = asset!("/assets/styles/hero.css");
const ANIMATION_CSS: Asset = asset!("/assets/styles/animations.css");
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
    
    // Initialize i18n with English as default
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("../i18n/en.ftl")))
            .with_locale((langid!("es-ES"), include_str!("../i18n/es.ftl")))  
            .with_locale((langid!("ca-ES"), include_str!("../i18n/ca.ftl")))
            .with_fallback(langid!("en-US"))
    });
    
    use_context_provider(|| theme_context);
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: HEADER_CSS }
        document::Link { rel: "stylesheet", href: HERO_CSS }
        document::Link { rel: "stylesheet", href: ANIMATION_CSS }
        document::Script { src: THEME_JS }
        
        // Layout principal
        div { class: "app-layout",
            // Header
            Header {}
            
            // Hero Section (vacío 1/3 + logo 2/3)
            Hero {}
            
            // About Section
            AboutSection {}
            
            // Projects Section
            ProjectsSection {}
            
            // Contact Section
            ContactSection {}
            
            // Cards Section
            div { class: "cards-section",
                GithubCard {}
            }
            
            // Footer (placeholder)
            footer { class: "footer",
                p { {t!("footer-copyright")} }
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
                {t!("nav-github")}
            }
        }
    }
}
