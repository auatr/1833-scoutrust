use dioxus::prelude::*;

const DROPDOWN_CSS: Asset = asset!("/assets/styling/components/dropdown.css");

#[component]
pub fn Dropdown(
    value: Signal<String>,
    title: String,
    options: Vec<String>,
    placeholder: Option<String>,
    on_change: Option<EventHandler<String>>,
) -> Element {
    rsx! {
          document::Link { rel: "stylesheet", href: DROPDOWN_CSS }

          div { class: "dropdown-container",
               p { class: "dropdown-label", "{title}" }

               select {
                    class: "dropdown-select large full-width",
                    value: "{value()}",
                    onchange: move |event| {
                         let new_value = event.value();
                         value.set(new_value.clone());
                         if let Some(handler) = &on_change {
                              handler.call(new_value);
                         }
                    },

                    if let Some(placeholder_text) = placeholder {
                         option {
                         value: "",
                         disabled: true,
                         selected: value().is_empty(),
                         "{placeholder_text}"
                         }
                    }

                    for option in options.iter() {
                         option {
                         value: "{option}",
                         selected: option == &value(),
                         "{option}"
                         }
                    }
               }
          }
    }
}
