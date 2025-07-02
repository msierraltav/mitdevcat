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
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```


## Resources

- Icons (Font awesomne)[https://github.com/FortAwesome/Font-Awesome]

## Deploy 

( deploy en github plages)[https://dioxuslabs.com/learn/0.6/cookbook/publishing/]

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