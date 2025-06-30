use dioxus::prelude::*;
use crate::{ThemeContext, MIT_LOGO_LIGHT, MIT_LOGO_DARK};

#[component]
pub fn Hero() -> Element {
    let theme_context = use_context::<Signal<ThemeContext>>();
    
    // Seleccionar el logo según el tema
    let logo_src = if theme_context().is_dark {
        MIT_LOGO_LIGHT // Logo claro para fondo oscuro
    } else {
        MIT_LOGO_DARK // Logo oscuro para fondo claro
    };
    
    rsx! {
        div { class: "hero-section",
            // Sección vacía (1/3 izquierda)
            div { class: "hero-left" }
            
            // Logo (2/3 derecha)  
            div { class: "hero-right",
                img {
                    src: logo_src,
                    alt: "MIT Logo",
                    class: "hero-image"
                }
            }
        }
    }
}
