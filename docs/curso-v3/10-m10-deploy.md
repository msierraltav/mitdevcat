# M10 — Deploy y verificación

## Concepto

Falta lo más honesto del curso: **comprobar que otra persona puede verlo.**

El proyecto ya tiene el deploy a GitHub Pages armado (`docs/` + `docs/CNAME`).
Hay dos destinos y conviene entender qué papel juega cada uno:

| Destino | Para qué | Cómo se actualiza |
|---|---|---|
| GitHub Pages | público, siempre arriba, HTTPS gratis | `git push` de `docs/` |
| La Pi (M9) | tu laboratorio, y lo que querés controlar | `cargo run` + reiniciar servicio |

## Generar lo que se publica

Decidí dónde escribe el generador. Dos opciones honestas:

**A. Seguir con `docs/`** (como hoy): `cargo run -- --out docs`. Pero ojo:
`docs/` además tiene `curso-v2` y `curso-v3`, así que el generador **no puede
borrar la carpeta**: escribe sus archivos y no toca el resto.

**B. Cambiar a `build/`** y que el deploy lo copie. Más limpio, un paso más.

Cualquiera de las dos sirve. Lo que no sirve es que el generador borre
`docs/curso-*` y te borre el curso. Escribí primero el chequeo.

## Ejercicio

1. Generar el sitio completo: 3 idiomas × 3 anchos, con links e i18n.
2. Un script `tools/deploy.sh` que: corre los tests, genera, y **aborta si un
   test falla**. Sin tests verdes no se publica.
3. Publicar y verificar.

## Criterio de aceptación

```sh
cargo test && ./tools/deploy.sh
```

- [ ] El script no publica si un test falla (probalo rompiendo un test).
- [ ] `docs/curso-v2` y `docs/curso-v3` siguen existiendo después de generar.
- [ ] La URL pública carga en el teléfono, en datos móviles (no en el wifi de
      casa: así probás internet de verdad).

## Chequeo de mortales #3 — el final

Desde un teléfono, con los datos móviles:

- [ ] La página carga en menos de 2 segundos.
- [ ] Se lee sin hacer zoom (40 columnas es el ancho de un teléfono).
- [ ] Los tres idiomas cambian de verdad.
- [ ] Los links abren.
- [ ] Al girar el teléfono no se rompe nada.

Si algo falla acá, no es "un detalle de CSS": es el módulo que corresponde.
La página angosta es M3. Los links son M7. Los idiomas son M8.

## Cierre del curso

Tenés, al final:

- una página que es **un programa de 60 crates** en vez de 635,
- un loop de **~1 segundo** en una Pi 4,
- tests que prueban el layout, el escape, los links y los idiomas,
- y sabés por qué cada decisión es la que es.

Lo que aprendiste de Rust, sin que el curso lo dijera en voz alta: ownership
(`&Content`), iteradores, `Result`, `match`, `u16` vs `usize`, `format!`,
`#[test]`, y la diferencia entre bytes y caracteres que arruina layouts.

## Para seguir

Ideas que quedan abiertas, cuando quieras:

- **Tema claro/oscuro**: una paleta por `prefers-color-scheme`.
- **ASCII art en el hero**: `figlet` mental, pero medido.
- **Datos vivos**: tus repos de GitHub como un panel más del grid, generados en
  build.
- **Curso v4**: enseñar esto, con estas mismas palabras, a otra persona.

Y la regla que no se negocia: **lo que se rompe, se rompe con un test que lo
caza.** Eso es todo el método.
