# Clase 04 — M4: Secciones (About, Projects, Contact)

**Archivos destino: `src/components/about.rs`, `projects.rs`, `contact.rs`.**

## Objetivo

Las tres secciones de contenido del portafolio, con datos escritos a mano.
El objetivo de esta clase es el *maquetado*, no el contenido final (M7 trae repos reales).

## Conceptos nuevos

### 1. Listas en rsx: `iter` (el `map()` de React)

```rust
let habilidades = ["Rust", "TypeScript", "Dioxus"];

rsx! {
    ul {
        for habilidad in habilidades {
            li { "{habilidad}" }
        }
    }
}
```

Fijate: `for x in colección` dentro de rsx, y `{variable}` para interpolar texto.

### 2. Grid de tarjetas (CSS)

```css
.projects-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: var(--space-4); }
.card { background: var(--color-surface); padding: var(--space-4); border-radius: 8px; }
```

`auto-fit + minmax` = responsive sin una sola media query.

### 3. Links externos seguros

```rust
a { href: repo_url, target: "_blank", rel: "noopener noreferrer", "Ver en GitHub" }
```

## Paso a paso

1. **About** — `about.rs`: una sección con `id: "about"`, tu presentación en 2-3
   párrafos y una lista de habilidades con `for`. Activá el módulo en `mod.rs`.
2. **Projects** — `projects.rs`: sección con `id: "projects"` y un grid de 2-3
   tarjetas **fijas** (nombre, descripción, link). Esta es tu maqueta para M7.
3. **Contact** — `contact.rs`: sección con `id: "contact"`, links grandes a
   GitHub/email con el ícono `github.svg`.
4. En `main.rs` componé todo en orden:

```rust
rsx! {
    Header {}
    Hero {}
    About {}
    Projects {}
    Contact {}
}
```

5. Si algo se repite (la tarjeta de proyecto aparece 3 veces), ese es el momento
   natural de aprender **props**:

```rust
#[component]
fn ProjectCard(name: String, description: String, url: String) -> Element {
    rsx! {
        div { class: "card",
            h3 { "{name}" }
            p { "{description}" }
            a { href: url, target: "_blank", "Ver más" }
        }
    }
}

// y en Projects:
ProjectCard { name: "Mitdevcat".to_string(), description: "Este sitio".to_string(), url: "https://github.com/...".to_string() }
```

Las props son parámetros de la función componente: eso es todo.

## Ejercicios

1. Refactorizá el grid para usar `ProjectCard` con 3 props.
2. Dale a cada sección un `h2` con estilo común (¿`.section-title` en layout.css?).
3. Animación de entrada suave con `scroll-behavior: smooth` en `html`.

## Checklist M4

- [ ] Las 3 secciones compilan, están en `mod.rs` y en `App`
- [ ] Las anclas del Header (M3) scrollean a cada sección
- [ ] El grid de proyectos es responsive sin media queries
- [ ] Hay al menos un componente con props (`ProjectCard`)
