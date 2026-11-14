// ============================================================================
// M7: API de GitHub
// ============================================================================
// Objetivo: traer tus repos públicos y calcular stats para la sección
// Projects. El sitio viejo tenía este mismo archivo (200 líneas); vamos a
// escribirlo de nuevo, más simple y a tu manera.
//
// Endpoint: https://api.github.com/users/msierraltav/repos
//
// Conceptos nuevos:
//   - Structs de Rust con serde (Deserialize) para mapear el JSON
//   - use_resource + async move en el componente
//   - Manejo de los 3 estados: cargando / error / datos
//
// Nota práctica: la API de GitHub pide header User-Agent en requests
// desde WASM y tiene límite de 60 req/hora sin token. Lo vemos en M7.
//
// Cómo activarlo:
//   1. Escribí el struct GitHubRepo (con serde::Deserialize).
//   2. En src/main.rs descomentá: `mod github_api;`

// use serde::Deserialize;
//
// #[derive(Deserialize, Clone, Debug)]
// pub struct GitHubRepo {
//     pub name: String,
//     pub description: Option<String>,
//     pub html_url: String,
//     pub stargazers_count: u32,
//     pub language: Option<String>,
// }
//
// TODO M7: función async fetch_repos() -> Result<Vec<GitHubRepo>, ...>
