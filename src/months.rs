use chrono::Month;
use dioxus::prelude::*;

// Assuming your CSS will be in /assets/months.css

fn get_month_name(month_number: u32) -> String {
    Month::try_from(month_number as u8)
        .ok()
        .map(|month| month.name().to_string())
        .unwrap_or_else(|| format!("M{}", month_number)) // Fallback if name fails
}

#[component]
pub fn Months(mut selected_month_signal: Signal<i32>) -> Element {
    rsx! {
        // Outermost container, can be used for centering the whole block if needed
        div {
            id: "month_banner", // This ID can be used for overall positioning/background

            // This div will be our flex container for the buttons
            div {
                class: "month-buttons-container", // New class for flex styling
                for month_num in 1..=12 { // Iterate 1 to 12
                    button {
                        key: "month-{month_num}", // Good practice for lists
                        class: "month-button",    // Class for individual button styling
                        // Optionally, add a "selected" class if the month matches the signal
                        // class: if selected_month_signal() == month_num as i32 { "month-button selected" } else { "month-button" },
                        onclick: move |_| {
                            selected_month_signal.set(month_num as i32);
                        },
                        "{get_month_name(month_num)}"
                    }
                }
            }
        }
    }
}