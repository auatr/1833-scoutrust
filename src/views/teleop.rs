use crate::components::Counter;
use crate::config::data::{self, GLOBAL_DATA};
use dioxus::prelude::*;
use serde_json::{Number, Value};
use std::collections::HashMap;

const TELEOP_CSS: Asset = asset!("/assets/styling/teleop.css");

#[component]
pub fn Teleop() -> Element {
    let teleop_data =
        GLOBAL_DATA.with(|data| data.get_phase_data("teleop").cloned().unwrap_or_default());

    // Create local state by cloning the global teleop data
    let mut local_teleop_data = use_signal(|| teleop_data.clone());

    rsx! {
        document::Link { rel: "stylesheet", href: TELEOP_CSS }

        div { class: "container",
            div { class: "header",
                p { class: "title", "Teleop Scouting" }
            }

            if local_teleop_data.read().is_empty() {
                div { class: "empty-state",
                    "No teleop data categories configured"
                }
            } else {
                for (category, items) in local_teleop_data.read().iter() {
                    div { class: "counters-section",
                        h2 { "{category}" }
                        div { class: "counter-grid",
                            {items.iter().map(|(item, value)| {
                                let item_clone = item.clone();
                                let category_clone = category.clone();
                                let initial_count = value.as_i64().unwrap_or(0) as i32;

                                rsx! {
                                    div { class: "counter-item",
                                        Counter {
                                            count: use_signal(|| initial_count),
                                            title: item.clone(),
                                            on_change: move |new_count| {
                                                local_teleop_data.with_mut(|data| {
                                                    if let Some(category_data) = data.get_mut(&category_clone) {
                                                        if let Some(value) = category_data.get_mut(&item_clone) {
                                                            *value = Value::Number(Number::from(new_count));
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }
            }

            div { class: "submit-section",
                button {
                    class: "submit-button",
                    onclick: move |_| {
                        // print local data for debugging
                        println!("Local Teleop Data: {:?}", local_teleop_data.read());


                        // Update global data with local data
                        GLOBAL_DATA.with_mut(|global_data| {
                            let local_data = local_teleop_data.read();
                            for (category, items) in local_data.iter() {
                                for (item, value) in items {
                                    let count = value.as_i64().unwrap_or(0) as i32;
                                    global_data.add("teleop", category, item, Value::Number(Number::from(count)));
                                }
                            }
                        });

                        // Print the updated data
                        GLOBAL_DATA.with(|data| {
                            data.print_phase("prematch");
                        });

                        GLOBAL_DATA.with(|data| {
                            data.print_phase("teleop");
                        });
                    },
                    "Submit Teleop Data"
                }
            }
        }
    }
}
