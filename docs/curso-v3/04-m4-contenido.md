# M4 — El contenido como datos

## Concepto

Hasta acá el texto está incrustado en el medio del dibujo. Eso funciona una vez
y se vuelve una trampa en cuanto necesitás:

- **tres anchos** distintos (40/60/100),
- **tres idiomas** (es/en/ca),
- **datos que cambian** (tus proyectos, tu descripción).

La regla es vieja como la programación: **el contenido es un dato, el render es
una función.** El render no sabe qué dice tu página; sabe cómo se dibuja una
página.

```rust
pub struct Content {
    pub nombre: String,
    pub lugar: String,
    pub frase: Copy,            // Copy { corto, largo } — de M3
    pub stack: Vec<String>,
    pub proyectos: Vec<Proyecto>,
}

pub struct Proyecto {
    pub lang: String,
    pub nombre: String,
    pub desc: String,
    pub href: String,
}

// El render pide contenido y ancho. Nunca literales.
pub fn pagina(contenido: &Content, ancho: u16) -> Buffer
```

## Por qué esto es Rust y no burocracia

Fijate qué pide la firma: `&Content`. Un préstamo, no una copia. El render lee el
contenido y no lo consume, así que el que llama lo conserva y lo puede reusar
para los tres anchos y los tres idiomas — **seis renders, un solo `Content`**.

Si en cambio pasaras `Content` por valor, el primer render se quedaría con el
contenido y el segundo no compilaría. Ese error del compilador es la lección.

Para estructuras grandes y muy compartidas aparecería un `Rc<Content>`. Todavía
no hace falta: no optimices antes de tener el problema.

## Ejercicio

1. Mover **todo** el texto de tu página a `Content::sample()` (nombre y apellido,
   lugar, frase corta y larga, stack, tres proyectos con su descripción y link).
2. Cambiar `pagina(ancho)` por `pagina(contenido: &Content, ancho: u16)`.
3. Verificar con un test que **el render no tiene literales propios**.

Ese último punto es un test raro y hermoso: no mira la salida, mira el
comportamiento.

```rust
#[test]
fn el_render_usa_el_contenido_que_le_das() {
    let mut c = Content::sample();
    c.nombre = "Nombre de prueba".to_string();
    let texto = a_texto(&pagina(&c, 100));
    assert!(texto.contains("Nombre de prueba"));
}
```

Si el render tuviera "Mit" escrito adentro, este test falla. Ahí está el valor.

## Criterio de aceptación

- [ ] `Content::sample()` tiene todos los textos; no queda ninguno en el render.
- [ ] `pagina` recibe `&Content` y funciona con contenido inventado.
- [ ] Los tests de M3 siguen pasando (no rompiste nada).
- [ ] El test de "nombre de prueba" pasa.

## Puerta que queda abierta

Con el contenido separado, en M8 el i18n es casi gratis: un `Content` por
idioma, el mismo render. Si el texto estuviera incrustado, habría que reescribir
todo el dibujo tres veces.

## Qué aprendiste

- Separar datos de presentación, con la firma de la función como contrato.
- Préstamos (`&T`) como forma de compartir sin copiar y sin perder el contenido.
- Que hay tests que prueban **arquitectura** y no salida.
