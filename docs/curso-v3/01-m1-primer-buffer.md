# M1 — Tu primer buffer

## Concepto

Un `Buffer` es memoria: un rectángulo de celdas. Nada más. No sabe de HTML, no
sabe de terminales, no dibuja nada en pantalla.

`ratatui` te da el tipo y las coordenadas. Vos ponés los caracteres.

```rust
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

let mut buf = Buffer::empty(Rect::new(0, 0, 40, 10)); // x, y, ancho, alto
buf.set_string(3, 2, "Mit", Default::default());      // x, y, texto, estilo
```

Fijate en tres cosas:

1. **Las coordenadas son (x, y)**, con origen en la esquina de arriba a la
   izquierda. `x` es columna, `y` es fila.
2. **`u16`**, no `usize`. Un ancho de pantalla nunca va a necesitar más.
3. **Nada de esto dibuja**: el buffer es una estructura de datos. Recién en M6
   se convierte en algo que un navegador entiende.

## Ejercicio

En `~/code/mitdevcat-curso`:

```sh
cargo add ratatui --no-default-features -F std
```

1. En `src/lib.rs`, escribí una función `dibujar() -> Buffer` que devuelva un
   buffer de **40 × 10** con tu nombre en la fila 2, columna 3.
2. Escribí una función `a_texto(buf: &Buffer) -> String` que recorra todas las
   celdas y devuelva un `String`, una línea por fila.
3. Escribí el test.

Pistas de API, no de código:

- Se recorre con dos `for` anidados: filas (y) y columnas (x).
- Una celda se lee con `&buf[(x, y)]` y su texto con `.symbol()`.
- Para no arrastrar espacios al final de cada línea: `String::trim_end()`.

## Criterio de aceptación

```rust
#[test]
fn el_primer_buffer_dice_mi_nombre() {
    let texto = a_texto(&dibujar());
    assert_eq!(texto.lines().nth(2).unwrap(), "   Mit");
}

#[test]
fn ninguna_linea_excede_el_ancho() {
    for linea in a_texto(&dibujar()).lines() {
        assert!(linea.chars().count() <= 40);
    }
}
```

- [ ] Ambos tests pasan.
- [ ] `cargo test` no tiene warnings.
- [ ] Entendés por qué el primer test espera **tres espacios** antes de `Mit`.

## Errores que vas a cometer (y está bien)

**`&str` vs `String`.** `set_string` pide algo que se pueda leer como texto. Tu
función `a_texto` tiene que **construir** un `String` nuevo, porque no puede
devolver prestado algo que vive adentro del buffer.

**Índices fuera de rango.** Si pedís `buf[(40, 0)]` en un buffer de 40 columnas,
panic: las columnas válidas van de 0 a 39. Rust te protege de escribir donde no
hay nada, y te lo va a decir con el nombre exacto.

**Contar bytes en vez de caracteres.** `"ñ".len()` es **2** (bytes UTF-8), pero
ocupa **1** celda. Para medir celdas usá `.chars().count()`. Esa distinción te
va a salvar en M3 y otra vez en M8.

## Cómo se mira

Todavía no hay HTML. Para ver tu buffer:

```sh
cargo test -- --nocapture
```

o agregá un `println!("{}", a_texto(&dibujar()))` en un test temporal.

## Qué aprendiste

- Un buffer es datos, no una pantalla.
- Coordenadas (x, y) y el tipo `u16`.
- La diferencia entre bytes y caracteres, que en este curso no es un detalle:
  es todo el layout.
