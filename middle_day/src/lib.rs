use chrono::prelude::*;
use chrono::NaiveDate;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if year % 4 == 0 {
        return None;
    }

    let mut date_str = String::from("02/07/");
    date_str.push_str(year.to_string().as_str());

    let date = match NaiveDate::parse_from_str(date_str.as_str(), "%d/%m/%Y") {
        Ok(date) => date.format("%a %m %Y"),
        Err(_) => return None,
    };

    let binding = date.to_string().clone();
    let formatted_date = binding.split(" ").collect::<Vec<_>>();

    match formatted_date[0].parse::<Weekday>() {
        Ok(date) => Some(date),
        Err(_) => None,
    }
}
