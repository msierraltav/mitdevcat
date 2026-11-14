# M7 — Regiones clicables

## Concepto

En una grilla, un botón no existe. Existe un **rectángulo de celdas** que tiene
que comportarse como link. Entonces el render tiene que **informar dónde puso
qué**:

```rust
pub struct HitArea {
    pub rect: Rect,   // dónde están esas celdas
    pub href: String, // a dónde llevan
}

pub fn pagina(contenido: &Content, ancho: u16) -> (Buffer, Vec<HitArea>);
```

Ese es el cambio de diseño del módulo: la función ahora devuelve **dos cosas**.
La página es el dibujo, y los hit areas son la capa de interacción.

## El problema del HTML anidado

Al serializar, las celdas de un hit area van envueltas en `<a>`. Y acá hay una
trampa: los spans de color también envuelven celdas. Si no tenés cuidado generás

```html
<a href="/es/"><span>es</span><a href="/en/">en</a></a>   <!-- inválido -->
```

Regla: **las regiones no se solapan**, y se cortan en el borde del `Rect`. Lo más
simple es generar el `<a>` **por fuera** del span, o cortar el run cuando entra o
sale de un hit area. Elegí una estrategia y escribí el test que la clava.

## Foco y teclado

Sin `<button>` ni `tabindex` ordenado, el que navega con teclado queda afuera.
Mínimo viable:

- Un `<a href>` real por destino (el navegador ya lo hace focusable).
- Orden de tab = orden en el documento. Si tus hit areas no están en orden
  visual, el tab salta raro. Ordenalos vos al emitirlos.

## Ejercicio

1. Cambiar la firma a `-> (Buffer, Vec<HitArea>)`.
2. Registrar un hit area para cada link de la navegación, cada proyecto y cada
   idioma. Para eso necesitás saber en qué `Rect` cae cada uno: `Rect` tiene
   `x`, `y`, `width`, `height`, y podés construirlo desde la posición donde
   empezaste a dibujar.
3. En la capa HTML, envolver esas celdas en `<a>`.
4. Un test por cada cosa que puede salir mal.

## Criterio de aceptación

```rust
#[test]
fn hay_un_link_por_cada_destino() {
    let (_, links) = pagina(&Content::sample(), 100);
    let c = Content::sample();
    assert_eq!(links.len(), c.nav.len() + c.proyectos.len() + c.idiomas.len());
}

#[test]
fn el_html_no_anida_links() {
    let (buf, links) = pagina(&Content::sample(), 100);
    let html = a_html(&buf, &links);
    // contador simple: cada <a ...> abre, cada </a> cierra, y no se abre dos veces seguidas
    let mut abiertos = 0i32;
    for token in html.split('<') {
        if token.starts_with("a ") { abiertos += 1; assert!(abiertos <= 1, "<a> anidado"); }
        if token.starts_with("/a>") { abiertos -= 1; }
    }
    assert_eq!(abiertos, 0, "quedaron links sin cerrar");
}

#[test]
fn las_regiones_no_se_salen_de_la_grilla() {
    let (buf, links) = pagina(&Content::sample(), 40);
    for link in links {
        assert!(link.rect.right() <= buf.area.width, "{:?} se sale", link.rect);
        assert!(link.rect.bottom() <= buf.area.height);
    }
}
```

- [ ] Los tres tests pasan.
- [ ] Cada link de la página apunta a un destino que existe.

## Chequeo de mortales #2

Con el ratón: clickeá cada link. Con el teclado: `Tab` y `Enter`. Si el foco
salta desordenado, no es un problema de CSS: es tu orden de emisión.

## Qué aprendiste

- A devolver más de una cosa de una función (y por qué no hace falta inventar un
  struct de retorno gigante).
- Que la interacción es un dato, no un efecto.
- Que "casi HTML válido" es HTML inválido y se testea.
