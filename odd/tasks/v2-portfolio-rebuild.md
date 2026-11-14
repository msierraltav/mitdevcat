# Reconstrucción del portafolio Mitdevcat v2

Portafolio reescrito desde cero en Rust + Dioxus 0.6, con nuevo diseño.
El usuario escribe el código a mano; el asistente actúa como mentor/arquitecto.
Rama de trabajo: `v2` (el sitio actual queda intacto en `main`).

## Specs

- **S1**: "quiero volver a hacerla, conservando las imágenes que existen, pero hacerla de 0, en su mayoría a mano ocupándote como medium o mentor" — reescritura desde cero, el usuario escribe, el asistente guía.
- **S2**: Stack elegido: **Rust + Dioxus 0.6** (otra vez, pero desde cero).
- **S3**: Funcionalidades a mantener: **multi-idioma (es/en/ca)**, **proyectos desde GitHub API**, **modo claro/oscuro**.
- **S4**: **Nuevo diseño** desde cero (no conservar el look & feel actual).
- **S5**: "dejame solo el main y el primer componente; igual quiero que los componentes estén cada uno en su archivo para tener los archivos ordenados, como si fuera un proyecto de componentes de React" — estructura: `main.rs` mínimo + `src/components/` con un archivo por componente + `mod.rs` como índice.
- **S6**: Imágenes a conservar (en `assets/`, sin tocar): `Mit-logo-light.png`, `Mit-logo-dark.png`, `header.svg`, `github.svg`, `favicon.ico`.
- **S7**: Estructura con stubs: cada milestone tiene sus archivos ya creados con guía/comentarios; el código real lo escribe el usuario al llegar a ese hito.

## Tasks

| ID | Milestone | Specs | Archivos | Estado |
|----|-----------|-------|----------|--------|
| M1 | Esqueleto: main.rs mínimo + Hero | S1, S5 | `src/main.rs`, `src/components/mod.rs`, `src/components/hero.rs` | en progreso (falta verificar `dx serve` en la máquina del usuario) |
| M2 | Diseño base: layout, tipografía, paleta con CSS variables | S4 | `assets/styles/v2/variables.css`, `assets/styles/v2/layout.css` | pendiente |
| M3 | Header nuevo | S3, S4, S6 | `src/components/header.rs` | pendiente |
| M4 | Secciones estáticas: About, Projects, Contact | S4 | `src/components/about.rs`, `src/components/projects.rs`, `src/components/contact.rs` | pendiente |
| M5 | Modo claro/oscuro nativo (signals, sin JS externo) | S3 | `src/components/theme_toggle.rs` + variables.css | pendiente |
| M6 | Multi-idioma con dioxus-i18n (es/en/ca) | S3 | `src/components/language_selector.rs`, `i18n/*.ftl` | pendiente |
| M7 | Proyectos desde GitHub API (use_resource + reqwest) | S3 | `src/github_api.rs`, `src/components/projects.rs` | pendiente |
| M8 | Deploy a GitHub Pages (dx bundle) | — | `DEPLOY_GITHUB_PAGES.md` (ya existe) | pendiente |

## Log

- **L1**: "adelante, quiero vovler a hacerla , conservando las imanges que existen, pero hacerla de 0 , en u mayoria a mano ocupandote como medium o mentor"
- **L2**: Decisiones del usuario: Rust + Dioxus otra vez; mantener multi-idioma, GitHub API y modo claro/oscuro; diseño nuevo.
- **L3**: "antes de seguir, limpiame el codigo de la pagina, y vamos a ir de a poco, no mas contenido poara el main creame dejalo y dejame solo el main y el primero componente, igual quiero que os componentes esten cada uno en su archivo para tner los archivos ordenados, como si fuera un proyecto de componentes de react" — hecho: borrados card/header/sections/language_selector/github_api; creada rama `v2`.
- **L4**: "guarda este plan en una carpeta, crea todo los archivos con los pasos propuestos" — opción elegida: **plan + estructura con stubs**; creados este documento y los archivos stub por milestone.
- **L5**: Nota de entorno: en esta máquina no hay toolchain de Rust (`cargo`/`rustc`/`rustup` ausentes); la verificación de compilación la hace el usuario con `dx serve`.
