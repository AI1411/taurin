use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimestampUnit {
    Seconds,
    Milliseconds,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimezoneOption {
    Local,
    Utc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnixToDateTimeResult {
    pub success: bool,
    pub datetime: String,
    pub iso8601: String,
    pub date: String,
    pub time: String,
    pub day_of_week: String,
    pub relative_time: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeToUnixResult {
    pub success: bool,
    pub unix_seconds: i64,
    pub unix_milliseconds: i64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUnixTimeResult {
    pub unix_seconds: i64,
    pub unix_milliseconds: i64,
    pub datetime: String,
    pub iso8601: String,
}

pub fn unix_to_datetime(
    timestamp: i64,
    unit: TimestampUnit,
    timezone: TimezoneOption,
) -> UnixToDateTimeResult {
    let timestamp_secs = match unit {
        TimestampUnit::Seconds => timestamp,
        TimestampUnit::Milliseconds => timestamp / 1000,
    };

    let timestamp_nanos = match unit {
        TimestampUnit::Seconds => 0,
        TimestampUnit::Milliseconds => ((timestamp % 1000) * 1_000_000) as u32,
    };

    let utc_dt = match DateTime::<Utc>::from_timestamp(timestamp_secs, timestamp_nanos) {
        Some(dt) => dt,
        None => {
            return UnixToDateTimeResult {
                success: false,
                datetime: String::new(),
                iso8601: String::new(),
                date: String::new(),
                time: String::new(),
                day_of_week: String::new(),
                relative_time: String::new(),
                error: Some("Invalid timestamp".to_string()),
            }
        }
    };

    let (datetime_str, iso8601, date_str, time_str, day_of_week) = match timezone {
        TimezoneOption::Local => {
            let local_dt: DateTime<Local> = utc_dt.with_timezone(&Local);
            (
                local_dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                local_dt.to_rfc3339(),
                local_dt.format("%Y-%m-%d").to_string(),
                local_dt.format("%H:%M:%S").to_string(),
                local_dt.format("%A").to_string(),
            )
        }
        TimezoneOption::Utc => (
            utc_dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            utc_dt.to_rfc3339(),
            utc_dt.format("%Y-%m-%d").to_string(),
            utc_dt.format("%H:%M:%S").to_string(),
            utc_dt.format("%A").to_string(),
        ),
    };

    let relative_time = calculate_relative_time(timestamp_secs);

    UnixToDateTimeResult {
        success: true,
        datetime: datetime_str,
        iso8601,
        date: date_str,
        time: time_str,
        day_of_week,
        relative_time,
        error: None,
    }
}

pub fn datetime_to_unix(datetime_str: &str, timezone: TimezoneOption) -> DateTimeToUnixResult {
    // Try multiple formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M",
        "%Y-%m-%d",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
        "%Y/%m/%d",
    ];

    // Try to parse as RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(datetime_str) {
        let unix_secs = dt.timestamp();
        let unix_ms = dt.timestamp_millis();
        return DateTimeToUnixResult {
            success: true,
            unix_seconds: unix_secs,
            unix_milliseconds: unix_ms,
            error: None,
        };
    }

    for format in formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(datetime_str, format) {
            let unix_secs = match timezone {
                TimezoneOption::Local => Local
                    .from_local_datetime(&naive)
                    .single()
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|| naive.and_utc().timestamp()),
                TimezoneOption::Utc => naive.and_utc().timestamp(),
            };
            let unix_ms = unix_secs * 1000;
            return DateTimeToUnixResult {
                success: true,
                unix_seconds: unix_secs,
                unix_milliseconds: unix_ms,
                error: None,
            };
        }

        // Try date-only formats
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(datetime_str, format) {
            let naive = naive_date.and_hms_opt(0, 0, 0).unwrap();
            let unix_secs = match timezone {
                TimezoneOption::Local => Local
                    .from_local_datetime(&naive)
                    .single()
                    .map(|dt| dt.timestamp())
                    .unwrap_or_else(|| naive.and_utc().timestamp()),
                TimezoneOption::Utc => naive.and_utc().timestamp(),
            };
            let unix_ms = unix_secs * 1000;
            return DateTimeToUnixResult {
                success: true,
                unix_seconds: unix_secs,
                unix_milliseconds: unix_ms,
                error: None,
            };
        }
    }

    DateTimeToUnixResult {
        success: false,
        unix_seconds: 0,
        unix_milliseconds: 0,
        error: Some("Invalid datetime format".to_string()),
    }
}

pub fn get_current_unix_time() -> CurrentUnixTimeResult {
    let now = Utc::now();
    let local_now = Local::now();

    CurrentUnixTimeResult {
        unix_seconds: now.timestamp(),
        unix_milliseconds: now.timestamp_millis(),
        datetime: local_now.format("%Y-%m-%d %H:%M:%S").to_string(),
        iso8601: now.to_rfc3339(),
    }
}

fn calculate_relative_time(timestamp_secs: i64) -> String {
    let now = Utc::now().timestamp();
    let diff = now - timestamp_secs;

    if diff == 0 {
        return "now".to_string();
    }

    let (abs_diff, suffix, prefix) = if diff > 0 {
        (diff, " ago", "")
    } else {
        (-diff, "", "in ")
    };

    let (value, unit) = if abs_diff < 60 {
        (abs_diff, if abs_diff == 1 { "second" } else { "seconds" })
    } else if abs_diff < 3600 {
        let mins = abs_diff / 60;
        (mins, if mins == 1 { "minute" } else { "minutes" })
    } else if abs_diff < 86400 {
        let hours = abs_diff / 3600;
        (hours, if hours == 1 { "hour" } else { "hours" })
    } else if abs_diff < 2592000 {
        let days = abs_diff / 86400;
        (days, if days == 1 { "day" } else { "days" })
    } else if abs_diff < 31536000 {
        let months = abs_diff / 2592000;
        (months, if months == 1 { "month" } else { "months" })
    } else {
        let years = abs_diff / 31536000;
        (years, if years == 1 { "year" } else { "years" })
    };

    format!("{}{} {}{}", prefix, value, unit, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unix_to_datetime() {
        let result = unix_to_datetime(0, TimestampUnit::Seconds, TimezoneOption::Utc);
        assert!(result.success);
        assert_eq!(result.datetime, "1970-01-01 00:00:00 UTC");
        assert_eq!(result.day_of_week, "Thursday");
    }

    #[test]
    fn test_unix_to_datetime_milliseconds() {
        let result = unix_to_datetime(1000000, TimestampUnit::Milliseconds, TimezoneOption::Utc);
        assert!(result.success);
        assert_eq!(result.datetime, "1970-01-01 00:16:40 UTC");
    }

    #[test]
    fn test_datetime_to_unix() {
        let result = datetime_to_unix("1970-01-01 00:00:00", TimezoneOption::Utc);
        assert!(result.success);
        assert_eq!(result.unix_seconds, 0);
    }

    #[test]
    fn test_datetime_to_unix_iso8601() {
        let result = datetime_to_unix("2020-01-01T00:00:00Z", TimezoneOption::Utc);
        assert!(result.success);
        assert_eq!(result.unix_seconds, 1577836800);
    }

    #[test]
    fn test_get_current_unix_time() {
        let result = get_current_unix_time();
        assert!(result.unix_seconds > 0);
        assert!(result.unix_milliseconds > 0);
    }

    #[test]
    fn test_relative_time() {
        let now = Utc::now().timestamp();
        let result = unix_to_datetime(now - 3600, TimestampUnit::Seconds, TimezoneOption::Utc);
        assert!(result.relative_time.contains("hour"));
    }
}
