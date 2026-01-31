use chrono::{DateTime, Datelike, Duration, TimeZone, Utc, Weekday};

/// Calculate the number of working days between two dates (excluding weekends)
pub fn working_days_between(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    let mut days = 0;
    let mut current = start.date_naive();
    let end_date = end.date_naive();

    while current <= end_date {
        if !is_weekend(current.weekday()) {
            days += 1;
        }
        current = current.succ_opt().unwrap();
    }

    days
}

/// Check if a given weekday is a weekend
pub fn is_weekend(weekday: Weekday) -> bool {
    matches!(weekday, Weekday::Sat | Weekday::Sun)
}

/// Round a datetime to the start of the day
pub fn start_of_day(dt: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0)
        .unwrap()
}

/// Round a datetime to the end of the day
pub fn end_of_day(dt: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 23, 59, 59)
        .unwrap()
}

/// Get the start of the week for a given date (Monday)
pub fn start_of_week(dt: DateTime<Utc>) -> DateTime<Utc> {
    let days_from_monday = dt.weekday().num_days_from_monday();
    dt - Duration::days(days_from_monday as i64)
}

/// Get the end of the week for a given date (Sunday)
pub fn end_of_week(dt: DateTime<Utc>) -> DateTime<Utc> {
    let days_to_sunday = 6 - dt.weekday().num_days_from_monday();
    dt + Duration::days(days_to_sunday as i64)
}

/// Get the start of the month for a given date
pub fn start_of_month(dt: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(dt.year(), dt.month(), 1, 0, 0, 0)
        .unwrap()
}

/// Get the end of the month for a given date
pub fn end_of_month(dt: DateTime<Utc>) -> DateTime<Utc> {
    let next_month = if dt.month() == 12 {
        Utc.with_ymd_and_hms(dt.year() + 1, 1, 1, 0, 0, 0)
            .unwrap()
    } else {
        Utc.with_ymd_and_hms(dt.year(), dt.month() + 1, 1, 0, 0, 0)
            .unwrap()
    };
    next_month - Duration::days(1)
}

/// Get the start of the quarter for a given date
pub fn start_of_quarter(dt: DateTime<Utc>) -> DateTime<Utc> {
    let quarter_start_month = ((dt.month() - 1) / 3) * 3 + 1;
    Utc.with_ymd_and_hms(dt.year(), quarter_start_month, 1, 0, 0, 0)
        .unwrap()
}

/// Get the end of the quarter for a given date
pub fn end_of_quarter(dt: DateTime<Utc>) -> DateTime<Utc> {
    let quarter_start_month = ((dt.month() - 1) / 3) * 3 + 1;
    let quarter_end_month = quarter_start_month + 2;

    let next_quarter = if quarter_end_month == 12 {
        Utc.with_ymd_and_hms(dt.year() + 1, 1, 1, 0, 0, 0)
            .unwrap()
    } else {
        Utc.with_ymd_and_hms(dt.year(), quarter_end_month + 1, 1, 0, 0, 0)
            .unwrap()
    };
    next_quarter - Duration::days(1)
}

/// Get the start of the year for a given date
pub fn start_of_year(dt: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(dt.year(), 1, 1, 0, 0, 0).unwrap()
}

/// Get the end of the year for a given date
pub fn end_of_year(dt: DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(dt.year(), 12, 31, 23, 59, 59)
        .unwrap()
}

/// Calculate the duration in days between two datetimes
pub fn duration_in_days(start: DateTime<Utc>, end: DateTime<Utc>) -> i64 {
    (end - start).num_days()
}

/// Add working days to a date (skipping weekends)
pub fn add_working_days(start: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    let mut current = start;
    let mut remaining = days;

    while remaining > 0 {
        current = current + Duration::days(1);
        if !is_weekend(current.weekday()) {
            remaining -= 1;
        }
    }

    current
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn test_is_weekend() {
        assert!(is_weekend(Weekday::Sat));
        assert!(is_weekend(Weekday::Sun));
        assert!(!is_weekend(Weekday::Mon));
        assert!(!is_weekend(Weekday::Fri));
    }

    #[test]
    fn test_working_days_between() {
        // Monday to Friday = 5 working days
        let monday = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let friday = Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap();
        assert_eq!(working_days_between(monday, friday), 5);

        // Monday to Sunday = 5 working days (excludes weekend)
        let sunday = Utc.with_ymd_and_hms(2024, 1, 7, 0, 0, 0).unwrap();
        assert_eq!(working_days_between(monday, sunday), 5);
    }

    #[test]
    fn test_start_end_of_day() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 14, 30, 45).unwrap();

        let start = start_of_day(dt);
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);

        let end = end_of_day(dt);
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);
    }

    #[test]
    fn test_duration_in_days() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 11, 0, 0, 0).unwrap();
        assert_eq!(duration_in_days(start, end), 10);
    }
}
