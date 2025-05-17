use chrono::NaiveDate;
use chrono::prelude::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if year % 4 == 0 {
        return None;
    }

    let mut date_str = String::from("02/07/");
    date_str.push_str(year.to_string().as_str());

    let format = "%d/%m/%Y";

    let date = NaiveDate::parse_from_str(date_str.as_str(), format)
        .unwrap()
        .format("%a %m %Y");

    let binding = date.to_string().clone();
    let formatted_date = binding.split(" ").collect::<Vec<_>>();

    Some(formatted_date[0].parse::<Weekday>().unwrap())
}
