# M0 — La página es un programa

## Concepto

Una página web parece un documento, pero también es un **dibujo**. Pensala así:
tenés una hoja cuadriculada, con columnas y filas. Cada cruce es una **celda**,
y cada celda guarda exactamente un carácter (y un color, y un estilo).

Eso es una **TUI** (terminal user interface): un programa que dibuja en una
grilla fija de caracteres. Y esa grilla es, literalmente, la misma que usa un
HTML viejo y noble: texto en una fuente monoespaciada.

Lo que cambia todo: **no necesitás un framework para esto.**

Un framework de componentes (React, Dioxus, Leptos) resuelve un problema que
nosotros no tenemos: mantener un árbol de componentes sincronizado con eventos
del usuario en tiempo real. Nuestra página es estática: dice lo que tiene que
decir y no reacciona a nada. Traer un motor de reconciliación para eso es pagar
un precio enorme por una capacidad que no usás.

Y el precio, en esta máquina, es este:

```
Dioxus 0.7 + wasm:  635 crates   ·   loop de 3m 29s
ratatui solo:        60 crates   ·   loop de ~1s
```

## La decisión central del curso

La página se dibuja como una grilla y se emite como HTML estático.

```
contenido  ──►  ratatui (calcula celdas)  ──►  Buffer  ──►  HTML
```

Tres consecuencias que vas a vivir en cada módulo:

1. **Una celda, un carácter.** No hay "que el navegador decida". Vos decidís.
2. **La grilla no se reacomoda sola.** Si un texto es más largo que el ancho,
   no fluye a la línea siguiente: **se rompe el dibujo**. Por eso hay un
   presupuesto de columnas y hay que medir.
3. **No hay semántica HTML.** No hay `<h1>`, ni `<nav>`, ni tab order. Hay que
   recuperarla a mano (lo hacemos en M6 y M7).

Consecuencia 2 es la más importante del curso entero: **en una grilla, el copy se
escribe por ancho**, no se "arregla" con CSS.

## Ejercicio (sin computadora)

Antes de escribir una línea de Rust, dibujá la página en papel cuadriculado.

- Ancho: **40 columnas** (el teléfono).
- Alto: el que quieras, entre 15 y 25 filas.
- Tiene que tener: el nombre, una frase que diga qué hacés, tres proyectos y
  una forma de contacto.

Reglas:
- Un carácter por celda. Ni uno más.
- Lo que no entre, no entra: acortalo o cortalo con `…`.
- Contá las columnas de cada línea **a mano**.

## Criterio de aceptación

- [ ] Todas las líneas miden **40 columnas o menos**.
- [ ] Ninguna línea queda cortada a mitad de palabra sin marca.
- [ ] Podés señalar, en tu dibujo, dónde va cada una de las tres secciones.

Si una línea no entra, no la estires: **reescribí el texto**. Ese es el trabajo
real de maquetar en grilla, y es exactamente lo que el código va a tener que
hacer solo más adelante.

## Qué aprendiste

- Qué es un buffer y por qué es más simple que un DOM.
- Por qué un framework es una decisión de costo, no de modernidad.
- Que el texto es parte del layout, no una capa separada.

## Números para tener a mano

```
ratatui con features por defecto:  188 crates
ratatui default-features = false:   60 crates
```

La diferencia son `crossterm`, `all-widgets`, `widget-calendar` y `macros`.
No los necesitamos: no hay terminal, no hay eventos, no hay calendario.
