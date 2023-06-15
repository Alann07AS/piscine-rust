pub use json::JsonValue;
use chrono::NaiveDateTime;

pub use std::collections::HashMap;
	// let date = DateTime::parse_from_rfc3339("2020-12-10T08:26:02Z").unwrap();


pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    data.members().for_each(|c| {
        let date = NaiveDateTime::parse_from_str(
            c["commit"]["author"]["date"].as_str().unwrap(),
            "%Y-%m-%dT%H:%M:%SZ"
        ).unwrap().format("%Y-W%U").to_string();
        *map.entry(date).or_insert(0) += 1;
    });
    map
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    data.members().for_each(|c| {
        *map.entry(c["author"]["login"].as_str().unwrap().to_string()).or_insert(0) += 1;
    });
    map
}