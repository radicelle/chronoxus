use chrono::{Datelike};
use chrono::{Utc};
use dioxus::prelude::*;
use crate::calendar_grid::CalendarGrid;

#[component]
pub fn Inputs(selected_month: i32) -> Element {
    rsx! {
        div {
            id: "daily_input",
            CalendarGrid { year: Utc::now().year(), month: selected_month }
        }
    }
}
