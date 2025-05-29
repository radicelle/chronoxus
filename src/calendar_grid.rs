#![allow(non_snake_case)] // To allow component names like App
use dioxus::prelude::*;
use chrono::{Datelike, Month, NaiveDate};
use num_traits::cast::FromPrimitive; // For Month::from_i32

// Assume this is in your assets directory

fn get_day_name(year: i32, month_number: i32, day_number: u32) -> String {
    NaiveDate::from_ymd_opt(year, month_number as u32, day_number)
        .map(|date| date.weekday().to_string()[..3].to_string()) // Short day name e.g. "Mon"
        .unwrap_or_else(|| "Err".to_string())
}

fn days_in_month(year: i32, month: i32) -> u32 {
    if month == 2 {
        if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
            29 // Leap year
        } else {
            28
        }
    } else if [4, 6, 9, 11].contains(&month) {
        30
    } else {
        31
    }
}

#[component]
pub fn CalendarGrid(year: i32, month: i32) -> Element {
    let num_days = days_in_month(year, month);
    let month_name = Month::from_i32(month).map_or("Invalid Month".to_string(), |m| m.name().to_string());

    rsx! {
        div {
            h2 { style: "text-align:center; margin-bottom: 15px; color: #555;",
                "{month_name} {year}"
            }
            div { class: "calendar-grid-container",
                for day_num in 1..=num_days {
                    {
                        let day_str = day_num.to_string();
                        let day_name_str = get_day_name(year, month, day_num);
                        let input_id = format!("day-{}-{}-{}", year, month, day_num);

                        rsx! {
                            div {
                                key: "{input_id}-item", // Unique key for the item
                                class: "calendar-item",
                                div { class: "day-header", "{day_name_str} {day_str}" }
                                input {
                                    id: "{input_id}",
                                    class: "day-input",
                                    r#type: "text", // Use r#type for "type"
                                    placeholder: "", // Placeholder for the input
                                    "aria-label": "Event for {day_name_str} {day_str}, {month_name} {year}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
