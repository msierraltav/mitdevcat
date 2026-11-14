# Curso Mitdevcat v2 — Reconstrucción del portafolio en Dioxus

Bienvenida al curso. Vas a reescribir tu portafolio desde cero en **Rust + Dioxus 0.6**,
escribiendo todo a mano. Este curso es tu guía: una clase por milestone.

## Reglas del curso

1. **Vos escribís el código.** Los ejemplos de las clases son referencias, no para copiar
   y pegar completo: escribilos a mano, nombrando cada parte en voz alta. Funciona.
2. **Un milestone a la vez.** No avances sin completar el checklist de la clase actual.
3. **Todo en su archivo.** Cada componente vive en `src/components/<nombre>.rs`, y se
   registra en `src/components/mod.rs` (el "índice", como en un proyecto React).
4. **Trabajás en la rama `v2`**; el sitio viejo queda intacto en `main`.

## Clases

| Clase | Milestone | Qué construye |
|-------|-----------|---------------|
| [01](./01-m1-esqueleto.md) | M1 | Esqueleto: `main.rs` mínimo + primer componente |
| [02](./02-m2-diseno-base.md) | M2 | Diseño base: paleta, tipografía, layout (CSS variables) |
| [03](./03-m3-header.md) | M3 | Header nuevo con logo y navegación |
| [04](./04-m4-secciones.md) | M4 | Secciones About, Projects, Contact |
| [05](./05-m5-tema.md) | M5 | Modo claro/oscuro nativo (signals, sin JS externo) |
| [06](./06-m6-i18n.md) | M6 | Multi-idioma (es/en/ca) con dioxus-i18n |
| [07](./07-m7-github-api.md) | M7 | Proyectos reales desde la API de GitHub |
| [08](./08-m8-deploy.md) | M8 | Deploy a GitHub Pages |

## Estado actual

- ✅ Proyecto limpio: `main.rs` mínimo + `components/hero.rs` (M1 hecho, falta tu `dx serve` de verificación)
- ✅ Stubs creados en `src/components/` (comentados, se activan al llegar a cada milestone)
- ✅ Plan completo en `odd/tasks/v2-portfolio-rebuild.md`
- ⚠️ Esta máquina no tiene Rust instalado; la verificación de compilación es tuya con `dx serve`

## Recursos de referencia

- [Dioxus 0.6 docs](https://dioxuslabs.com/learn/0.6/) — la biblio oficial, bilingüe en ejemplos
- `i18n/es.ftl`, `i18n/en.ftl`, `i18n/ca.ftl` — tus traducciones viejas (se reusan en M6)
- `DEPLOY_GITHUB_PAGES.md` — tus notas de deploy (se reusan en M8)
- Imágenes conservadas en `assets/`: logos claro/oscuro, `header.svg`, `github.svg`, favicon
