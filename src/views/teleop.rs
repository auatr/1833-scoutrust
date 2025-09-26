use crate::components::Module;
use crate::config::data::{GLOBAL_DATA, TELEOP_CONFIG};
use dioxus::prelude::*;
use serde_json::Value;

const TELEOP_CSS: Asset = asset!("/assets/styling/views/match.css");

#[component]
pub fn Teleop() -> Element {
    let teleop_data =
        GLOBAL_DATA.with(|data| data.get_phase_data("teleop").cloned().unwrap_or_default());

    let mut local_teleop_data = use_signal(|| teleop_data.clone());
    let mut is_submitted = use_signal(|| false);

    let handle_input_change = move |(category, item, value): (String, String, Value)| {
        local_teleop_data.with_mut(|data| {
            if let Some(category_data) = data.get_mut(&category) {
                category_data.insert(item, value);
            }
        });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: TELEOP_CSS }

        div { class: "match-container teleop-container",
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
                                    for (item, value) in items.iter() {
                                        Module {
                                            category: category.clone(),
                                            item: item.clone(),
                                            value: value.clone(),
                                            config_entries: TELEOP_CONFIG.to_vec(),
                                            on_change: handle_input_change,
                                        }
                                    }
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
                                     match value {
                                        Value::Number(num) => {
                                            global_data.add("teleop", category, item, Value::Number(num.clone()));
                                        }
                                        Value::Bool(bool_val) => {
                                            global_data.add("teleop", category, item, Value::Bool(*bool_val));
                                        }
                                        Value::String(str_val) => {
                                            global_data.add("teleop", category, item, Value::String(str_val.clone()));
                                        }
                                        _ => {
                                        }
                                    }
                                }
                            }
                        });

                        // Print the updated data
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
