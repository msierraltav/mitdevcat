//! Contract tests. These run in the terminal: no browser, no HTTP server.
//!
//! Each test pins one rule from the specs:
//! - S2  : the page is a TUI grid, one render per width.
//! - S2b : copy is authored per width, and nothing overflows the column budget.
//! - S2/S8: the HTML layer escapes user text and emits style classes.

use mitdevcat::content::{Content, Width};
use mitdevcat::html::{buffer_to_html, buffer_to_text};
use mitdevcat::render::page;

fn lines(w: Width) -> Vec<String> {
    let content = Content::sample();
    let buf = page(&content, w);
    buffer_to_text(&buf).lines().map(|l| l.to_string()).collect()
}

#[test]
fn ninguna_linea_excede_las_columnas_del_ancho() {
    for w in Width::ALL {
        let cols = w.cols() as usize;
        for (i, l) in lines(w).iter().enumerate() {
            let len = l.chars().count();
            assert!(
                len <= cols,
                "{:?}: la linea {} mide {} columnas y el presupuesto es {}",
                w,
                i,
                len,
                cols
            );
        }
    }
}

#[test]
fn el_hero_usa_copy_corto_en_narrow_y_largo_en_wide() {
    let c = Content::sample();
    let narrow = buffer_to_text(&page(&c, Width::Narrow));
    let wide = buffer_to_text(&page(&c, Width::Wide));

    assert!(
        narrow.contains(&c.tagline.short),
        "narrow no muestra el copy corto: {:?}",
        narrow
    );
    assert!(
        !narrow.contains(&c.tagline.long),
        "narrow no deberia mostrar el copy largo"
    );
    assert!(
        wide.contains(&c.tagline.long),
        "wide no muestra el copy largo: {:?}",
        wide
    );
}

#[test]
fn el_hero_cierra_la_caja_en_todos_los_anchos() {
    for w in Width::ALL {
        let ls = lines(w);
        let first = ls
            .iter()
            .position(|l| l.starts_with('┌'))
            .unwrap_or_else(|| panic!("{:?}: sin borde superior", w));
        let last = ls
            .iter()
            .position(|l| l.starts_with('└'))
            .unwrap_or_else(|| panic!("{:?}: sin borde inferior", w));
        assert!(last > first, "{:?}: borde inferior antes que el superior", w);

        for (i, l) in ls.iter().enumerate() {
            if i > first && i < last {
                assert!(
                    l.starts_with('│') && l.ends_with('│'),
                    "{:?}: la fila {} no cierra la caja: {:?}",
                    w,
                    i,
                    l
                );
            }
        }
    }
}

#[test]
fn el_texto_no_arrastra_espacios_finales() {
    for w in Width::ALL {
        let text = buffer_to_text(&page(&Content::sample(), w));
        for (i, l) in text.lines().enumerate() {
            assert_eq!(
                l,
                l.trim_end(),
                "{:?}: la fila {} termina en espacios",
                w,
                i
            );
        }
    }
}

#[test]
fn el_html_escapa_caracteres_peligrosos() {
    let mut buf = ratatui::buffer::Buffer::empty(ratatui::layout::Rect::new(0, 0, 10, 1));
    buf.set_string(0, 0, "<a&b>", ratatui::style::Style::default());

    let html = buffer_to_html(&buf);

    assert!(!html.contains("<a&b>"), "texto sin escapar: {}", html);
    assert!(
        html.contains("&lt;a&amp;b&gt;"),
        "escape incorrecto: {}",
        html
    );
}

#[test]
fn el_html_emite_clases_de_color() {
    let mut buf = ratatui::buffer::Buffer::empty(ratatui::layout::Rect::new(0, 0, 4, 1));
    buf.set_string(
        0,
        0,
        "ok",
        ratatui::style::Style::default().fg(ratatui::style::Color::Cyan),
    );

    let html = buffer_to_html(&buf);

    assert!(
        html.contains("class=\"f-cyan\""),
        "no se emitio la clase esperada: {}",
        html
    );
}

#[test]
fn el_html_no_deja_celdas_fuera_de_la_grilla() {
    let c = Content::sample();
    let buf = page(&c, Width::Wide);
    let html = buffer_to_html(&buf);

    // one <pre> body, one newline per buffer row
    assert_eq!(
        html.matches('\n').count(),
        buf.area.height as usize,
        "el HTML no tiene una fila por linea del buffer"
    );
}
