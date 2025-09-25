use crate::{components::Counter, config::data::GLOBAL_DATA};
use dioxus::prelude::*;
use serde_json::{Number, Value};

const TELEOP_CSS: Asset = asset!("/assets/styling/teleop.css");

#[component]
pub fn Teleop() -> Element {
    let teleop_data =
        GLOBAL_DATA.with(|data| data.get_phase_data("teleop").cloned().unwrap_or_default());

    rsx! {
        document::Link { rel: "stylesheet", href: TELEOP_CSS }

        div { class: "container",
            div { class: "header",
                p { class: "title", "Teleop Scouting" }
                p { class: "subtitle", "Record teleoperated period data here" }
            }

            if teleop_data.is_empty() {
                div { class: "empty-state",
                    "No teleop data categories configured"
                }
            } else {
                for (category, items) in teleop_data.iter() {
                    div { class: "counters-section",
                        h2 { "{category}" }
                        div { class: "counter-grid",
                            for (item, value) in items.iter() {
                                div { class: "counter-item",
                                    Counter {
                                        count: use_signal(|| value.as_i64().unwrap_or(0) as i32),
                                        title: item.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "submit-section",
                button {
                    class: "submit-button",
                    onclick: move |_| {
                        GLOBAL_DATA.with_mut(|data| {
                            for (category, items) in &teleop_data {
                                for (item, value) in items {
                                    let count = value.as_i64().unwrap_or(0) as i32;
                                    data.add("teleop", category, item, Value::Number(Number::from(count)));
                                }
                            }

                            // GLOBAL_DATA.with(|data| {
                            //     data.print_phase("teleop");
                            // });
                        });
                    },
                    "Submit Teleop Data"
                }
            }
        }
    }
}
