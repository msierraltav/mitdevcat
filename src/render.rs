//! The TUI layer: content plus a column budget becomes a cell buffer.
//!
//! No terminal and no crossterm: a [`Buffer`] is allocated directly and widgets
//! render into it. The same buffer is what the HTML layer serialises.

use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::content::{Content, Width};

/// Total rows of the page. The grid is fixed, like any TUI.
pub const HEIGHT: u16 = 22;

/// Render the whole page for one column budget.
pub fn page(content: &Content, width: Width) -> Buffer {
    let cols = width.cols();
    let area = Rect::new(0, 0, cols, HEIGHT);
    let mut buf = Buffer::empty(area);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(area);

    render_hero(content, width, rows[0], &mut buf);
    render_nav(content, width, rows[1], &mut buf);
    render_projects(content, width, rows[2], &mut buf);
    render_footer(content, width, rows[3], &mut buf);

    buf
}

fn dim() -> Style {
    Style::default().fg(Color::DarkGray)
}

fn render_hero(content: &Content, width: Width, area: Rect, buf: &mut Buffer) {
    // Borders take one column each side; everything else must fit inside.
    let inner = area.width.saturating_sub(2) as usize;
    let indent = "   ";
    let name_line = truncate(
        &format!("{}{} · {}", indent, content.name, content.place),
        inner,
    );
    let tagline = truncate(
        &format!("{}> {}", indent, content.tagline.for_width(width)),
        inner,
    );
    let stack = format!(
        "{}{}",
        indent,
        fit_join(&content.stack, " · ", inner.saturating_sub(indent.len()))
    );

    let hero = Paragraph::new(vec![
        Line::from(""),
        Line::from(name_line)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Line::from(tagline).style(Style::default().fg(Color::White)),
        Line::from(""),
        Line::from(stack).style(Style::default().fg(Color::Green)),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(dim())
            .title(Span::styled(
                " mitdevcat ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
    )
    .wrap(Wrap { trim: false });
    hero.render(area, buf);
}

fn render_nav(content: &Content, width: Width, area: Rect, buf: &mut Buffer) {
    let labels: Vec<String> = content
        .nav
        .iter()
        .map(|link| format!("[ {} ]", link.label))
        .collect();
    let sep = match width {
        Width::Narrow => " ",
        _ => "   ",
    };
    let line = fit_join(&labels, sep, area.width as usize);
    let nav = Paragraph::new(Line::from(line).style(Style::default().fg(Color::Cyan)))
        .alignment(Alignment::Center);
    nav.render(area, buf);
}

fn render_projects(content: &Content, width: Width, area: Rect, buf: &mut Buffer) {
    // Only a left border, so the budget is one column wider.
    let inner = area.width.saturating_sub(1) as usize;
    let mut lines: Vec<Line> = Vec::new();
    for project in &content.projects {
        let prefix = format!("  {:<6}{:<14}", project.lang, project.name);
        let room = inner.saturating_sub(prefix.chars().count());
        let desc = truncate(&project.desc, room);
        lines.push(Line::from(vec![
            Span::styled("  ", dim()),
            Span::styled(format!("{:<6}", project.lang), Style::default().fg(Color::Magenta)),
            Span::styled(
                format!("{:<14}", project.name),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(desc, dim()),
        ]));
    }

    let projects = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::LEFT)
            .border_style(dim())
            .title(Span::styled(" proyectos ", Style::default().fg(Color::White))),
    );
    projects.render(area, buf);
}

fn render_footer(content: &Content, width: Width, area: Rect, buf: &mut Buffer) {
    let items: Vec<String> = content
        .languages
        .iter()
        .map(|link| {
            let code: String = link.label.chars().take(2).collect();
            format!("[{}] {}", code, link.label)
        })
        .collect();
    let sep = match width {
        Width::Narrow => " ",
        _ => "  ",
    };
    let indent = "  ";
    let line = format!(
        "{}{}",
        indent,
        fit_join(&items, sep, (area.width as usize).saturating_sub(indent.len()))
    );
    let footer = Paragraph::new(Line::from(line).style(dim()))
        .block(Block::default().borders(Borders::TOP).border_style(dim()));
    footer.render(area, buf);
}

/// Cut `text` to at most `max` characters, marking the cut with `…`.
pub fn truncate(text: &str, max: usize) -> String {
    if max == 0 {
        return String::new();
    }
    if text.chars().count() <= max {
        return text.to_string();
    }
    let mut out: String = text.chars().take(max - 1).collect();
    out.push('…');
    out
}

/// Join as many items as fit in `max` characters, dropping whole items.
pub fn fit_join(items: &[String], sep: &str, max: usize) -> String {
    let mut out = String::new();
    for item in items {
        if out.is_empty() {
            out.push_str(&truncate(item, max));
        } else if out.chars().count() + sep.chars().count() + item.chars().count() <= max {
            out.push_str(sep);
            out.push_str(item);
        } else {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_no_toca_lo_que_entra() {
        assert_eq!(truncate("hola", 4), "hola");
        assert_eq!(truncate("hola", 10), "hola");
    }

    #[test]
    fn truncate_marca_el_corte() {
        assert_eq!(truncate("hola mundo", 7), "hola m…");
        assert_eq!(truncate("hola", 0), "");
    }

    #[test]
    fn fit_join_cae_items_enteros() {
        let items = vec!["aaaa".to_string(), "bbbb".to_string(), "cccc".to_string()];
        assert_eq!(fit_join(&items, "-", 4), "aaaa");
        assert_eq!(fit_join(&items, "-", 9), "aaaa-bbbb");
        assert_eq!(fit_join(&items, "-", 14), "aaaa-bbbb-cccc");
    }

    #[test]
    fn fit_join_trunca_el_primer_item_si_no_entra() {
        let items = vec!["aaaaaaaa".to_string()];
        assert_eq!(fit_join(&items, "-", 4), "aaa…");
    }
}
