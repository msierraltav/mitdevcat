use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn AboutSection() -> Element {
    rsx! {
        section { class: "about-section",
            div { class: "about-container",
                h2 { class: "about-title", {t!("about-title")} }
                p { class: "about-description", {t!("about-description")} }
                button { 
                    class: "btn-primary",
                    {t!("btn-learn-more")}
                }
            }
        }
    }
}

#[component]
pub fn ProjectsSection() -> Element {
    rsx! {
        section { class: "projects-section",
            div { class: "projects-container",
                h2 { class: "projects-title", {t!("projects-title")} }
                p { class: "projects-description", {t!("projects-description")} }
                div { class: "projects-grid",
                    div { class: "project-card",
                        h3 { "Sample Project 1" }
                        p { "A description of the project" }
                        button { 
                            class: "btn-secondary",
                            {t!("btn-view-project")}
                        }
                    }
                    div { class: "project-card",
                        h3 { "Sample Project 2" }
                        p { "Another project description" }
                        button { 
                            class: "btn-secondary",
                            {t!("btn-view-project")}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ContactSection() -> Element {
    rsx! {
        section { class: "contact-section",
            div { class: "contact-container",
                h2 { class: "contact-title", {t!("contact-title")} }
                p { class: "contact-description", {t!("contact-description")} }
                div { class: "contact-buttons",
                    button { 
                        class: "btn-primary",
                        {t!("contact-email")}
                    }
                    button { 
                        class: "btn-secondary",
                        {t!("contact-github")}
                    }
                }
            }
        }
    }
}
