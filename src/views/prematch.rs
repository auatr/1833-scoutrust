use dioxus::prelude::*;

use crate::config::data::GLOBAL_DATA;
use serde_json::Value;

#[derive(Clone, Default)]
struct OffGameData {
    team_number: String,
    match_number: String,
}

#[component]
pub fn Prematch() -> Element {
    let mut off_game_data = use_signal(|| OffGameData::default());

    let team_number = off_game_data().team_number.clone();
    let match_number = off_game_data().match_number.clone();

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
                            let mut data = off_game_data().clone();
                            data.team_number = evt.value().clone();
                            off_game_data.set(data);
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
                            let mut data = off_game_data().clone();
                            data.match_number = evt.value().clone();
                            off_game_data.set(data);
                        },
                        r#type: "number"
                    }
                }
            }

            if !team_number.is_empty() && !match_number.is_empty() {
                div { class: "button-container",
                    button {
                        class: "subtitle-block",
                        onclick: move |_| {
                            let current_data = off_game_data();

                            GLOBAL_DATA.lock().unwrap().add(
                                "prematch",
                                "Match Info",
                                "team_number",
                                Value::String(current_data.team_number.clone()),
                            );
                            GLOBAL_DATA.lock().unwrap().add(
                                "prematch",
                                "Match Info",
                                "match_number",
                                Value::String(current_data.match_number.clone()),
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
