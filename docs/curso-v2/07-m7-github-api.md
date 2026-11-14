# Clase 07 — M7: Proyectos desde la API de GitHub

**Archivos destino: `src/github_api.rs` + refactor de `src/components/projects.rs`.**

## Objetivo

Reemplazar las tarjetas fijas de M4 por tus repos reales:
`https://api.github.com/users/msierraltav/repos`

## Conceptos nuevos

### 1. Deserializar JSON con serde (Rust fuerte, JSON flexible)

```rust
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubRepo {
    pub name: String,
    pub description: Option<String>,   // puede faltar en el JSON
    pub html_url: String,
    pub stargazers_count: u32,
    pub language: Option<String>,
}
```

`Option<String>` es la respuesta de Rust a "este campo a veces no está".
Solo pedís los campos que usás: serde ignora el resto del JSON.

### 2. La función async

```rust
pub async fn fetch_repos() -> Result<Vec<GitHubRepo>, reqwest::Error> {
    let client = reqwest::Client::new();
    let repos = client
        .get("https://api.github.com/users/msierraltav/repos")
        .header("User-Agent", "mitdevcat-portfolio")
        .send()
        .await?
        .json::<Vec<GitHubRepo>>()
        .await?;
    Ok(repos)
}
```

- `.await?` = "esperá la respuesta y si falla, propagá el error".
- El `User-Agent` es **obligatorio** para la API de GitHub.

### 3. `use_resource` — async en el componente

```rust
let repos_resource = use_resource(|| async { fetch_repos().await });

rsx! {
    match &*repos_resource.read() {
        Some(Ok(repos)) => rsx! {
            for repo in repos { ProjectCard { /* ... */ } }
        },
        Some(Err(_)) => rsx! { p { "No pude cargar los repos 😿" } },
        None => rsx! { p { "Cargando..." } },
    }
}
```

Los **tres estados** siempre existen: cargando (`None`), error (`Some(Err)`),
datos (`Some(Ok)`). Un buen diseño los muestra todos.

## Advertencias prácticas (te ahorran horas)

1. **Rate limit**: sin token son 60 requests/hora por IP. Para un portafolio
   personal alcanza; si te quedás corto, filtrá y ordená del lado del cliente:
   `repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count))`.
2. **CORS**: GitHub API permite CORS desde cualquier origen, así que WASM
   funciona directo. Si algún día migrás a otra API, esto puede romperse.
3. **No mezcles** el fetch con el render: `github_api.rs` sabe de red,
   `projects.rs` sabe de UI. Separación limpia.

## Paso a paso

1. Escribí el struct y `fetch_repos` en `github_api.rs`.
2. En `main.rs`: `mod github_api;` (el stub ya te lo recuerda).
3. Refactorizá `projects.rs`: `use_resource` + el `match` de 3 estados.
4. Reusá tu `ProjectCard` de M4 como está: la interfaz no cambió, solo el origen
   de los datos. **Ese es el premio de haber hecho bien M4.**

## Ejercicios

1. Mostrá ⭐ y el lenguaje de cada repo en la tarjeta.
2. Filtrá los forks: `repos.iter().filter(|r| !r.fork)` — (agregá `pub fork: bool` al struct).
3. Mostrá cuántos repos hay: "N proyectos" en el título de la sección.

## Checklist M7

- [ ] La sección muestra tus repos reales
- [ ] Los tres estados (cargando/error/datos) están cubiertos
- [ ] `github_api.rs` no contiene HTML y `projects.rs` no contiene URLs
- [ ] Con la red desconectada, el sitio muestra el error con gracia (no se rompe)
