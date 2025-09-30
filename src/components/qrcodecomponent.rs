use dioxus::prelude::*;
use qrcode::{render::svg, QrCode};

#[component]
pub fn QrCodeComponent(json_data: String) -> Element {
    let mut error_message = use_signal(|| None);
    let mut qr_svg = use_signal(|| String::new());

    let json_data_clone = json_data.clone();
    use_effect(move || {
        // Validate that the input is valid JSON
        if let Err(e) = serde_json::from_str::<serde_json::Value>(&json_data_clone) {
            error_message.set(Some(format!("Invalid JSON: {}", e)));
            qr_svg.set(String::new());
            return;
        }

        error_message.set(None);

        // Generate QR code
        match QrCode::new(json_data_clone.as_bytes()) {
            Ok(code) => {
                let svg = code
                    .render()
                    .min_dimensions(400, 400)
                    .dark_color(svg::Color("#000000"))
                    .light_color(svg::Color("#ffffff"))
                    .build();
                qr_svg.set(svg);
            }
            Err(e) => {
                error_message.set(Some(format!("QR generation error: {}", e)));
            }
        }
    });

    rsx! {
        div {
            class: "qr-container",

            // Error message display
            if let Some(error) = error_message.read().as_ref() {
                div {
                    class: "error",
                    style: "color: red; margin-bottom: 10px;",
                    "Error: {error}"
                }
            }

            // QR code display
            if !qr_svg.read().is_empty() {
                div {
                    class: "qr-code",
                    dangerous_inner_html: "{qr_svg}"
                }
            } else if error_message.read().is_none() {
                div { "Generating QR code..." }
            }
        }
    }
}
