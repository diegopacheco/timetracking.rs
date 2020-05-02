use chrono::prelude::*;
use chrono::NaiveDate;
use bdays::HolidayCalendar;

pub fn current_year() -> i32 {
    return Local::now().date().year();
}

pub fn current_month() -> u32 {
    return Local::now().date().month();
}

pub fn today() -> u32 {
    return Local::now().date().day();
}

pub fn working_days() -> i64 {
    let cal = bdays::calendars::WeekendsOnly;
    let year = current_year();
    let month = current_month();
    let mut days = 0;
    for i in 1..31 {
        let date = NaiveDate::from_ymd(year, month, i);
        if cal.is_bday(date) {
            days += 1;
        }
    }
    return days;
}

pub fn workable_days() -> i32 {
    let today = today() as i32;
    let working_days = working_days() as i32;
    return working_days - today as i32;
}

pub fn worked_days() -> u32 {
    let today = today() as u32;
    let working_days = working_days() as u32;
    return working_days - (working_days - today);
}