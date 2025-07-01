use dioxus::prelude::*;
use dioxus_i18n::t;
use crate::ThemeContext;
use crate::components::language_selector::LanguageSelector;

#[component]
pub fn Header() -> Element {
    let mut theme_context = use_context::<Signal<ThemeContext>>();
    
    let toggle_theme = move |_| {
        let current_theme = theme_context();
        theme_context.set(ThemeContext { is_dark: !current_theme.is_dark });
        
        // Llamar a la función JavaScript global
        let _ = web_sys::js_sys::eval("window.toggleTheme()");
    };
    
    rsx! {
        header { class: "main-header",
            // Logo/Brand
            div { class: "header-brand",
                span { {t!("header-home")} }
            }
            
            // Navigation and controls
            div { class: "header-controls",
                // Language selector
                LanguageSelector {}
                
                // Theme toggle button
                button { 
                    class: "theme-toggle",
                    onclick: toggle_theme,
                    title: if theme_context().is_dark { 
                        "Light theme" 
                    } else { 
                        "Dark theme" 
                    },
                    span { class: "theme-icon",
                        if theme_context().is_dark {
                            "☀️"
                        } else {
                            "🌙"
                        }
                    }
                }
            }
        }
    }
}
