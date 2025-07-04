use chrono::{DateTime, Datelike, Utc};
use json::JsonValue;
use std::collections::HashMap;

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for commit in data.members() {
        let count = counts
            .entry(commit["author"]["login"].to_string())
            .or_insert(0);
        *count += 1;
    }

    counts
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                let date_utc = date.with_timezone(&Utc);
                let iso_week = date_utc.iso_week();
                let year = iso_week.year();
                let week = iso_week.week();

                let week_str = format!("{}-W{}", year, week);
                *counts.entry(week_str).or_insert(0) += 1;
            }
        }
    }

    counts
}
