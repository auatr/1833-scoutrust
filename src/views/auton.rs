use dioxus::prelude::*;

use crate::components::Counter;
use crate::config::data::GLOBAL_DATA;

const CSS: Asset = asset!("/assets/styling/auton.css");

#[component]
pub fn Auton() -> Element {
    let global_data = GLOBAL_DATA();
    let auton_config = global_data.get_phase_data("auton").unwrap();
    
    rsx! {
        document::Link {rel: "stylesheet", href: CSS} 
        
        div {
            div { class: "header", h1 { class: "header_text", "Auton" } }
            for (category, items) in auton_config.iter() {
                div { class: "section",
                    h2 { "{category}" }
                    for (item, value) in items.iter() {
                        if  value.is_number() { 
                            Counter {
                                count: use_signal(|| value.as_i64().unwrap_or(0) as i32),
                                title: item.clone()
                            } 
                        }
                        else if value.is_boolean() { // Need to make way to update boolean
                            button { "{item}" } // Currently returns item id
                        }
                    }
                } 
            } 
            button { "Submit" }
            link { "Teleop" }
        }
    }
}
