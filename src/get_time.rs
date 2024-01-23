use chrono::{Datelike, Local, Timelike, Weekday};

pub fn fetch_current_time() -> String {
    let now = Local::now();
    let weekday = match now.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    format!(
        "[TEST] Obsidian Sync time : {}, {}-{}-{} {}:{}:{}",
        weekday,
        now.day(),
        now.month(),
        now.year(),
        now.hour(),
        now.minute(),
        now.second()
    )
}
