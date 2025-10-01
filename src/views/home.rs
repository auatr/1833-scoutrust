use dioxus::prelude::*;

use crate::config::data::GLOBAL_DATA;

const HOME_CSS: Asset = asset!("/assets/styling/views/home.css");

#[component]
pub fn Home() -> Element {
    GLOBAL_DATA.with_mut(|data| data.reset());
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div { class: "container",
            div { class: "sub-container",
                Link { class: "button", to: "/pages/prematch",
                    "START SCOUTING"
                }
            }
            // div { class: "sub-container",
            //     Link { class: "button", to: "/pages/prevmatches", // doesn't exist yet
            //         "See Previous Matches"
            //     }
            // }
        }
    }
}
