use dioxus::prelude::*;

use crate::config::data::{self, GLOBAL_DATA};
use serde_json::Value;

#[component]
pub fn Prematch() -> Element {
    let mut off_data = use_signal(|| data::initialize_data());
    
    let off_data_read = off_data.read(); // Need to make sure this and below values are updated on input
    
    let team_number = off_data_read.get("prematch", "Match ID", "TN").unwrap();
    let match_number = off_data_read.get("prematch", "Match ID", "MN").unwrap();

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
                        },
                        r#type: "number"
                    }
                }
            }

            if !team_number.to_string().is_empty() && !match_number.to_string().is_empty() {
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
