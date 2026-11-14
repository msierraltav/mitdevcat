# Clase 08 — M8: Deploy a GitHub Pages

**Referencia: tu propio `DEPLOY_GITHUB_PAGES.md` en la raíz (ya lo hiciste una vez).**

## Objetivo

Publicar la v2 en tu GitHub Pages actual, reemplazando al sitio viejo.

## Concepto: `dx bundle`

```bash
dx bundle --out-dir docs
```

Compila el WASM en modo release, optimiza assets y deja todo estático en `docs/`.
GitHub Pages sirve esa carpeta tal cual.

## Paso a paso (según tus notas, actualizadas)

1. ```bash
   dx bundle --out-dir docs
   ```

2. El build pone el contenido dentro de `docs/public/`; movelo a la raíz de `docs/`:

   ```bash
   mv docs/public/* docs/
   ```

3. SPA de una página → copiá el index como 404 (para que Pages sirva algo siempre):

   ```bash
   cp docs/index.html docs/404.html
   ```

4. Verificá que `docs/CNAME` sigue ahí (ya lo tenías de antes) y que apunta a tu dominio.
5. Commit + push a la rama desde la que Pages sirve (la que tengas configurada en
   Settings → Pages de tu repo).
6. Esperá ~1 minuto, refrescá con cache off (Ctrl+Shift+R) y celebrá. 🎉

## Cosas que cambian respecto al sitio viejo

- **No hay `theme-toggle.js` ni CSS suelto en `public/`**: Dioxus empaqueta todo
  con hashes (`/assets/main-abc123.css`). No versiones a mano nada.
- Si el header.svg viejo ya no lo usás, podés borrarlo de `assets/` antes del bundle.

## Pre-deploy checklist

- [ ] `dx serve` en modo release no muestra warnings nuevos graves
- [ ] Probaste claro/oscuro, los 3 idiomas y la carga de repos
- [ ] Probaste en el celular (o DevTools responsive): el portafolio se mira en móvil
- [ ] Los links de contacto funcionan

## Post-deploy

- [ ] El sitio carga desde una pestaña de incógnito
- [ ] La API de GitHub responde desde el dominio publicado
- [ ] Actualizá `main` con la v2 cuando estés conforme: `git checkout main && git merge v2`

## Después del curso

Ideas para seguir evolucionando (opcional, sin presión):

- Meta tags Open Graph para que los links compartidos se vean bien
- Lighthouse audit: apuntá a 90+ en Performance y Accessibility
- Un blog en Rust puro con rutas de Dioxus (`dioxus-router`)
