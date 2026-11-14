# Clase 05 — M5: Modo claro/oscuro nativo

**Archivo destino: `src/components/theme_toggle.rs`.**
Adiós `theme-toggle.js`: el tema se maneja con estado de Dioxus y CSS puro.

## Objetivo

Un botón en el Header que alterna el tema, **sin tocar ningún componente más**:
la magia vive en las variables CSS que disciplinamos en M2.

## La arquitectura (3 piezas)

```
Signal (Rust)  ──use_effect──>  atributo data-theme en <body>
                                        │
CSS: [data-theme="dark"] { --color-bg: #111; ... }
                                        │
Todos los componentes siguen usando var(--color-bg) y no se enteran.
```

### 1. `use_signal` — el estado (como `useState`)

```rust
// en App:
let mut is_dark = use_signal(|| true);
```

### 2. `use_effect` — el efecto secundario (como `useEffect`)

```rust
use_effect(move || {
    let theme = if is_dark() { "dark" } else { "light" };
    document::eval("document.body.setAttribute('data-theme', '{theme}')");
});
```

Corre cada vez que `is_dark` cambia. Una línea de JS mínimo para tocar el DOM.

### 3. El botón que muta la señal

```rust
#[component]
pub fn ThemeToggle(mut is_dark: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "theme-toggle",
            onclick: move |_| is_dark.toggle(),
            // ícono según tema:
            if is_dark() { "🌙" } else { "☀️" }
        }
    }
}
```

Fijate el `mut` en la prop `Signal`: para mutar una señal se necesita.

### 4. El CSS (solo esto, en `variables.css`)

```css
:root { /* tus variables claras de M2 */ }

[data-theme="dark"] {
  --color-bg: #121212;
  --color-surface: #1e1e1e;
  --color-text: #eaeaea;
  /* --color-accent si necesita versión más clara */
}
```

Nada más. Ningún otro CSS cambia.

## Paso a paso

1. En `App`: la señal + el `use_effect`.
2. Escribí `theme_toggle.rs` y montalo dentro del `Header` (pasale la señal como prop:

   ```rust
   ThemeToggle { is_dark }
   ```

   Pasar la señal es pasar "una conexión viva" al estado, no una copia).
3. Agregá el bloque `[data-theme="dark"]` a `variables.css`.

## Ejercicios

1. Hacé que el botón rote el ícono con una transición CSS.
2. Persistí el tema: guardalo en `localStorage` dentro del `use_effect` y
   leelo al iniciar (`document::eval` con `localStorage.getItem`). Pista:
   el valor inicial de la señal puede ser `true` y el effect corregirlo.
3. Preguntale al sistema: `window.matchMedia('(prefers-color-scheme: dark)')`.

## Checklist M5

- [ ] El botón alterna el tema y todos los colores cambian juntos
- [ ] Cero referencias a colores en componentes o CSS fuera de `variables.css`
- [ ] No queda ningún `theme-toggle.js` en el proyecto
- [ ] (Bonus) El tema sobrevive un refresh
