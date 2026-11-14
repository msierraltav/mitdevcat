// ============================================================================
// M4: Sección Contact
// ============================================================================
// Objetivo: sección de contacto con links (GitHub, email, lo que quieras).
//
// Conceptos nuevos:
//   - Links externos con target: "_blank", rel: "noopener noreferrer"
//   - Íconos: podés usar el github.svg que ya está en assets/
//
// Cómo activarlo:
//   1. Descomentá el bloque de abajo.
//   2. En src/components/mod.rs descomentá: `pub mod contact;`
//   3. En src/main.rs: `use components::contact::Contact;` y agregá
//      `<Contact />` como última sección.

// use dioxus::prelude::*;
//
// #[component]
// pub fn Contact() -> Element {
//     rsx! {
//         section { id: "contact",
//             class: "section contact",
//             // TODO M4: links de contacto
//         }
//     }
// }
