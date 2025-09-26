use dioxus::prelude::*;

use views::{Auton, Home, Postmatch, Prematch, Teleop};

mod components;
mod config;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home,

    #[route("/pages/prematch")]
    Prematch,
    
    #[route("/pages/auton")]
    Auton,

    #[route("/pages/teleop")]
    Teleop,

    #[route("/pages/postmatch")]
    Postmatch,
}

const FAVICON: Asset = asset!("/assets/images/BEAN.png");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
