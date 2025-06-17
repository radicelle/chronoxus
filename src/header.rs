use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut show_text_input = use_signal(|| false);
    
    rsx! {
        div {
            id: "header",
            div { class: "header-content",
                div { class: "logo", "chronoxus" }
                div {
                    class: "settings-icon",
                    onclick: move |_| show_text_input.set(true),
                    "⚙️" // Gear emoji as a placeholder, can be replaced with an actual icon
                }
            }
            
            // Large text input window that appears when the gear icon is clicked
            div {
                class: if show_text_input() { "text-input-window show" } else { "text-input-window" },
                div { class: "text-input-container",
                    div { class: "text-input-header",
                        h3 { "Settings" }
                        button {
                            class: "close-button",
                            onclick: move |_| show_text_input.set(false),
                            "×" // Close button
                        }
                    }
                    textarea {
                        class: "large-text-input",
                        placeholder: "Enter your text here..."
                    }
                }
            }
        }
    }
}