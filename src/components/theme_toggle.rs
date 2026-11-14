// ============================================================================
// M5: Modo claro/oscuro nativo (sin theme-toggle.js)
// ============================================================================
// Objetivo: reemplazar el JS externo del sitio viejo por Dioxus puro.
//
// La estrategia que te voy a guiar (más limpia que la del sitio viejo):
//   1. Una señal `is_dark: Signal<bool>` vive en App.
//   2. Un `use_effect` la escribe en el DOM como atributo:
//      document.body().set_attribute("data-theme", "dark")
//   3. El CSS usa SOLO variables: [data-theme="dark"] { --bg: ...; }
//      Así ningún componente sabe nada del tema, solo el CSS.
//
// Conceptos nuevos:
//   - use_signal (estado, como useState de React)
//   - use_effect (efecto secundario, como useEffect)
//   - onClick: move |_| { ... } para el botón
//
// Cómo activarlo:
//   1. Descomentá el bloque de abajo.
//   2. En src/components/mod.rs descomentá: `pub mod theme_toggle;`
//   3. Montalo dentro del Header (M3) o donde prefieras.

// use dioxus::prelude::*;
//
// #[component]
// pub fn ThemeToggle(is_dark: Signal<bool>) -> Element {
//     rsx! {
//         button {
//             class: "theme-toggle",
//             onclick: move |_| is_dark.set(!is_dark()),
//             // TODO M5: ícono sol/luna según el tema
//         }
//     }
// }
