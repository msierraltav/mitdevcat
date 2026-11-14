//! mitdevcat — generate the static site.
//!
//! The page is rendered as a TUI grid at three column budgets, then emitted as
//! static HTML. Development and testing happen in the terminal; the browser is
//! only the final check.

use std::path::PathBuf;
use std::process::ExitCode;

use mitdevcat::content::{Content, Width};
use mitdevcat::html::{buffer_to_html, buffer_to_text, escape};
use mitdevcat::render::page;

const CSS: &str = include_str!("../assets/tui.css");

const USAGE: &str = "\
mitdevcat — static site generator (TUI grid -> HTML)

USAGE:
    mitdevcat [--out <dir>] [--print]

OPTIONS:
    --out <dir>    output directory (default: build)
    --print        print the 100-column render as text to stdout
    -h, --help     show this help
";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

struct Args {
    out: PathBuf,
    print: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut args = Args {
        out: PathBuf::from("build"),
        print: false,
    };
    let mut rest = std::env::args().skip(1);
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--out" => {
                let dir = rest.next().ok_or("--out necesita un directorio")?;
                args.out = PathBuf::from(dir);
            }
            "--print" => args.print = true,
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            other => return Err(format!("opcion desconocida: {other}\n\n{USAGE}")),
        }
    }
    Ok(args)
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    let content = Content::sample();

    if args.print {
        print!("{}", buffer_to_text(&page(&content, Width::Wide)));
        return Ok(());
    }

    std::fs::create_dir_all(&args.out).map_err(|e| format!("{}: {e}", args.out.display()))?;

    let mut panels = String::new();
    for width in Width::ALL {
        let buf = page(&content, width);
        let text_path = args.out.join(format!("page-{}.txt", width.slug()));
        std::fs::write(&text_path, buffer_to_text(&buf))
            .map_err(|e| format!("{}: {e}", text_path.display()))?;
        panels.push_str(&format!(
            "<pre class=\"term w{}\" aria-label=\"mitdevcat, {} columnas\">\n{}</pre>\n",
            width.slug(),
            width.cols(),
            buffer_to_html(&buf)
        ));
    }

    let index_path = args.out.join("index.html");
    std::fs::write(&index_path, document(&content, &panels))
        .map_err(|e| format!("{}: {e}", index_path.display()))?;

    println!(
        "escrito {} (40/60/100 columnas, {} proyectos)",
        index_path.display(),
        content.projects.len()
    );
    Ok(())
}

fn document(content: &Content, panels: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"es\">\n<head>\n\
<meta charset=\"utf-8\">\n\
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n\
<title>mitdevcat — {name}</title>\n\
<meta name=\"description\" content=\"{desc}\">\n\
<style>\n{css}</style>\n</head>\n<body>\n{summary}{panels}</body>\n</html>\n",
        name = escape(&content.name),
        desc = escape(&content.tagline.long),
        css = CSS,
        summary = summary(content),
        panels = panels,
    )
}

/// Real HTML for screen readers and crawlers: the grid stays a grid (S2), but
/// the information is still reachable without seeing cells (S2b).
fn summary(content: &Content) -> String {
    let mut projects = String::new();
    for project in &content.projects {
        projects.push_str(&format!(
            "<li><a href=\"{}\">{}</a>: {}</li>",
            escape(&project.href),
            escape(&project.name),
            escape(&project.desc)
        ));
    }
    let nav: Vec<String> = content
        .nav
        .iter()
        .map(|link| format!("<a href=\"{}\">{}</a>", escape(&link.href), escape(&link.label)))
        .collect();
    format!(
        "<div class=\"sr-only\"><h1>{} — {}</h1><p>{} · {}</p>\
<p>{}</p><nav>{}</nav><ul>{}</ul><p>{}</p></div>\n",
        escape(&content.name),
        escape(&content.place),
        escape(&content.tagline.long),
        escape(&content.stack.join(", ")),
        escape(&content.copyright),
        nav.join(" "),
        projects,
        escape(&content.copyright),
    )
}
