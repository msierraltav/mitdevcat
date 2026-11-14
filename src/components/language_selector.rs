// ============================================================================
// M6: Selector de idioma (es / en / ca)
// ============================================================================
// Objetivo: botón o dropdown que cambia el idioma de TODO el sitio.
//
// La pieza clave que ya existe del sitio viejo: la librería dioxus-i18n
// y los archivos i18n/es.ftl, i18n/en.ftl, i18n/ca.ftl (no los borramos).
//
// Conceptos nuevos:
//   - use_init_i18n + I18nConfig en App (inicialización)
//   - La macro t!("texto") en cada componente para traducir
//   - i18n.set_language(langid!("es-ES")) para cambiar de idioma
//
// Cómo activarlo:
//   1. Descomentá el bloque de abajo.
//   2. En src/components/mod.rs descomentá: `pub mod language_selector;`
//   3. Montalo dentro del Header (M3).

// use dioxus::prelude::*;
// use dioxus_i18n::prelude::*;
// use unic_langid::langid;
//
// #[component]
// pub fn LanguageSelector() -> Element {
//     rsx! {
//         div { class: "language-selector",
//             // TODO M6: un botón por idioma (ES / EN / CA)
//         }
//     }
// }
