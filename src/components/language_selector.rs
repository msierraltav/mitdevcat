use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, t, unic_langid::langid};

#[component]
pub fn LanguageSelector() -> Element {
    let mut i18n = i18n();
    
    let change_to_english = move |_| {
        i18n.set_language(langid!("en-US"));
    };
    
    let change_to_spanish = move |_| {
        i18n.set_language(langid!("es-ES"));
    };
    
    let change_to_catalan = move |_| {
        i18n.set_language(langid!("ca-ES"));
    };
    
    rsx! {
        div { class: "language-selector",
            button { 
                class: "lang-btn",
                onclick: change_to_english,
                title: t!("language-en"),
                "EN"
            }
            button { 
                class: "lang-btn",
                onclick: change_to_spanish,
                title: t!("language-es"),
                "ES"
            }
            button { 
                class: "lang-btn",
                onclick: change_to_catalan,
                title: t!("language-ca"),
                "CA"
            }
        }
    }
}
