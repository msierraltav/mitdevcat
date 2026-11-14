# M3 — Widgets y copy por ancho

## Concepto

Un **widget** es algo que sabe dibujarse dentro de un `Rect`. Los que necesitás:

```rust
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

let parrafo = Paragraph::new("hola")
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" mitdevcat "),
    )
    .wrap(Wrap { trim: false });

parrafo.render(area, &mut buf);
```

- `Block` dibuja la caja: bordes, título, estilo.
- `Paragraph` dibuja texto, con alineación y wrapping.
- `Span`/`Line` permiten colores distintos dentro de una misma línea.

Ojo: `render` **consume** el widget. Se usa y se tira; el buffer es lo que queda.

## El problema central del curso

En una grilla, un texto largo **no se reacomoda**: se rompe. Y hay tres formas de
romperlo:

**(a) El texto desborda el ancho.** Escribís 43 caracteres en una caja de 40:
la línea se sale y los bordes de la caja quedan abiertos.

**(b) `Wrap` te salva pero te miente.** Con `Wrap { trim: true }` el texto se
parte en dos líneas y encima **te borra la indentación**. Así se ve un hero
arruinado:

```
│Mit · Santiago de Chile     │   ← la indentación desapareció
│> construyo cosas chicas    │
│que no se caen              │   ← y la segunda línea salió sin prefijo
```

Con `Wrap { trim: false }` la indentación se respeta, pero el texto sigue
cortándose.

**(c) La solución de verdad: no desbordar nunca.** Se trunca **antes** de
dibujar, midiendo el presupuesto, y se marca el corte:

```rust
// "youtube music en terminal" con presupuesto 17
"youtube music en…"
```

Por eso el copy se escribe **por ancho**: en 40 columnas el stack es
"Rust · TypeScript · Linux", en 100 es también "pi-hole enjoyer". No es el mismo
texto acortado por CSS: son dos textos, elegidos por presupuesto.

## Ejercicio

1. Dos funciones de medición, con sus tests:

```rust
/// Corta `texto` a lo sumo en `max` caracteres y marca el corte con '…'.
pub fn truncar(texto: &str, max: usize) -> String { /* vos */ }

/// Une tantos items como entren en `max`, descartando items enteros.
pub fn unir_que_entre(items: &[&str], sep: &str, max: usize) -> String { /* vos */ }
```

2. Un tipo para el copy por ancho:

```rust
pub struct Copy { pub corto: String, pub largo: String }
// Copy::para_ancho(40) -> corto ; Copy::para_ancho(100) -> largo
```

3. Un hero con `Block` de bordes completos, título, tu nombre, la frase y el
   stack — que **cierre la caja** en 40 y en 100 columnas.

## Criterio de aceptación

```rust
#[test]
fn truncar_no_toca_lo_que_entra() {
    assert_eq!(truncar("hola", 4), "hola");
    assert_eq!(truncar("hola", 10), "hola");
}

#[test]
fn truncar_marca_el_corte() {
    assert_eq!(truncar("hola mundo", 7), "hola m…");
    assert_eq!(truncar("hola", 0), "");
}

#[test]
fn unir_cae_items_enteros() {
    assert_eq!(unir_que_entre(&["aaaa", "bbbb", "cccc"], "-", 9), "aaaa-bbbb");
}

#[test]
fn el_hero_cierra_la_caja_en_los_dos_anchos() {
    for ancho in [40u16, 100] {
        let texto = a_texto(&pagina(ancho));
        let filas: Vec<&str> = texto.lines().collect();
        let arriba = filas.iter().position(|l| l.starts_with('┌')).unwrap();
        let abajo = filas.iter().position(|l| l.starts_with('└')).unwrap();
        for fila in &filas[arriba + 1..abajo] {
            assert!(fila.starts_with('│'), "no abre: {fila:?}");
            assert!(fila.ends_with('│'), "no cierra: {fila:?}  (ancho {ancho})");
        }
    }
}
```

- [ ] Los cuatro tests pasan.
- [ ] A 40 columnas, ninguna línea supera 40 caracteres.
- [ ] A 40 columnas el stack **no** dice "pi-hole enjoyer".
- [ ] Probaste `Wrap { trim: true }` y viste con tus ojos el hero arruinado.

## Trampas concretas

**`max - 1` con `max = 0`.** `usize` no tiene negativos: `0 - 1` hace panic en
debug. Usá `saturating_sub` o cortá temprano.

**Medir con `.len()`.** Otra vez: bytes, no caracteres.

**`{:<6}` en `format!`.** Rellena a la derecha hasta 6 caracteres — sirve para
alinear columnas de la lista de proyectos, y cuenta **caracteres**, no bytes.

**El presupuesto de un borde.** Una caja con `Borders::ALL` en 40 columnas tiene
**38** columnas útiles adentro. Si no le restás los bordes, todo el cálculo queda
corrido dos columnas y el test explota con un mensaje que parece mentira.

## Qué aprendiste

- Widgets que se dibujan dentro de un rect, sin estado.
- Que `Wrap` es una red de contención, no una estrategia de layout.
- Medir y truncar con presupuesto: la habilidad central de todo el curso.
