mod components;
use components::header::Header;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
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
        Header {}
        GithubCard {}
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

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            }
        }
    }
}
