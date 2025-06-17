use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use icalendar::{Calendar, Component, Event, EventLike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Add;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimePeriod {
    pub start_time: String,
    pub end_time: String,
    pub title: String,
    pub description: String,
}

pub fn time_str(time_str: &str) -> NaiveTime {
    NaiveTime::parse_from_str(time_str, "%H:%M").unwrap()
}

pub fn parse_config(contents: &str) -> Result<HashMap<String, TimePeriod>, serde_yaml::Error> {
    serde_yaml::from_str(contents)
}

pub fn create_event_datetime(
    base_date: &NaiveDate, 
    index: usize, 
    time_string: &str
) -> NaiveDateTime {
    NaiveDateTime::new(
        base_date.add(TimeDelta::days(index as i64)),
        time_str(time_string),
    )
}

pub fn create_calendar_event(
    time_period: &TimePeriod, 
    start_time: NaiveDateTime, 
    end_time: NaiveDateTime
) -> Event {
    Event::new()
        .summary(&time_period.title)
        .starts(start_time)
        .ends(end_time)
        //.description(&time_period.description)
        .done()
}

pub fn build_calendar(
    hours: &[&str], 
    config_map: &HashMap<String, TimePeriod>, 
    first_date: NaiveDate
) -> Calendar {
    let mut calendar = Calendar::new();

    for (index, &hour) in hours.iter().enumerate() {
        if let Some(event) = config_map.get(hour) {
            add_event_to_calendar(&mut calendar, event, &first_date, index);
        }
    }

    calendar
}

fn add_event_to_calendar(
    calendar: &mut Calendar, 
    time_period: &TimePeriod, 
    first_date: &NaiveDate, 
    index: usize
) {
    let start_time = create_event_datetime(first_date, index, &time_period.start_time);
    let end_time = create_event_datetime(first_date, index, &time_period.end_time);

    let event = create_calendar_event(time_period, start_time, end_time);
    calendar.push(event);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn generate_calendar_blob(hours: &[&str], first_date: &str) -> Result<String, Box<dyn std::error::Error>> {
    log("before reading config");
    let config_map = parse_config(DEFAULT_CONFIG)?;

    let first_date = first_date.parse::<NaiveDate>().unwrap();
    let calendar = build_calendar(hours, &config_map, first_date);

    // Convert calendar to string
    Ok(calendar.to_string())
}


static DEFAULT_CONFIG: &str = r#"F:
  start_time: "9:00"
  end_time: "17:00"
  title: "Formation 7h"
  description: ""

TA:
  start_time: "7:30"
  end_time: "19:00"
  title: "TA bloc 11h"
  description: ""

T:
  start_time: "7:30"
  end_time: "18:00"
  title: "T bloc 10h"
  description: ""

AM:
  start_time: "10:00"
  end_time: "20:00"
  title: "AM 9,5h"
  description: ""

M:
  start_time: "7:30"
  end_time: "14:00"
  title: "Matin 6,5h"
  description: ""

J3:
  start_time: "7:30"
  end_time: "16:00"
  title: "J3 Journée 8h"
  description: ""

R2:
  start_time: "9:30"
  end_time: "19:30"
  title: "R2 journée 9.5h"
  description: ""

R1:
  start_time: "8:15"
  end_time: "16:15"
  title: "R1 matin 7.5h"
  description: ""

SEM:
  start_time: "8:00"
  end_time: "13:00"
  title: "SEM (soins externes) 5h"
  description: ""
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_default_config() {
        // Test that the default configuration can be parsed without errors
        let result = parse_config(DEFAULT_CONFIG);
        assert!(result.is_ok(), "Failed to parse DEFAULT_CONFIG: {:?}", result.err());

        // Validate the parsed configuration
        let config_map = result.unwrap();

        // Check that we have the expected number of entries
        assert_eq!(config_map.len(), 9, "Expected 9 entries in the config map");

        // Check that all expected keys are present
        let expected_keys = ["F", "TA", "T", "AM", "M", "J3", "R2", "R1", "SEM"];
        for key in expected_keys.iter() {
            assert!(config_map.contains_key(*key), "Missing key: {}", key);
        }

        // Validate a few specific entries
        if let Some(f_entry) = config_map.get("F") {
            assert_eq!(f_entry.start_time, "9:00");
            assert_eq!(f_entry.end_time, "17:00");
            assert_eq!(f_entry.title, "Formation 7h");
        } else {
            panic!("F entry not found in config map");
        }

        if let Some(ta_entry) = config_map.get("TA") {
            assert_eq!(ta_entry.start_time, "7:30");
            assert_eq!(ta_entry.end_time, "19:00");
            assert_eq!(ta_entry.title, "TA bloc 11h");
        } else {
            panic!("TA entry not found in config map");
        }

        if let Some(sem_entry) = config_map.get("SEM") {
            assert_eq!(sem_entry.start_time, "8:00");
            assert_eq!(sem_entry.end_time, "13:00");
            assert_eq!(sem_entry.title, "SEM (soins externes) 5h");
        } else {
            panic!("SEM entry not found in config map");
        }
    }
}
