use dioxus::prelude::*;

use crate::config::data::GLOBAL_DATA;

const CONFIRMATION_CSS: Asset = asset!("/assets/styling/views/confirmation.css");

#[component]
pub fn Confirmation() -> Element {
    let prematch_data =
        GLOBAL_DATA.with(|data| data.get_phase_data("prematch").cloned().unwrap_or_default());
    let auton_data =
        GLOBAL_DATA.with(|data| data.get_phase_data("auton").cloned().unwrap_or_default());
    let teleop_data =
        GLOBAL_DATA.with(|data| data.get_phase_data("teleop").cloned().unwrap_or_default());
    let postmatch_data = GLOBAL_DATA.with(|data| {
        data.get_phase_data("postmatch")
            .cloned()
            .unwrap_or_default()
    });

    let render_phase_data =
        |data: &indexmap::IndexMap<String, indexmap::IndexMap<String, serde_json::Value>>| {
            if data.is_empty() {
                rsx! {
                    p { class: "no-data", "No data entered for this phase." }
                }
            } else {
                rsx! {
                    for (category, items) in data.iter() {
                        div { class: "category-section",
                            div { class: "category-header",
                                h2 { class: "category-title", "{category}" }
                            }
                            div { class: "counters-list",
                                for (item, value) in items.iter() {
                                    div { class: "item-row",
                                        span { class: "item-name", "{item}" }
                                        span { class: "item-value", "{value}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

    rsx! {
        document::Link { rel: "stylesheet", href: CONFIRMATION_CSS }

        div { class: "confirmation-container",
            div { class: "top-nav",
               Link { class: "nav-btn", to: "/pages/postmatch",
                    span { "Postmatch" }
                    i { class: "arrow right" }
               }
            }

            div { class: "header-section",
                div { class: "main-header",
                    h1 { class: "title", "Confirmation" }
                }
            }

            div { class: "content-area",
                h2 { "Review Your Data" }
                p { "Please review the data you have entered for each phase before submission." }

                // Prematch Data
                div { class: "phase-summary",
                    h3 {
                        span { class: "phase-icon", }
                        "Prematch Data"
                    }
                    {render_phase_data(&prematch_data)}
                }

                // Auton Data
                div { class: "phase-summary",
                    h3 {
                        span { class: "phase-icon", }
                        "Auton Data"
                    }
                    {render_phase_data(&auton_data)}
                }

                // Teleop Data
                div { class: "phase-summary",
                    h3 {
                        span { class: "phase-icon", }
                        "Teleop Data"
                    }
                    {render_phase_data(&teleop_data)}
                }

                // Postmatch Data
                div { class: "phase-summary",
                    h3 {
                        span { class: "phase-icon", }
                        "Postmatch Data"
                    }
                    {render_phase_data(&postmatch_data)}
                }

                // Bottom right QR button
                div { class: "bottom-nav",
                    Link { class: "nav-btn", to: "/pages/qr",
                        span { "QR Code" }
                        i { class: "arrow right" }
                    }
                }
            }
        }
    }
}
