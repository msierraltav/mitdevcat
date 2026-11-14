//! The HTML layer: a rendered cell buffer becomes text or static HTML.
//!
//! No wasm, no JavaScript, no terminal: the buffer is the whole interface.

use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier};

/// Plain text, one line per buffer row, without trailing padding.
pub fn buffer_to_text(buf: &Buffer) -> String {
    let mut out = String::new();
    for y in 0..buf.area.height {
        let mut row = String::new();
        for x in 0..buf.area.width {
            row.push_str(buf[(x, y)].symbol());
        }
        out.push_str(row.trim_end());
        out.push('\n');
    }
    out
}

/// Static HTML: one `<span>` per run of cells sharing a style.
pub fn buffer_to_html(buf: &Buffer) -> String {
    let mut out = String::new();
    for y in 0..buf.area.height {
        let mut x = 0u16;
        while x < buf.area.width {
            let key = key_at(buf, x, y);
            let mut text = String::new();
            while x < buf.area.width && key_at(buf, x, y) == key {
                text.push_str(buf[(x, y)].symbol());
                x += 1;
            }
            let class = class_attr(key);
            if class.is_empty() {
                out.push_str(&escape(&text));
            } else {
                out.push_str("<span class=\"");
                out.push_str(&class);
                out.push_str("\">");
                out.push_str(&escape(&text));
                out.push_str("</span>");
            }
        }
        out.push('\n');
    }
    out
}

/// (foreground, background, modifier bits) of one cell.
type Key = (Option<&'static str>, Option<&'static str>, u8);

fn key_at(buf: &Buffer, x: u16, y: u16) -> Key {
    let cell = &buf[(x, y)];
    let mut mods = 0u8;
    if cell.modifier.contains(Modifier::BOLD) {
        mods |= BOLD;
    }
    if cell.modifier.contains(Modifier::ITALIC) {
        mods |= ITALIC;
    }
    if cell.modifier.contains(Modifier::DIM) {
        mods |= DIM;
    }
    (color_class(cell.fg), color_class(cell.bg), mods)
}

const BOLD: u8 = 1;
const ITALIC: u8 = 2;
const DIM: u8 = 4;

fn class_attr(key: Key) -> String {
    let mut out = String::new();
    if let Some(fg) = key.0 {
        out.push_str("f-");
        out.push_str(fg);
    }
    if let Some(bg) = key.1 {
        push_token(&mut out, "b-");
        out.push_str(bg);
    }
    if key.2 & BOLD != 0 {
        push_token(&mut out, "bold");
    }
    if key.2 & ITALIC != 0 {
        push_token(&mut out, "italic");
    }
    if key.2 & DIM != 0 {
        push_token(&mut out, "dim");
    }
    out
}

fn push_token(out: &mut String, token: &str) {
    if !out.is_empty() {
        out.push(' ');
    }
    out.push_str(token);
}

fn color_class(color: Color) -> Option<&'static str> {
    Some(match color {
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
        Color::Yellow => "yellow",
        Color::Blue => "blue",
        Color::Magenta => "magenta",
        Color::Cyan => "cyan",
        Color::Gray => "gray",
        Color::DarkGray => "darkgray",
        Color::LightRed => "lightred",
        Color::LightGreen => "lightgreen",
        Color::LightYellow => "lightyellow",
        Color::LightBlue => "lightblue",
        Color::LightMagenta => "lightmagenta",
        Color::LightCyan => "lightcyan",
        Color::White => "white",
        _ => return None,
    })
}

/// Escape text so it is safe inside HTML text nodes and attributes.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;
    use ratatui::style::Style;

    #[test]
    fn el_texto_recorta_el_relleno_de_cada_fila() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 8, 2));
        buf.set_string(0, 0, "hola", Style::default());
        let text = buffer_to_text(&buf);
        assert_eq!(text, "hola\n\n");
    }

    #[test]
    fn los_modificadores_se_traducen_a_clases() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 4, 1));
        buf.set_string(
            0,
            0,
            "x",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD | Modifier::DIM),
        );
        let html = buffer_to_html(&buf);
        assert!(html.contains("f-green bold dim"), "html: {}", html);
    }
}
