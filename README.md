# Mitdevcat

why I hate myself doing this portafolio in rust ?

- "Writen for humans by a human"

## Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve            # dev server with hot-reload -> http://localhost:8080
dx build --platform web   # production-style build check
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

### Raspberry Pi notes (4 GB RAM, aarch64)

This project is developed on a Raspberry Pi 4. Some hard-won rules:

- **Never `cargo install dioxus-cli` on the Pi.** Install the prebuilt `dx`
  binary instead:
  ```bash
  curl -fsSL https://dioxus.dev/install.sh | bash
  ```
- `.cargo/config.toml` pins `jobs = 2` so `rustc` doesn't exhaust the 4 GB of RAM
  (full-parallel builds OOM here). Builds take ~4 min wasm / ~8 min full dx build.
- `/tmp` is a 1.9 GB tmpfs: `cargo install` builds there and dies with
  `No space left on device` even when the SD card has plenty of room.
- Wasm target must be installed: `rustup target add wasm32-unknown-unknown`.

Stack: Dioxus 0.7 + dioxus-i18n 0.5 (migrated from 0.6, dx 0.7 requires 0.7).


## Resources

- Icons (Font awesomne)[https://github.com/FortAwesome/Font-Awesome]

## Deploy 

( deploy en github pages)[https://dioxuslabs.com/learn/0.7/cookbook/publishing/]

```bash
# build 
dx bundle --out-dir docs

# move the static content from docs/public to docs/
mv docs/public/* docs

# make a copy of docs/index.html and rename to docs/404.html
cp docs/index.html docs/404.html

# add and commit with git

# push to github
```

## TODO

- [ ] first version
- [ ] Use github actions to build the site and run it using github pages.
- [ ] Separate components in files and folders.