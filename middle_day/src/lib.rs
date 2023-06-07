use chrono::{NaiveDate, Datelike, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    if (date.iter_days().count()+1) % 2 == 0 {
        None
    } else {
        Some(date.weekday())
    }
}