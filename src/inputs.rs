use chrono::{Datelike, Month};
use chrono::Utc;
use dioxus::prelude::*;

const INPUTS_CSS: Asset = asset!("/assets/inputs.css");

fn get_number_of_days(month_number: i32) -> u8 {
    Month::try_from(month_number as u8)
        .ok()
        .map(|month| month.num_days(Utc::now().year()).expect("year not found"))
        .expect("can't get number of days")
}

#[component]
pub fn Inputs(selected_month: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: INPUTS_CSS }
        div {
            id: "daily_input",
            table {
                 tbody {
                    tr {
                        for day in 1 .. get_number_of_days(selected_month) {
                            th { id: "day", class: "day", th { "day" } }

                        }
                    }
                    tr {
                        for day in 1 .. get_number_of_days(selected_month) {
                            th { id: "day", class: "day", th { "day" } }
                        }
                    }
                }
            }
        }
    }
}
