# Clase 03 — M3: Header nuevo

**Archivo destino: `src/components/header.rs`** (stub listo con instrucciones).

## Objetivo

Barra superior con logo + navegación por anclas. Acá el sitio deja de ser
"Hola mundo" y empieza a parecer una página.

## Conceptos nuevos

### 1. Imágenes en Dioxus: `asset!()`

```rust
const MIT_LOGO: Asset = asset!("/assets/Mit-logo-dark.png");

rsx! {
    img { src: MIT_LOGO, alt: "MitDevcat" }
}
```

`asset!()` resuelve la ruta en build time y optimiza el asset. Las constantes
`Asset` van arriba del archivo, fuera de la función.

### 2. Composición: App arma, componentes muestran

Después de M3, `App` se lee como un índice de la página:

```rust
rsx! {
    Header {}
    Hero {}
}
```

### 3. Anclas: navegación sin rutas

Para un portafolio de una página no hace falta routing: `<a href="#about">`
y cada sección tiene `id: "about"`. Simple y honesto.

## Paso a paso

1. Descomentá el componente en `src/components/header.rs` y escribilo:

```rust
#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "header",
            div { class: "container header-inner",
                img { src: MIT_LOGO, class: "header-logo", alt: "MitDevcat" }
                nav { class: "header-nav",
                    a { href: "#about", "About" }
                    a { href: "#projects", "Projects" }
                    a { href: "#contact", "Contact" }
                }
            }
        }
    }
}
```

2. En `mod.rs` descomentá `pub mod header;`
3. En `main.rs`: `use components::header::Header;` y agregá `<Header {} />` arriba.
4. CSS nuevo: `assets/styles/v2/header.css` — flexbox con `justify-content: space-between`.

```css
.header { position: sticky; top: 0; background: var(--color-surface); }
.header-inner { display: flex; justify-content: space-between; align-items: center; }
.header-nav { display: flex; gap: var(--space-4); }
```

5. Linkéalo desde `main.rs` con `asset!()` + `document::Link`.

## Detalles a decidir vos

- ¿Sticky o estático el header?
- ¿Logo a la izquierda y nav a la derecha, o todo centrado?
- ¿Los links en mayúsculas chicas con `letter-spacing` o normales?

## Ejercicios

1. Hacé que el link activo (hover) use `--color-accent`.
2. Agregá el link de tu GitHub con el ícono `github.svg`.

## Checklist M3

- [ ] `header.rs` compila y está declarado en `mod.rs`
- [ ] Logo visible con su `alt`
- [ ] Los 3 anclas scrollean a sus secciones
- [ ] `header.css` usa solo variables de `variables.css`
