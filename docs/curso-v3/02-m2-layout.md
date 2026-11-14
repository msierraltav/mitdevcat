# M2 — Layout y constraints

## Concepto

Tenés 22 filas de alto y querés hero arriba, navegación, proyectos y pie. Podrías
calcular los `y` a mano — y lo harías mal en cuanto cambie una sección.

`ratatui` trae un **solver de restricciones**: le das prioridades y él reparte.

```rust
use ratatui::layout::{Constraint, Direction, Layout};

let filas = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(9),  // hero: 9 filas, sí o sí
        Constraint::Length(3),  // nav: 3
        Constraint::Min(6),     // proyectos: al menos 6, se queda el resto
        Constraint::Length(3),  // pie: 3
    ])
    .split(area);
```

Las tres constraints que vas a usar todo el curso:

| Constraint | Significado | Cuándo |
|---|---|---|
| `Length(n)` | exactamente `n` filas | zonas de tamaño fijo (hero, pie) |
| `Min(n)` | al menos `n`, se lleva el sobrante | la zona que absorbe el espacio libre |
| `Fill(peso)` | reparte el sobrante por peso | dos paneles que crecen juntos |
| `Percentage(n)` | `n%` del total | rara vez: es frágil cuando cambia el alto |

**La regla que importa:** si el total de `Length` supera el alto disponible, las
últimas zonas quedan con **cero filas**. No hay error, no hay panic: hay una
sección que desaparece. Es el bug clásico de las TUIs.

## Ejercicio

1. Definí una constante `ALTO: u16 = 22` y una función
   `pagina(ancho: u16) -> Buffer` que reparta las cuatro zonas.
2. En cada zona, escribí el nombre de la zona (`hero`, `nav`, `proyectos`,
   `pie`) para ver dónde cayó.
3. Escribí un test que verifique que **ninguna zona mide cero**.

## Criterio de aceptación

```rust
#[test]
fn ninguna_zona_queda_vacia() {
    let filas = zonas(40); // tu función que devuelve Vec<Rect>
    for (i, r) in filas.iter().enumerate() {
        assert!(r.height > 0, "la zona {i} quedó con alto 0: {r:?}");
    }
}

#[test]
fn las_zonas_suman_el_alto_total() {
    let total: u16 = zonas(40).iter().map(|r| r.height).sum();
    assert_eq!(total, ALTO);
}
```

- [ ] Ambos tests pasan.
- [ ] Provocaste el bug a propósito: cambiaste un `Length(9)` por `Length(30)`
      y viste una zona en cero. **Hacelo**: entender el síntoma es el objetivo.

## Si te trabás

- `split` devuelve `Rc<[Rect]>`, así que se indexa como `filas[0]`.
- `Rect` tiene `x`, `y`, `width`, `height`. `area` se arma con
  `Rect::new(0, 0, ancho, ALTO)`.
- Si un test falla por alturas, imprimí `{:?}` de las zonas: el `Rect` te dice
  todo.

## Por qué no calculamos a mano

Porque el reparto tiene reglas que se descubren después: qué pasa si `Min` no
alcanza, cómo se ordenan dos `Min`, qué ocurre si el terminal mide menos que la
suma de los `Length`. El solver ya resolvió eso. Vos declarás la intención.

## Qué aprendiste

- Repartir espacio es un problema de restricciones, no de aritmética.
- `Length` es una promesa que puede romper el layout; `Min` absorbe.
- Un layout puede fallar **en silencio**: por eso se testea.
