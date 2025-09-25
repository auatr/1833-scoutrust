use dioxus::prelude::*;

const COUNTER_CSS: Asset = asset!("/assets/styling/counter.css");

#[component]
pub fn Counter(count: Signal<i32>, title: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: COUNTER_CSS }

        div { class: "container",
            div { class: "label",
                p { class: "labelText", "{title}" }
            }
            button { class: "control-btn", onclick: move |_| count -= 1,
                span { class: "control-text", "-" }
            }
            div { class: "counter",
                p { class: "counter-text", "{count()}" }
            }
            button { class: "control-btn", onclick: move |_| count += 1,
                span { class: "control-text", "+" }
            }
        }
    }
}
