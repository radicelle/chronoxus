use chrono::{Datelike, Month};
use chrono::{Utc};
use dioxus::prelude::*;
use crate::calendar_grid::CalendarGrid;

fn get_number_of_days(month_number: i32) -> u8 {
    Month::try_from(month_number as u8)
        .ok()
        .map(|month| month.num_days(Utc::now().year()).expect("year not found"))
        .expect("can't get number of days")
}

#[component]
pub fn Inputs(selected_month: i32) -> Element {
    rsx! {
        div {
            id: "daily_input",
            CalendarGrid { year: Utc::now().year(), month: selected_month }
        }
    }
}
