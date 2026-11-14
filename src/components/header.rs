// ============================================================================
// M3: Header nuevo
// ============================================================================
// Objetivo: barra superior fija con:
//   - Logo (usar Mit-logo-light.png / Mit-logo-dark.png, ya en assets/)
//   - Navegación (anclas a About, Projects, Contact)
//   - Toggle de tema claro/oscuro (llega en M5)
//   - Selector de idioma (llega en M6)
//
// Conceptos nuevos en este milestone:
//   - asset!() para importar imágenes
//   - Props de componente (parámetros de la fn)
//   - Anclas HTML: a { href: "#about" }
//
// Cómo activarlo cuando trabajes este milestone:
//   1. Descomentá el bloque de abajo.
//   2. En src/components/mod.rs descomentá: `pub mod header;`
//   3. En src/main.rs: `use components::header::Header;` y agregá `<Header />`
//      dentro de rsx! antes de `<Hero />`.

// use dioxus::prelude::*;
//
// #[component]
// pub fn Header() -> Element {
//     rsx! {
//         header { class: "header",
//             // TODO M3: logo + nav + (M5) toggle tema + (M6) idiomas
//         }
//     }
// }
