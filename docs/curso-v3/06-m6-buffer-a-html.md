# M6 — Buffer → HTML

## Concepto

Ahora el buffer se convierte en la página que el mundo ve. Y es más simple de lo
que parece: **una `<pre>` y spans.**

```html
<pre class="term w100"><span class="f-cyan bold">mitdevcat</span>
<span class="f-darkgray">│   </span>Mit · Santiago de Chile
</pre>
```

Tres piezas y listo:

1. **`<pre>` con `white-space: pre`** — el navegador respeta espacios y saltos de
   línea. Sin eso, el HTML colapsa los espacios y la grilla desaparece.
2. **Fuente monoespaciada** — si no, "i" y "M" ocupan distinto y nada alinea.
3. **`<span class="f-*">`** — un span por **run** de celdas con el mismo estilo,
   no uno por celda. Un span por celda genera 4000 spans por página y el
   navegador sufre.

## El algoritmo (pensalo antes de escribirlo)

Recorrés fila por fila. Dentro de una fila:

```
x = 0
mientras x < ancho:
    estilo = estilo_de(x, y)
    juntar todas las celdas siguientes que tengan EL MISMO estilo
    emitir un span con ese texto (escapado) y ese estilo
    x = donde terminó el run
```

Eso es todo. La parte difícil es **escapar** y **mapear colores a clases**.

## Escapar no es opcional

Si tu página dice `Rust & TypeScript` y no escapás, el navegador se come el `&`.
Si dice `<` tenés HTML roto. Escapá siempre:

| carácter | se emite como |
|---|---|
| `&` | `&amp;` |
| `<` | `&lt;` |
| `>` | `&gt;` |

Y ojo: escapá **después** de agrupar el run, no antes, para no romper el conteo
de celdas.

## Ejercicio

1. `a_html(buf: &Buffer) -> String`, con runs y escape.
2. Un mapa `Color → clase CSS` (`Color::Cyan` → `f-cyan`, etc.) y un
   `assets/tui.css` con la paleta, la fuente monoespaciada y `white-space: pre`.
3. El binario que escribe `build/index.html` con **los tres anchos**:

```html
<pre class="term w40">...</pre>
<pre class="term w60">...</pre>
<pre class="term w100">...</pre>
```

y CSS que muestre uno solo:

```css
@media (min-width: 900px) { pre.w60 { display: none } pre.w100 { display: block } }
```

## Chequeo de mortales #1

```sh
cargo run
python3 -m http.server -d build 8080
```

Abrí `http://<la-pi>:8080` y achicá la ventana. Tiene que cambiar de ancho en los
cortes. **Es el único momento del módulo en que abrís el navegador.**

## Criterio de aceptación

```rust
#[test]
fn el_html_escapa_caracteres_peligrosos() {
    // un buffer con "<a&b>" no puede aparecer crudo en la salida
}

#[test]
fn el_html_emite_una_fila_por_linea() {
    let html = a_html(&pagina(&Content::sample(), 100));
    assert_eq!(html.matches('\n').count(), ALTO as usize);
}

#[test]
fn el_html_no_tiene_un_span_por_celda() {
    let html = a_html(&pagina(&Content::sample(), 100));
    // con runs, son decenas; con un span por celda, miles
    assert!(html.matches("<span").count() < 200, "estás emitiendo un span por celda");
}
```

- [ ] Los tres tests pasan.
- [ ] La página se ve en el navegador y cambia de ancho en los cortes.
- [ ] Entendés por qué el test del span-por-celda existe (rendimiento real).

## Accesibilidad: no la dejes para el final

La grilla no tiene semántica. Recuperá lo mínimo **ya**, mientras el módulo está
fresco:

- `<html lang="es">` — el idioma real.
- `<title>` y `<meta name="description">`.
- Un bloque oculto `.sr-only` con `<h1>`, `<nav>` y los links reales: invisible
  en la grilla, presente para lectores de pantalla y buscadores.

No es híbrido: la página que se ve sigue siendo 100% grilla. Es no esconderle la
información a quien no puede ver celdas.

## Qué aprendiste

- HTML es texto; la grilla sobrevive si no la reflowás.
- Runs en vez de celdas: una decisión de rendimiento que se testea.
- Que escapar es una regla, no una precaución.
