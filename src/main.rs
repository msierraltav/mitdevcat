mod components;
use components::header::Header;
use components::hero::Hero;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_CSS: Asset = asset!("/assets/styles/header.css");
const ANIMATION_CSS: Asset = asset!("/assets/styles/animations.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const GITHUB_SVG: Asset = asset!("/assets/github.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: HEADER_CSS }
        document::Link { rel: "stylesheet", href: ANIMATION_CSS }
        Hero {}

        div{
            class:"content-container",
            Header {}
            GithubCard {}
        }
    }
}

#[component]
pub fn GithubCard() -> Element {
    rsx! {
      div {
        id: "github-card",
        a {
          href: "http://www.github.com/mitdevcat",
          p { "Github" }
          img {
            src: GITHUB_SVG,
            id: "github-logo",
          }
        }
      }
    }
}
