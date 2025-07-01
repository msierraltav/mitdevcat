# MitDevcat - Internationalization Guide

This project uses `dioxus-i18n` for internationalization support in English, Spanish, and Catalan.

## Setup

The internationalization is configured in `src/main.rs`:

```rust
use_init_i18n(|| {
    I18nConfig::new(langid!("en-US"))
        .with_locale((langid!("en-US"), include_str!("../i18n/en.ftl")))
        .with_locale((langid!("es-ES"), include_str!("../i18n/es.ftl")))  
        .with_locale((langid!("ca-ES"), include_str!("../i18n/ca.ftl")))
        .with_fallback(langid!("en-US"))
});
```

## Language Files

Translation files are located in the `i18n/` directory:

- `en.ftl` - English translations
- `es.ftl` - Spanish translations  
- `ca.ftl` - Catalan translations

## Translation Format

The project uses Fluent format for translations:

```fluent
# Comments
translation-key = Translation text
button-label = Click me
greeting = Hello, {$name}!
```

## Usage in Components

To use translations in your components:

```rust
use dioxus_i18n::t;

#[component]
fn MyComponent() -> Element {
    rsx! {
        h1 { {t!("hero-welcome")} }
        p { {t!("hero-description")} }
    }
}
```

## Language Switching

The language selector component allows users to switch between languages:

```rust
use dioxus_i18n::{prelude::*, unic_langid::langid};

let mut i18n = i18n();
let change_language = move |_| {
    i18n.set_language(langid!("es-ES"));
};
```

## Supported Languages

- **English (en-US)** - Default language
- **Spanish (es-ES)** - Complete translation
- **Catalan (ca-ES)** - Complete translation

## Features Demonstrated

1. **Language switching** - Users can switch between languages using buttons in the header
2. **Dynamic translations** - All text content updates immediately when language changes
3. **Fallback support** - Falls back to English if a translation is missing
4. **Component isolation** - Each component can use translations independently

## Adding New Languages

1. Create a new `.ftl` file in the `i18n/` directory
2. Add the locale configuration in `main.rs`
3. Update the language selector component
4. Add the language option to the header

## Example Usage

The main sections that demonstrate i18n features:

- **Header**: Navigation and language switcher
- **Hero**: Welcome message and description
- **About**: About section with call-to-action
- **Projects**: Projects showcase with buttons
- **Contact**: Contact information and buttons
- **Footer**: Copyright notice

## Troubleshooting

### dx serve Issues

If `dx serve` is running but assets aren't loading:

1. **404 Not Found**: Check that the `Dioxus.toml` configuration is correct
2. **Assets not loading**: Try cleaning the build cache:
   ```bash
   cargo clean
   dx serve --hot-reload
   ```
3. **Proc-macro errors**: Clean and rebuild:
   ```bash
   cargo clean
   cargo check
   ```

### Common Asset Issues

- **CSS not applying**: Check that CSS files are listed in `Dioxus.toml` under `[web.resource]`
- **Images not loading**: Verify asset paths use `/assets/` prefix
- **JavaScript not working**: Ensure JS files are included in the asset configuration

### Development vs Production

- Development: `dx serve --hot-reload`
- Production build: `dx build --release`
- Deploy: Files are built to the `docs/` directory for GitHub Pages
