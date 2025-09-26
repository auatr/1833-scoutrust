use dioxus::prelude::*;

const TEXT_INPUT_CSS: Asset = asset!("/assets/styling/input.css");

#[component]

pub fn Input(
    input_type: String,
    text: Signal<String>,
    title: String,
    on_change: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TEXT_INPUT_CSS }
          div { class: "input-container",
               p { class: "input-label", "{title}" }

               input {
                    class: "input-field large full-width",
                    r#type: "{input_type}",
                    value: "{text}",
                    placeholder: "",
                    oninput: move |event| {
                         let new_value = event.value();
                         text.set(new_value.clone());
                         if let Some(handler) = &on_change {
                              handler.call(new_value);
                         }
                    }
               }
          }
    }
}
