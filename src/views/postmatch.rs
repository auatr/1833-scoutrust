use crate::components::Module;
use crate::config::data::{GLOBAL_DATA, POSTMATCH_CONFIG};
use dioxus::prelude::*;
use serde_json::Value;
const POSTMATCH_CSS: Asset = asset!("/assets/styling/views/match.css");

#[component]
pub fn Postmatch() -> Element {
    let postmatch_data = GLOBAL_DATA.with(|data| {
        data.get_phase_data("postmatch")
            .cloned()
            .unwrap_or_default()
    });

    let mut local_postmatch_data = use_signal(|| postmatch_data.clone());
    let mut is_submitted = use_signal(|| false);

    let handle_input_change = move |(category, item, value): (String, String, Value)| {
        local_postmatch_data.with_mut(|data| {
            if let Some(category_data) = data.get_mut(&category) {
                category_data.insert(item, value);
            }
        });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: POSTMATCH_CSS }

        div { class: "match-container teleop-container",
            div { class: "header-section",
                div { class: "main-header",
                    h1 { class: "title", "Postmatch" }
                }

                div { class: "navigation",
                    Link { class: "nav-btn prev", to: "/pages/teleop",
                        i { class: "arrow left" }
                        span { "Teleop" }
                    }
                }
            }

            div { class: "content-area",
                if local_postmatch_data.read().is_empty() {
                    div { class: "empty-state",
                        i { class: "icon data" }
                        p { "No postmatch data categories configured" }
                    }
                } else {
                    div { class: "counters-column",
                        for (category, items) in local_postmatch_data.read().iter() {
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
                                            config_entries: POSTMATCH_CONFIG.to_vec(),
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
                            let local_data = local_postmatch_data.read();
                            for (category, items) in local_data.iter() {
                                for (item, value) in items {
                                     match value {
                                        Value::Number(num) => {
                                            global_data.add("postmatch", category, item, Value::Number(num.clone()));
                                        }
                                        Value::Bool(bool_val) => {
                                            global_data.add("postmatch", category, item, Value::Bool(*bool_val));
                                        }
                                        Value::String(str_val) => {
                                            global_data.add("postmatch", category, item, Value::String(str_val.clone()));
                                        }
                                        _ => {
                                        }
                                    }
                                }
                            }
                        });

                        // Print the updated data
                        GLOBAL_DATA.with(|data| {
                            data.print_phase("postmatch");
                        });

                        is_submitted.set(true);
                    },
                    i { class: "icon check" }
                    span { "Submit Postmatch Data" }
                }

                if is_submitted.read().to_owned() {
                    Link { class: "nav-btn next enabled", to: "/pages/confirmation",
                        span { "Confirmation" }
                        i { class: "arrow right" }
                    }
                } else {
                    button {
                        class: "nav-btn next disabled",
                        disabled: true,
                        span { "Confirmation" }
                        i { class: "arrow right" }
                    }
                }
            }
        }
    }
}
