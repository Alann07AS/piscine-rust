pub use chrono::{NaiveDate, Datelike, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    let date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    if (date.iter_days().take_while(|datebis| datebis.year() == year).count()) % 2 == 0 {
        None
    } else {
        Some(date.weekday())
    }
}