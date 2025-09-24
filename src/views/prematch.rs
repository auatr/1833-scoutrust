use dioxus::prelude::*;

use crate::config::data::{initialize_match, GLOBAL_DATA};
use serde_json::Value;

#[component]
pub fn Prematch() -> Element {
    let mut off_match = use_signal(|| initialize_match());
    
    let offmatch_init = off_match.read().clone();
    
    let mut team_number = use_signal(|| offmatch_init.prematch.team_number);
    let mut match_number = use_signal(|| offmatch_init.prematch.match_number);

    rsx! {
        div { class: "container",
            div { class: "row-container",
                div { class: "input-container",
                    div { class: "subtitle-block",
                        p { class: "subtitle-block-text", "TEAM" }
                    }
                    input {
                        class: "input",
                        value: "{team_number.read()}",
                        placeholder: "Team Number",
                        oninput: move |evt| {
                            let mut new_data = off_match();
                            new_data.prematch.team_number = evt.value().clone().parse().unwrap();
                            off_match.set(new_data);
                            team_number.set(evt.value().clone().parse().unwrap_or_default());
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
                        value: "{match_number.read()}",
                        placeholder: "Match Number",
                        oninput: move |evt| {
                            let mut new_data = off_match();
                            new_data.prematch.match_number = evt.value().clone().parse().unwrap_or_default();
                            off_match.set(new_data);
                            match_number.set(evt.value().clone().parse().unwrap_or_default());
                        },
                        r#type: "number"
                    }
                }
            }

            if team_number.read().clone() != 9999 && match_number.read().clone() != 0 {
                div { class: "button-container",
                    button {
                        class: "subtitle-block",
                        onclick: move |_| {
                            GLOBAL_DATA.lock().unwrap().prematch.team_number = team_number.read().clone();
                            GLOBAL_DATA.lock().unwrap().prematch.match_number = match_number.read().clone();
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
