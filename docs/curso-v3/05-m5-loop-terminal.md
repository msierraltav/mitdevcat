# M5 — El loop de terminal

## Concepto

Esta es la clase que cambia cómo trabajás. Una página que se dibuja como grilla
se puede **leer como texto**, y el texto se compara con `diff`.

```
contenido ──► Buffer ──► texto ──► tu ojo / tus tests
```

Ese salto a texto es el loop. No necesitás navegador, ni servidor, ni recargar
nada: corrés el programa y mirás.

```sh
cargo run -- --print
```

## Devolver el texto por stdout

Agregá un binario (`src/main.rs`) con dos modos:

```rust
// mitdevcat [--out <dir>] [--print]
```

- `--print` escribe el render de 100 columnas a la salida estándar.
- Sin `--print`, genera los archivos (en M6).

Así podés hacer esto, que es todo el loop:

```sh
cargo run -- --print            # mirar
cargo run -- --print > /tmp/a.txt
# ...editás...
cargo run -- --print > /tmp/b.txt
diff -u /tmp/a.txt /tmp/b.txt   # ver qué cambió
```

## Golden tests: la red de seguridad

Un **golden test** compara la salida completa contra un archivo guardado. En una
página de grilla es la herramienta natural, y también la más peligrosa:
si lo regenerás cada vez que falla, no prueba nada.

Regla de la casa:

- **Golden** para el dibujo completo (detecta cambios que no pediste).
- **Propiedades** para las reglas (ninguna línea excede el ancho, los bordes
  cierran, el copy corto sólo aparece en angosto).

Las propiedades te dicen **qué** está mal. El golden te dice **dónde** cambió.

## Ejercicio

1. Binario con `--print` y `--out`.
2. Un script `tools/check.sh` que corra, en orden: `cargo test` y después el
   `--print`, para que veas el resultado sin escribir nada más.
3. Guardá un golden de 40 y otro de 100 columnas en `tests/golden/`.
4. Escribí el test que compara contra el golden, con la instrucción de cómo
   regenerarlo **a mano** (nunca desde el propio test).

## Criterio de aceptación

```rust
#[test]
fn el_render_de_100_columnas_no_cambio() {
    let esperado = include_str!("golden/page-100.txt");
    let actual = a_texto(&pagina(&Content::sample(), 100));
    assert_eq!(actual, esperado, "regenerá el golden con `cargo run -- --print` y revisá el diff");
}
```

- [ ] `cargo test` corre los tests y el script muestra el render.
- [ ] El golden de 100 existe y coincide.
- [ ] Rompiste algo a propósito (cambiá una palabra del contenido) y viste el
      test fallar con un diff legible.
- [ ] `checksum` mental: entendés que regenerar el golden es una decisión, no
      un reflejo.

## Higiene del loop

- **`cargo check` antes de `cargo run`** cuando el cambio es grande y no querés
  esperar al binario.
- **Nunca `cargo install`** en esta máquina: horas. `rustup component add
  rust-analyzer` sí, es binario prebuilt y te da errores en el editor.
- Un test que tarda más de un segundo en este proyecto es un test mal diseñado.

## Qué aprendiste

- El loop de una página puede ser texto, y entonces es rapidísimo.
- Golden vs propiedades: qué prueba cada uno y cuándo cada uno miente.
- A automatizar el chequeo que hacías a ojo.
