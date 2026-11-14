# Clase 02 — M2: Diseño base (paleta, tipografía, layout)

**Antes de escribir un componente más, decidimos la identidad visual.**
El CSS del sitio nuevo vive en `assets/styles/v2/` (ya tiene 2 stubs con guías).

## Objetivo

Que todo el sitio comparta una sola fuente de verdad visual: **variables CSS**.

## Concepto clave: CSS variables

```css
:root {
  --color-accent: #e05d3d;
  --font-heading: "Space Grotesk", sans-serif;
  --space-4: 1rem;
}

h1 { color: var(--color-accent); font-family: var(--font-heading); }
```

¿Por qué importa tanto? En M5 el modo oscuro será **solo** redefinir estas
variables bajo `[data-theme="dark"]`. Cero cambios en componentes. Esa es la
recompensa de la disciplina de hoy.

## Paso a paso

### 1. Elegí la tipografía

Portafolio = una familia para **títulos** y otra (o la misma, más liviana) para
**texto**. Sugerencias con carácter: Space Grotesk, Sora, Outfit, Fraunces.
Cargala con un `<link>` de Google Fonts (en M3 lo agregamos al `Header` o al `App`).

### 2. Elegí la paleta (3 roles + acento)

| Rol | Para qué | Ejemplo claro |
|-----|----------|---------------|
| `--color-bg` | fondo de página | `#faf8f5` |
| `--color-surface` | tarjetas, header | `#ffffff` |
| `--color-text` | texto principal | `#1a1a1a` |
| `--color-accent` | links, botones, destacados | el color que te represente |

Herramienta: [coolors.co](https://coolors.co). Regla: **máximo 1 acento**.

### 3. Definí la escala de espaciado

Base 4px, doblá: `0.25 / 0.5 / 1 / 1.5 / 2 / 3 / 4rem`. Nunca valores mágicos.

### 4. Escribí `assets/styles/v2/variables.css`

El stub ya te dice qué declarar. Completalo con tus elecciones.

### 5. Escribí `assets/styles/v2/layout.css`

Cuatro reglas, nada más por ahora:

```css
* { margin: 0; box-sizing: border-box; }

body { font-family: var(--font-body); color: var(--color-text);
       background: var(--color-bg); line-height: 1.6; }

.container { max-width: 800px; margin: 0 auto; padding: 0 var(--space-4); }

.section { padding: var(--space-8) 0; }
```

### 6. Conectá el CSS en Dioxus

En `src/main.rs`, declará los assets y linkéalos:

```rust
const VARIABLES_CSS: Asset = asset!("/assets/styles/v2/variables.css");
const LAYOUT_CSS: Asset = asset!("/assets/styles/v2/layout.css");

// dentro de App:
rsx! {
    document::Link { rel: "stylesheet", href: VARIABLES_CSS }
    document::Link { rel: "stylesheet", href: LAYOUT_CSS }
    // ...
}
```

## Ejercicios

1. Poné el `h1` del Hero con `font-family: var(--font-heading)` y el acento.
2. Envolvé el contenido del `Hero` en `div { class: "container" }`.
3. Cambiá `--color-accent` y verificá que el cambio se propaga a todo.

## Checklist M2

- [ ] Elegí y cargué la tipografía
- [ ] `variables.css` completo: 4 colores + 2 fuentes + escala de espaciado
- [ ] `layout.css` con las 4 reglas
- [ ] El Hero se ve con la nueva tipografía y el contenedor centrado
- [ ] Ningún color "pelado" fuera de `variables.css`
