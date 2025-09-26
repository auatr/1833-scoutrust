use crate::config::data::{self, GLOBAL_DATA};
use dioxus::prelude::*;
use serde_json::Value;

const PREMATCH_CSS: Asset = asset!("/assets/styling/views/prematch.css");

#[component]
pub fn Prematch() -> Element {
    let mut off_data = use_signal(|| data::initialize_data());
    let offdata_init = off_data.read().clone();

    let mut team_number = use_signal(|| {
        offdata_init
            .get("prematch", "Match Info", "TN")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    });

    let mut match_number = use_signal(|| {
        offdata_init
            .get("prematch", "Match Info", "MN")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    });

    let is_form_valid = !team_number.read().is_empty() && !match_number.read().is_empty();
    let mut is_submitted = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: PREMATCH_CSS }

        div { class: "prematch-container",
            div { class: "prematch-header",
                p { class: "prematch-subtitle", "Enter match information to begin" }
            }

            div { class: "input-group",
                div { class: "input-field",
                    label { class: "input-label", "Team Number" }
                    input {
                        class: "input-text",
                        value: "{team_number}",
                        placeholder: "Enter team number",
                        oninput: move |evt| {
                            let mut new_data = off_data();
                            new_data.add("prematch", "Match Info", "TN", Value::String(evt.value().clone()));
                            off_data.set(new_data);
                            team_number.set(evt.value().clone());
                        },
                        r#type: "number"
                    }
                }

                div { class: "input-field",
                    label { class: "input-label", "Match Number" }
                    input {
                        class: "input-text",
                        value: "{match_number}",
                        placeholder: "Enter match number",
                        oninput: move |evt| {
                            let mut new_data = off_data();
                            new_data.add("prematch", "Match Info", "MN", Value::String(evt.value().clone()));
                            off_data.set(new_data);
                            match_number.set(evt.value().clone());
                        },
                        r#type: "number"
                    }
                }
            }

            div { class: "button-group",
                button {
                    class: if is_form_valid { "btn-primary" } else { "btn-primary btn-disabled" },
                    disabled: !is_form_valid,
                    onclick: move |_| {
                        let current_data = off_data();

                        if let Some(team_value) = current_data.get("prematch", "Match Info", "TN") {
                            GLOBAL_DATA.with_mut(|data| {
                                data.add("prematch", "Match Info", "TN", team_value.clone());
                            });
                        }

                        if let Some(match_value) = current_data.get("prematch", "Match Info", "MN") {
                            GLOBAL_DATA.with_mut(|data| {
                                data.add("prematch", "Match Info", "MN", match_value.clone());
                            });
                        }

                        GLOBAL_DATA.with(|data| {
                            data.print_phase("prematch");
                        });

                        is_submitted.set(true);
                    },
                    "Submit"
                }

                if is_form_valid && is_submitted.read().to_owned() {
                    Link {
                        class: "btn-secondary",
                        to: "/pages/auton",
                        "Auton"
                    }
                } else {
                    button {
                        class: "btn-secondary btn-disabled",
                        disabled: true,
                        "Auton"
                    }
                }
            }

            if is_form_valid {
                div { class: "validation-message",
                    "Ready To Start Match"
                }
            }
        }
    }
}
