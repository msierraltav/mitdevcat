//! The page as data.
//!
//! Rendering never hardcodes copy: it reads a [`Content`]. Copy is authored per
//! width (S2b) because a cell grid wraps at fixed columns, so a single long
//! string cannot reflow into every budget.

/// A piece of copy with a short variant for narrow grids and a long one for wide.
#[derive(Debug, Clone)]
pub struct Copy {
    pub short: String,
    pub long: String,
}

impl Copy {
    pub fn new(short: impl Into<String>, long: impl Into<String>) -> Self {
        Self {
            short: short.into(),
            long: long.into(),
        }
    }

    /// The variant that fits the given budget.
    pub fn for_width(&self, width: Width) -> &str {
        match width {
            Width::Narrow => &self.short,
            _ => &self.long,
        }
    }
}

/// A labelled destination. Rendered as a clickable region (S2c).
#[derive(Debug, Clone)]
pub struct Link {
    pub label: String,
    pub href: String,
}

/// One row of the projects panel.
#[derive(Debug, Clone)]
pub struct Project {
    pub lang: String,
    pub name: String,
    pub desc: String,
    pub href: String,
}

/// The column budgets the site is authored for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    Narrow,
    Medium,
    Wide,
}

impl Width {
    pub const ALL: [Width; 3] = [Width::Narrow, Width::Medium, Width::Wide];

    /// The column budget of this width. This is the hard limit: no cell may
    /// live outside it.
    pub fn cols(self) -> u16 {
        match self {
            Width::Narrow => 40,
            Width::Medium => 60,
            Width::Wide => 100,
        }
    }

    /// Stable suffix used in generated file names.
    pub fn slug(self) -> &'static str {
        match self {
            Width::Narrow => "40",
            Width::Medium => "60",
            Width::Wide => "100",
        }
    }
}

/// Everything the page says.
#[derive(Debug, Clone)]
pub struct Content {
    pub name: String,
    pub place: String,
    pub tagline: Copy,
    pub stack: Vec<String>,
    pub nav: Vec<Link>,
    pub projects: Vec<Project>,
    pub languages: Vec<Link>,
    pub copyright: String,
}

impl Content {
    /// The real content of mitdevcat.
    pub fn sample() -> Self {
        Self {
            name: "Mit".to_string(),
            place: "Santiago de Chile".to_string(),
            tagline: Copy::new(
                "cosas chicas que no se caen",
                "construyo cosas chicas que no se caen",
            ),
            stack: vec![
                "Rust".to_string(),
                "TypeScript".to_string(),
                "Linux".to_string(),
                "pi-hole enjoyer".to_string(),
            ],
            nav: vec![
                Link {
                    label: "proyectos".to_string(),
                    href: "#proyectos".to_string(),
                },
                Link {
                    label: "sobre mi".to_string(),
                    href: "#sobre-mi".to_string(),
                },
                Link {
                    label: "contacto".to_string(),
                    href: "#contacto".to_string(),
                },
            ],
            projects: vec![
                Project {
                    lang: "rust".to_string(),
                    name: "mitdevcat".to_string(),
                    desc: "este mismo sitio".to_string(),
                    href: "https://github.com/msierraltav/mitdevcat".to_string(),
                },
                Project {
                    lang: "rust".to_string(),
                    name: "ytm-cli".to_string(),
                    desc: "youtube music en terminal".to_string(),
                    href: "https://github.com/msierraltav".to_string(),
                },
                Project {
                    lang: "rust".to_string(),
                    name: "pi-mood-gif".to_string(),
                    desc: "animos en la barra lateral".to_string(),
                    href: "https://github.com/msierraltav".to_string(),
                },
            ],
            languages: vec![
                Link {
                    label: "español".to_string(),
                    href: "/es/".to_string(),
                },
                Link {
                    label: "english".to_string(),
                    href: "/en/".to_string(),
                },
                Link {
                    label: "català".to_string(),
                    href: "/ca/".to_string(),
                },
            ],
            copyright: "© 2025 MitDevcat".to_string(),
        }
    }
}
