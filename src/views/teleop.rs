use crate::components::Counter;
use crate::config::data::GLOBAL_DATA;
use dioxus::prelude::*;
use serde_json::{Number, Value};
const TELEOP_CSS: Asset = asset!("/assets/styling/teleop.css");

#[component]
pub fn Teleop() -> Element {
    let teleop_data =
        GLOBAL_DATA.with(|data| data.get_phase_data("teleop").cloned().unwrap_or_default());

    let mut local_teleop_data = use_signal(|| teleop_data.clone());

    let mut is_submitted = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: TELEOP_CSS }

        div { class: "teleop-container",
            div { class: "header-section",
                div { class: "main-header",
                    h1 { class: "title", "Teleop" }
                }

                div { class: "navigation",
                    Link { class: "nav-btn prev", to: "/pages/auton",
                        i { class: "arrow left" }
                        span { "Auton" }
                    }
                }
            }

            div { class: "content-area",
                if local_teleop_data.read().is_empty() {
                    div { class: "empty-state",
                        i { class: "icon data" }
                        p { "No teleop data categories configured" }
                    }
                } else {
                    div { class: "counters-column",
                        for (category, items) in local_teleop_data.read().iter() {
                            div { class: "category-section",
                                div { class: "category-header",
                                    h2 { class: "category-title", "{category}" }
                                }
                                div { class: "counters-list",
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
                }
            }

            div { class: "action-bar",
                button {
                    class: "submit-btn primary",
                    onclick: move |_| {
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

                        is_submitted.set(true);
                    },
                    i { class: "icon check" }
                    span { "Submit Teleop Data" }
                }

                if is_submitted.read().to_owned() {
                    Link { class: "nav-btn next enabled", to: "/pages/postmatch",
                        span { "Postmatch" }
                        i { class: "arrow right" }
                    }
                } else {
                    button {
                        class: "nav-btn next disabled",
                        disabled: true,
                        span { "Postmatch" }
                        i { class: "arrow right" }
                    }
                }
            }
        }
    }
}
