# Clase 01 — M1: El esqueleto

**Estado: hecho (falta tu verificación con `dx serve`).**
Esta clase queda registrada para que entiendas *qué* se hizo y *por qué*.

## Objetivo

Un proyecto Dioxus mínimo donde el `main.rs` no sabe nada del contenido:
solo monta un componente raíz `App` que renderiza otros componentes.

## La estructura (estilo proyecto React)

```
src/
├─ main.rs          ← punto de entrada: launch(App) + App que arma la página
├─ components/
│  ├─ mod.rs        ← índice de componentes (como index.js)
│  └─ hero.rs       ← primer componente
```

## Los 3 conceptos de esta clase

### 1. `launch(App)` — el arrancador

```rust
fn main() {
    dioxus::launch(App);
}
```

Esto monta tu componente `App` en el DOM del navegador (WASM).

### 2. Componentes = funciones que devuelven `Element`

```rust
#[component]
fn App() -> Element {
    rsx! { Hero {} }
}
```

- `#[component]` genera el "pegamento" para usar la función como etiqueta.
- `rsx!` es el HTML: etiquetas, atributos con `:`, texto directo, hijos entre llaves.

### 3. Un archivo por componente, con `mod.rs` como índice

```rust
// mod.rs
pub mod hero;   // "existe un archivo hero.rs, es público"
```

En `main.rs` lo usás con la ruta completa del módulo:
`use components::hero::Hero;`

## Lo que quedó escrito

```rust
// src/components/hero.rs
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            h1 { "MitDevcat v2" }
            p { "Sitio en construcción, escrito a mano" }
        }
    }
}
```

## Verificación (tuya, en tu máquina)

```bash
git checkout v2        # ya estás
dx serve               # debería abrir localhost con el título
```

## Checklist M1

- [ ] `dx serve` compila sin errores
- [ ] Veo el título "MitDevcat v2" en el navegador
- [ ] Cambio el texto del `p`, guardo, y el hot-reload lo actualiza solo
- [ ] Entiendo para qué sirve `mod.rs`
