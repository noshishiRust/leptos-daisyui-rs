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
        current += Duration::days(1);
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
        assert!(!is_weekend(Weekday::Tue));
        assert!(!is_weekend(Weekday::Wed));
        assert!(!is_weekend(Weekday::Thu));
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

        // Same day = 1 working day
        assert_eq!(working_days_between(monday, monday), 1);
    }

    #[test]
    fn test_start_end_of_day() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 14, 30, 45).unwrap();

        let start = start_of_day(dt);
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);
        assert_eq!(start.day(), 15);

        let end = end_of_day(dt);
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);
        assert_eq!(end.day(), 15);
    }

    #[test]
    fn test_start_of_week() {
        // Wednesday Jan 3, 2024 -> should go back to Monday Jan 1
        let wed = Utc.with_ymd_and_hms(2024, 1, 3, 12, 0, 0).unwrap();
        let start = start_of_week(wed);
        assert_eq!(start.weekday(), Weekday::Mon);
        assert_eq!(start.day(), 1);

        // Monday should stay Monday
        let mon = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let start_mon = start_of_week(mon);
        assert_eq!(start_mon.day(), 1);
    }

    #[test]
    fn test_end_of_week() {
        // Monday Jan 1, 2024 -> should go to Sunday Jan 7
        let mon = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let end = end_of_week(mon);
        assert_eq!(end.weekday(), Weekday::Sun);
        assert_eq!(end.day(), 7);

        // Sunday should stay Sunday
        let sun = Utc.with_ymd_and_hms(2024, 1, 7, 12, 0, 0).unwrap();
        let end_sun = end_of_week(sun);
        assert_eq!(end_sun.day(), 7);
    }

    #[test]
    fn test_start_of_month() {
        // Middle of January should go to Jan 1
        let mid_jan = Utc.with_ymd_and_hms(2024, 1, 15, 12, 30, 0).unwrap();
        let start = start_of_month(mid_jan);
        assert_eq!(start.day(), 1);
        assert_eq!(start.month(), 1);
        assert_eq!(start.hour(), 0);

        // First day should stay first day
        let first = Utc.with_ymd_and_hms(2024, 6, 1, 12, 0, 0).unwrap();
        let start_first = start_of_month(first);
        assert_eq!(start_first.day(), 1);
        assert_eq!(start_first.month(), 6);
    }

    #[test]
    fn test_end_of_month() {
        // Middle of January should go to Jan 31
        let mid_jan = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
        let end = end_of_month(mid_jan);
        assert_eq!(end.day(), 31);
        assert_eq!(end.month(), 1);

        // February in leap year should be 29
        let feb = Utc.with_ymd_and_hms(2024, 2, 10, 12, 0, 0).unwrap();
        let end_feb = end_of_month(feb);
        assert_eq!(end_feb.day(), 29);

        // December should handle year transition
        let dec = Utc.with_ymd_and_hms(2024, 12, 15, 12, 0, 0).unwrap();
        let end_dec = end_of_month(dec);
        assert_eq!(end_dec.day(), 31);
        assert_eq!(end_dec.month(), 12);
    }

    #[test]
    fn test_start_of_quarter() {
        // Q1 (Jan-Mar) -> Jan 1
        let q1 = Utc.with_ymd_and_hms(2024, 2, 15, 0, 0, 0).unwrap();
        let start_q1 = start_of_quarter(q1);
        assert_eq!(start_q1.month(), 1);
        assert_eq!(start_q1.day(), 1);

        // Q2 (Apr-Jun) -> Apr 1
        let q2 = Utc.with_ymd_and_hms(2024, 5, 15, 0, 0, 0).unwrap();
        let start_q2 = start_of_quarter(q2);
        assert_eq!(start_q2.month(), 4);
        assert_eq!(start_q2.day(), 1);

        // Q3 (Jul-Sep) -> Jul 1
        let q3 = Utc.with_ymd_and_hms(2024, 8, 15, 0, 0, 0).unwrap();
        let start_q3 = start_of_quarter(q3);
        assert_eq!(start_q3.month(), 7);
        assert_eq!(start_q3.day(), 1);

        // Q4 (Oct-Dec) -> Oct 1
        let q4 = Utc.with_ymd_and_hms(2024, 11, 15, 0, 0, 0).unwrap();
        let start_q4 = start_of_quarter(q4);
        assert_eq!(start_q4.month(), 10);
        assert_eq!(start_q4.day(), 1);
    }

    #[test]
    fn test_end_of_quarter() {
        // Q1 ends March 31
        let q1 = Utc.with_ymd_and_hms(2024, 2, 15, 0, 0, 0).unwrap();
        let end_q1 = end_of_quarter(q1);
        assert_eq!(end_q1.month(), 3);
        assert_eq!(end_q1.day(), 31);

        // Q2 ends June 30
        let q2 = Utc.with_ymd_and_hms(2024, 5, 15, 0, 0, 0).unwrap();
        let end_q2 = end_of_quarter(q2);
        assert_eq!(end_q2.month(), 6);
        assert_eq!(end_q2.day(), 30);

        // Q3 ends September 30
        let q3 = Utc.with_ymd_and_hms(2024, 8, 15, 0, 0, 0).unwrap();
        let end_q3 = end_of_quarter(q3);
        assert_eq!(end_q3.month(), 9);
        assert_eq!(end_q3.day(), 30);

        // Q4 ends December 31
        let q4 = Utc.with_ymd_and_hms(2024, 11, 15, 0, 0, 0).unwrap();
        let end_q4 = end_of_quarter(q4);
        assert_eq!(end_q4.month(), 12);
        assert_eq!(end_q4.day(), 31);
    }

    #[test]
    fn test_start_of_year() {
        let mid_year = Utc.with_ymd_and_hms(2024, 6, 15, 12, 30, 45).unwrap();
        let start = start_of_year(mid_year);
        assert_eq!(start.year(), 2024);
        assert_eq!(start.month(), 1);
        assert_eq!(start.day(), 1);
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);
    }

    #[test]
    fn test_end_of_year() {
        let mid_year = Utc.with_ymd_and_hms(2024, 6, 15, 12, 0, 0).unwrap();
        let end = end_of_year(mid_year);
        assert_eq!(end.year(), 2024);
        assert_eq!(end.month(), 12);
        assert_eq!(end.day(), 31);
        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);
    }

    #[test]
    fn test_duration_in_days() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 11, 0, 0, 0).unwrap();
        assert_eq!(duration_in_days(start, end), 10);

        // Same day
        assert_eq!(duration_in_days(start, start), 0);

        // Negative (end before start)
        assert_eq!(duration_in_days(end, start), -10);
    }

    #[test]
    fn test_add_working_days() {
        // Friday + 1 working day = Monday (skip weekend)
        let friday = Utc.with_ymd_and_hms(2024, 1, 5, 12, 0, 0).unwrap();
        let next_day = add_working_days(friday, 1);
        assert_eq!(next_day.weekday(), Weekday::Mon);
        assert_eq!(next_day.day(), 8);

        // Friday + 5 working days = next Friday
        let five_days = add_working_days(friday, 5);
        assert_eq!(five_days.weekday(), Weekday::Fri);
        assert_eq!(five_days.day(), 12);

        // Monday + 0 working days = Monday
        let monday = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let same = add_working_days(monday, 0);
        assert_eq!(same, monday);
    }
}
