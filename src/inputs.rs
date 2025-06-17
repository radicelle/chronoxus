use crate::calendar;
use crate::calendar_grid::CalendarGrid;
use chrono::Datelike;
use chrono::Utc;
use dioxus::prelude::*;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{Blob, HtmlAnchorElement, Url};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
fn create_and_download_calendar(hours: Vec<&str>, start_date: &str) {
    log("Going to generate calendar");
    // Generate the calendar data as a string
    match calendar::generate_calendar_blob(&hours, start_date) {
        Ok(calendar_data) => {
            log("Calendar generated successfully");

            // Create a Blob from the calendar data
            let window = web_sys::window().expect("no global window exists");
            let document = window.document().expect("no document exists");

            // Create a JS array to hold our data
            let array = Array::new();
            array.push(&JsValue::from_str(&calendar_data));

            // Create a Blob with the calendar data
            let blob = Blob::new_with_str_sequence(&array)
                .expect("could not create blob");

            // Create a URL for the Blob
            let url = Url::create_object_url_with_blob(&blob)
                .expect("could not create object URL");

            // Create a new anchor element
            let anchor = document.create_element("a")
                .expect("could not create anchor element")
                .dyn_into::<HtmlAnchorElement>()
                .expect("could not cast to HtmlAnchorElement");

            // Set the download attribute and href
            anchor.set_download("calendrier.ics");
            anchor.set_href(&url);

            // Append to document, click, and remove
            document.body().expect("document has no body").append_child(&anchor).expect("could not append anchor");
            anchor.click();
            document.body().expect("document has no body").remove_child(&anchor).expect("could not remove anchor");

            // Clean up by revoking the URL
            Url::revoke_object_url(&url).expect("could not revoke object URL");
        },
        Err(e) => {
            log(&format!("Error generating calendar: {:?}", e));
        }
    }
}

#[component]
pub fn Inputs(selected_month: i32) -> Element {
    rsx! {
        div {
            id: "daily_input",
            CalendarGrid { year: Utc::now().year(), month: selected_month }
            button { 
                key: "create-calendar", 
                class: "create-button", 
                onclick: move |_| {
                    // Get the window and document objects
                    let window = web_sys::window().expect("no global window exists");
                    let document = window.document().expect("no document exists");

                    // Collect all input values from the calendar grid
                    let mut hours: Vec<&str> = Vec::new();
                    let year = Utc::now().year();

                    // Generate a start date string for the first day of the selected month
                    let start_date = format!("{}-{:02}-01", year, selected_month);

                    // Get all input elements with class "day-input"
                    if let Ok(inputs) = document.query_selector_all(".day-input") {
                        let length = inputs.length();
                        for i in 0..length {
                            if let Some(node) = inputs.get(i) {
                                if let Ok(input) = node.dyn_into::<web_sys::HtmlInputElement>() {
                                    let value = input.value();
                                    if !value.is_empty() {
                                        // Store non-empty values in the hours vector
                                        hours.push(Box::leak(value.into_boxed_str()));
                                    }
                                }
                            }
                        }
                    }

                    // Call the function with the collected hours and start date
                    create_and_download_calendar(hours, &start_date);
                }, 
                "Create" 
            }
        }
    }
}
