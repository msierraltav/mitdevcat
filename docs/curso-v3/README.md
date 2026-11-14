# Curso v3 — la página es un programa

Un curso para construir `mitdevcat` desde cero, aprendiendo Rust de verdad.
El código lo escribís vos: acá están el concepto, el ejercicio, el test que lo
prueba y el criterio para saber si quedó bien.

`docs/curso-v2/` queda como historia: enseñaba el camino Dioxus + wasm.

## El método (leelo una vez, seguilo siempre)

Cada módulo tiene la misma forma:

1. **Concepto** — por qué existe el problema, antes de la sintaxis.
2. **Ejercicio** — lo que escribís vos. Nunca es copiar: es resolver.
3. **Criterio de aceptación** — los tests que tienen que pasar. Si pasan, el
   módulo está cerrado, aunque el diseño no te guste todavía.
4. **Si te trabás** — la pista mínima. La implementación de referencia está en
   la rama `tui-total`: mirala **después** de intentarlo, nunca antes.

## Reglas de la casa

- **Se desarrolla y se prueba en la terminal.** El navegador es una verificación
  ocasional, no el loop.
- **Un concepto por módulo.** Si algo no se entiende, no se avanza: se pregunta.
- **Nada de `cargo install`.** En una Pi 4 construir herramientas desde la fuente
  son horas. Binarios prebuilt o nada.
- **Se mide, no se opina.** Los números de este curso salieron de esta máquina.

## El punto de partida, medido

Construimos sobre un proyecto real que ya existía y dolía:

| | Antes (Dioxus + wasm) | Objetivo del curso |
|---|---|---|
| Crates | 635 | ≤ 80 |
| `cargo check` | 2m 10s | — |
| Loop real | 3m 29s | ~1s |
| Salida | wasm + JS bundle | HTML plano |

La Pi 4 (4 GB, microSD) no era el problema. El grafo de dependencias sí.

## Dónde trabajás

Creá un proyecto nuevo y paralelo, así no rompés nada:

```sh
cd ~/code
cargo new mitdevcat-curso --lib
```

Al final del curso podés traer los archivos al repo real. La referencia
terminada vive en la rama `tui-total` de `~/code/mitdevcat`.

## Programa

| M | Módulo | Qué aprendés |
|---|--------|--------------|
| [M0](00-m0-la-idea.md) | La página es un programa | Grilla, celda, buffer. Por qué no un framework |
| [M1](01-m1-primer-buffer.md) | Tu primer buffer | `Buffer`, coordenadas, el primer test |
| [M2](02-m2-layout.md) | Layout y constraints | Repartir el alto; por qué `Min` y `Fill` no son lo mismo |
| [M3](03-m3-widgets.md) | Widgets y copy por ancho | `Block`, `Paragraph`, medir y truncar texto |
| [M4](04-m4-contenido.md) | El contenido como datos | Separar presentación de contenido |
| [M5](05-m5-loop-terminal.md) | El loop de terminal | Cómo se prueba una página sin abrirla |
| [M6](06-m6-buffer-a-html.md) | Buffer → HTML | Spans, escape, CSS. Primer chequeo de mortales |
| [M7](07-m7-links.md) | Regiones clicables | Links, foco, HTML válido |
| [M8](08-m8-i18n.md) | i18n con Fluent | Tres idiomas sin duplicar plantillas |
| [M9](09-m9-servir.md) | Servir desde la Pi | `systemd`, Caddy, y convivir con Pi-hole |
| [M10](10-m10-deploy.md) | Deploy y verificación | Publicar y comprobarlo desde el teléfono |

## Chequeos de mortales

Sólo tres, y son el único momento en que abrís el navegador:

- **M6** — la página existe y se ve.
- **M7** — los links se pueden clickear.
- **M10** — se ve bien desde un teléfono de verdad.

Todo lo demás se prueba en la terminal.
