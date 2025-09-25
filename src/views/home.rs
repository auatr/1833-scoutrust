use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { class: "container",
            div { class: "sub-container",
                Link { class: "button", to: "/pages/prematch",
                    "START SCOUTING"
                }
            }
            div { class: "sub-container",
                Link { class: "button", to: "/pages/prevmatches", // doesn't exist yet
                    "See Previous Matches"
                }
            }
        }
    }
}
