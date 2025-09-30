use dioxus::prelude::*;
use views::{Auton, Confirmation, Home, Postmatch, Prematch, Teleop};

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

    #[route("/pages/qr")]
    Qr,
    #[route("/pages/confirmation")]
    Confirmation,
}

const FAVICON: Asset = asset!("/assets/images/BEAN.png");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const CHEWY_FONT: Asset = asset!("/assets/fonts/Chewy-Regular.ttf");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // Preload the font using a link tag with preload
        document::Link {
            rel: "preload",
            href: CHEWY_FONT,
            r#as: "font",
            crossorigin: "anonymous"
        }

        Router::<Route> {}
    }
}
