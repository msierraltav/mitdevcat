mod components;
mod github_api;

use components::header::Header;
use components::hero::Hero;
use components::sections::{AboutSection, ProjectsSection, ContactSection};
use github_api::{calculate_github_stats, GitHubStats};
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
            // AboutSection {}
            
            // Projects Section
            // ProjectsSection {}
            
            // Contact Section
            // ContactSection {}
            
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
    let mut github_stats = use_signal(|| GitHubStats::default());
    let mut loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    
    // Load GitHub stats when component mounts
    use_effect(move || {
        spawn(async move {
            loading.set(true);
            match calculate_github_stats("msierraltav").await {
                Ok(stats) => {
                    github_stats.set(stats);
                    loading.set(false);
                    error.set(None);
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    });

    let stats = github_stats.read();
    let is_loading = loading.read();
    let error_msg = error.read();

    rsx! {
        div { class: "github-card",
            div { class: "github-card-header",
                img { 
                    src: GITHUB_SVG,
                    class: "github-icon",
                    alt: "GitHub"
                }
                div { class: "github-card-title",
                    h3 { "GitHub" }
                    p { class: "github-handle", "@msierraltav" }
                }
            }
            div { class: "github-card-stats",
                div { class: "stat",
                    span { 
                        class: if *is_loading { "stat-number loading" } else { "stat-number loaded" },
                        if *is_loading {
                            "⟳"
                        } else if error_msg.is_some() {
                            "0"
                        } else {
                            "{stats.total_repos}"
                        }
                    }
                    span { class: "stat-label", "Repos" }
                }
                div { class: "stat",
                    span { 
                        class: if *is_loading { "stat-number loading" } else { "stat-number loaded" },
                        if *is_loading {
                            "⟳"
                        } else if error_msg.is_some() {
                            "0"
                        } else if stats.total_commits >= 1000 {
                            "{stats.total_commits / 1000}k+"
                        } else {
                            "{stats.total_commits}"
                        }
                    }
                    span { class: "stat-label", "Commits" }
                }
                div { class: "stat",
                    span { 
                        class: if *is_loading { "stat-number loading" } else { "stat-number loaded" },
                        if *is_loading {
                            "⟳"
                        } else if error_msg.is_some() {
                            "0"
                        } else {
                            "{stats.total_stars}"
                        }
                    }
                    span { class: "stat-label", "Stars" }
                }
            }
            div { class: "github-card-description",
                p { 
                    if error_msg.is_some() {
                        "Error loading GitHub stats - using cached data"
                    } else {
                        "Building innovative solutions with modern technologies"
                    }
                }
            }
            a {
                href: "https://github.com/msierraltav",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "github-card-button",
                span { "View Profile" }
                svg { 
                    class: "arrow-icon",
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "16",
                    height: "16",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M7 17L17 7" }
                    path { d: "M7 7h10v10" }
                }
            }
        }
    }
}
