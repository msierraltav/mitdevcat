// ============================================================================
// M4: Sección About
// ============================================================================
// Objetivo: sección "sobre mí" con texto y quizás una foto o ilustración.
//
// Conceptos nuevos en este milestone:
//   - Estructura de una sección reutilizable: <section id="about">
//   - Espaciado y jerarquía tipográfica con las CSS variables de M2
//
// Cómo activarlo:
//   1. Descomentá el bloque de abajo.
//   2. En src/components/mod.rs descomentá: `pub mod about;`
//   3. En src/main.rs: `use components::about::About;` y agregá `<About />`
//      después de `<Hero />`.

// use dioxus::prelude::*;
//
// #[component]
// pub fn About() -> Element {
//     rsx! {
//         section { id: "about",
//             class: "section about",
//             // TODO M4: tu texto de presentación
//         }
//     }
// }
