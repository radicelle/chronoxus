use chrono::Month;
use chrono::Utc;
use dioxus::prelude::*;

const MONTH_CSS: Asset = asset!("/assets/months.css");

fn get_month_name(month_number: u32) -> String {
    Month::try_from(month_number as u8)
        .ok()
        .map(|month| month.name().to_string())
        .expect("unfounded")
}

#[component]
pub fn Months(mut selected_month_signal: Signal<i32>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MONTH_CSS }
        div {
            id: "month_banner",
            table {
                 tbody {
                    tr {
                        for month in 1..13 {
                            div {
                                id: month,
                                class: "month",
                                button {
                                    onclick: move |_| *selected_month_signal.write() = month as i32,
                                    "{get_month_name(month)}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
