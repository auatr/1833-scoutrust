use dioxus::prelude::*;

const COUNTER_CSS: Asset = asset!("/assets/styling/counter.css");

#[component]
pub fn Counter(count: Signal<i32>, title: String, on_change: Option<EventHandler<i32>>) -> Element {
    rsx! {
    document::Link { rel: "stylesheet", href: COUNTER_CSS }

    div { class: "container",
        div { class: "inner",
            div { class: "label",
                p { class: "labelText", "{title}" }
            }
                button { class: "control-btn",
                    onclick: move |_| {
                        let new_count = count() - 1;
                        count.set(new_count);
                        if let Some(handler) = &on_change {
                            handler.call(new_count);
                        }
                    },
                    span { class: "control-text", "-" }
                }
            }
            div { class: "counter",
                p { class: "counter-text", "{count()}" }
            }
                button { class: "control-btn",
                    onclick: move |_| {
                        let new_count = count() + 1;
                        count.set(new_count);
                        if let Some(handler) = &on_change {
                            handler.call(new_count);
                        }
                    },
                    span { class: "control-text", "+" }
                }
            }
        }
}
