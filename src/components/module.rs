use crate::components::{Counter, Input, Toggle};
use crate::config::data::ConfigEntry;
use dioxus::prelude::*;
use serde_json::{Number, Value};

#[component]
pub fn Module(
    category: String,
    item: String,
    value: Value,
    config_entries: Vec<ConfigEntry>,
    on_change: EventHandler<(String, String, Value)>,
) -> Element {
    // Find the item type from the config
    let item_type = config_entries.iter().find_map(|entry| {
        if entry.title == category {
            entry
                .items
                .iter()
                .find(|config_item| config_item.key == item)
                .map(|config_item| config_item.item_type.to_lowercase())
        } else {
            None
        }
    });

    let category_counter = category.clone();
    let item_counter = item.clone();
    let on_change_counter = on_change.clone();
    let handle_counter_change = move |new_count: i32| {
        on_change_counter.call((
            category_counter.clone(),
            item_counter.clone(),
            Value::Number(Number::from(new_count)),
        ));
    };

    let category_toggle = category.clone();
    let item_toggle = item.clone();
    let on_change_toggle = on_change.clone();
    let handle_toggle_change = move |new_state: bool| {
        on_change_toggle.call((
            category_toggle.clone(),
            item_toggle.clone(),
            Value::Bool(new_state),
        ));
    };

    let category_text = category.clone();
    let item_text = item.clone();
    let on_change_text = on_change.clone();
    let handle_text_change = move |new_text: String| {
        on_change_text.call((
            category_text.clone(),
            item_text.clone(),
            Value::String(new_text),
        ));
    };

    match item_type.as_deref() {
        Some("number") => {
            let initial_count = value.as_i64().unwrap_or(0) as i32;
            rsx! {
                div { class: "counter-item",
                    Counter {
                        count: use_signal(|| initial_count),
                        title: item.clone(),
                        on_change: handle_counter_change,
                    }
                }
            }
        }
        Some("boolean") => {
            let initial_state = value.as_bool().unwrap_or(false);
            rsx! {
                div { class: "counter-item",
                    Toggle {
                        is_toggled: use_signal(|| initial_state),
                        title: item.clone(),
                        on_change: handle_toggle_change,
                    }
                }
            }
        }
        Some("text-input") | Some("string") => {
            let initial_text = value.as_str().unwrap_or("").to_string();
            rsx! {
                div { class: "counter-item",
                    Input {
                        input_type: "text",
                        text: use_signal(|| initial_text),
                        title: item.clone(),
                        on_change: handle_text_change,
                    }
                }
            }
        }
        Some("int-input") => {
            let initial_text = value.as_str().unwrap_or("").to_string();
            rsx! {
                div { class: "counter-item",
                    Input {
                        input_type: "number",
                        text: use_signal(|| initial_text),
                        title: item.clone(),
                        on_change: handle_text_change,
                    }
                }
            }
        }
        _ => {
            rsx! {
                div { class: "counter-item error",
                    p { "Unknown input type for: {item}" }
                }
            }
        }
    }
}
