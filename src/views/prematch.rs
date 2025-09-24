use dioxus::prelude::*;

use crate::config::data::{self, GLOBAL_DATA};
use serde_json::Value;

#[component]
pub fn Prematch() -> Element {
    let mut off_data = use_signal(|| data::initialize_data());
    
    let offdata_init = off_data.read().clone();
    
    let mut team_number = use_signal(|| offdata_init.get("prematch", "Match ID", "TN").unwrap().clone().to_string());
    let mut match_number = use_signal(|| offdata_init.get("prematch", "Match ID", "TN").unwrap().clone().to_string());

    rsx! {
        div { class: "container",
            div { class: "row-container",
                div { class: "input-container",
                    div { class: "subtitle-block",
                        p { class: "subtitle-block-text", "TEAM" }
                    }
                    input {
                        class: "input",
                        value: "{team_number}",
                        placeholder: "Team Number",
                        oninput: move |evt| {
                            let mut new_data = off_data();
                            new_data.add("prematch", "Match ID", "TN", Value::String(evt.value().clone()));
                            off_data.set(new_data);
                            team_number.set(evt.value().clone());
                        },
                        r#type: "number"
                    }
                }

                div { class: "input-container",
                    div { class: "subtitle-block",
                        p { class: "subtitle-block-text", "MATCH" }
                    }
                    input {
                        class: "input",
                        value: "{match_number}",
                        placeholder: "Match Number",
                        oninput: move |evt| {
                            let mut new_data = off_data();
                            new_data.add("prematch", "Match ID", "MN", Value::String(evt.value().clone()));
                            off_data.set(new_data);
                            match_number.set(evt.value().clone());
                        },
                        r#type: "number"
                    }
                }
            }

            if !team_number.read().is_empty() && !match_number.read().is_empty() {
                div { class: "button-container",
                    button {
                        class: "subtitle-block",
                        onclick: move |_| {
                            let current_data = off_data();

                            GLOBAL_DATA.lock().unwrap().add(
                                "prematch",
                                "Match Info",
                                "team_number",
                                Value::String(current_data.get("prematch", "Match ID", "TN").unwrap().to_string()),
                            );
                            GLOBAL_DATA.lock().unwrap().add(
                                "prematch",
                                "Match Info",
                                "match_number",
                                Value::String(current_data.get("prematch", "Match ID", "TN").unwrap().to_string()),
                            );
                        },
                        "Submit Data"
                    }

                    Link {
                        class: "subtitle-block",
                        to: "/pages/auton",
                        "Auton"
                    }
                }
            }
        }
    }
}
