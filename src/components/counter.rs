use dioxus::prelude::*;

const COUNTER_CSS: Asset = asset!("/assets/styling/counter.css");

#[component]
pub fn Counter(count: Signal<i32>, title: String) -> Element {
    rsx! {
        p { title }
        button { onclick: move |_| count -= 1, "-" }
        p { "{count()}" }
        button { onclick: move |_| count += 1, "+" }
    }
}