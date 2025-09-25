use dioxus::prelude::*;

const TOGGLE_CSS: Asset = asset!("/assets/styling/toggle.css");

#[component]
pub fn Toggle(
    is_toggled: Signal<bool>,
    title: String,
    on_change: Option<EventHandler<bool>>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TOGGLE_CSS }

        div { class: "toggle-container",
            p { class: "toggle-label", "{title}" }

            label { class: "toggle-switch large",
                input {
                    class: "toggle-input",
                    r#type: "checkbox",
                    checked: is_toggled(),
                    onchange: move |event| {
                        let new_state = event.value().parse().unwrap_or(false);
                        is_toggled.set(new_state);
                        if let Some(handler) = &on_change {
                            handler.call(new_state);
                        }
                    }
                }
                span { class: "toggle-slider" }
            }
        }
    }
}
