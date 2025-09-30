use crate::{components::QrCodeComponent, config::data::GLOBAL_DATA};
use dioxus::prelude::*;

const QR_CSS: Asset = asset!("/assets/styling/views/qr.css");

#[component]
pub fn Qr() -> Element {
    rsx! {
       document::Link { rel: "stylesheet", href: QR_CSS }

       div {
           class: "qr-view",
              Link { class: "qr-back-button", to: "/pages/confirmation",
                   "Confirmation"
              }
              div {
                   class: "qr-content-container",

                   h1 {
                        class: "qr-title",
                        "QR Code"
                   }

                   QrCodeComponent {
                        json_data: GLOBAL_DATA.with(|data| data.convert_all_to_json())
                   }

                   Link { class: "qr-home-button", to: "/",
                        "Home"
                   }
              }
         }
    }
}
