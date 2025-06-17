mod months;
mod inputs;
mod calendar_grid;
mod header;
mod calendar;

use months::{Months};
use inputs::{Inputs};
use header::{Header};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Header {}
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    let selected_month = use_signal(|| 1);
    rsx! {
        div {
            id: "hero",
            Months { selected_month_signal: selected_month }
            Inputs { selected_month: selected_month() }
        }
    }
}
