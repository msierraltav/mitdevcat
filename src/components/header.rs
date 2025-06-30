use dioxus::prelude::*;
use crate::ThemeContext;

#[component]
pub fn Header() -> Element {
    let mut theme_context = use_context::<Signal<ThemeContext>>();
    
    let toggle_theme = move |_| {
        let current_theme = theme_context();
        theme_context.set(ThemeContext { is_dark: !current_theme.is_dark });
        
        // Llamar a la función JavaScript global
        unsafe {
            web_sys::js_sys::eval("window.toggleTheme()").ok();
        }
    };
    
    rsx! {
        header { class: "main-header",
            // Logo/Brand
            div { class: "header-brand",
                span { "Inicio" }
            }
            
            // Navigation and controls
            div { class: "header-controls",
                // Theme toggle button
                button { 
                    class: "theme-toggle",
                    onclick: toggle_theme,
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
