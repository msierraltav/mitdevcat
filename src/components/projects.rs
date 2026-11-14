// ============================================================================
// M4 (contenido estático) -> M7 (datos reales de GitHub API)
// ============================================================================
// Objetivo M4: sección de proyectos con tarjetas escritas a mano
//   (2 o 3 proyectos fijos, solo para maquetar el diseño).
//
// Objetivo M7: reemplazar las tarjetas fijas por tus repos reales,
//   traídos con use_resource + reqwest desde
//   https://api.github.com/users/msierraltav/repos
//
// Conceptos nuevos:
//   - M4: renderizar listas con iter (como map() en React)
//   - M7: use_resource, Option<Result<...>>, estados de carga y error
//
// Cómo activarlo:
//   1. Descomentá el bloque de abajo.
//   2. En src/components/mod.rs descomentá: `pub mod projects;`
//   3. En src/main.rs: `use components::projects::Projects;` y agregá
//      `<Projects />` a la lista de secciones.

// use dioxus::prelude::*;
//
// #[component]
// pub fn Projects() -> Element {
//     rsx! {
//         section { id: "projects",
//             class: "section projects",
//             h2 { "Proyectos" }
//             // TODO M4: grid de tarjetas fijas
//             // TODO M7: conectar con github_api.rs
//         }
//     }
// }
