//! Helper functions for calculating dates

use chrono::{Datelike, NaiveDate};
use lazy_static::lazy_static;

/// Creates a date with the previous month based on the given date.
pub fn pred_month(date: &NaiveDate) -> NaiveDate {
    let (year, month) = if date.month() == 1 {
        (date.year() - 1, 12)
    } else {
        (date.year(), date.month() - 1)
    };

    let day = date.day().min(num_days_of_month(year, month));

    NaiveDate::from_ymd(year, month, day)
}

/// Creates a date with the next month based on given date.
pub fn succ_month(date: &NaiveDate) -> NaiveDate {
    let (year, month) = if date.month() == 12 {
        (date.year() + 1, 1)
    } else {
        (date.year(), date.month() + 1)
    };

    let day = date.day().min(num_days_of_month(year, month));

    NaiveDate::from_ymd(year, month, day)
}

/// Creates a date with the previous year based on the given date.
pub fn pred_year(date: &NaiveDate) -> NaiveDate {
    let year = date.year() - 1;
    let day = date.day().min(num_days_of_month(year, date.month()));

    NaiveDate::from_ymd(year, date.month(), day)
}

/// Creates a date with the next year based on the given date.
pub fn succ_year(date: &NaiveDate) -> NaiveDate {
    let year = date.year() + 1;
    let day = date.day().min(num_days_of_month(year, date.month()));

    NaiveDate::from_ymd(year, date.month(), day)
}

/// Calculates the day number at the given position in the calendar table based
/// on the given year and month.
pub fn position_to_day(x: usize, y: usize, year: i32, month: u32) -> (usize, i8) {
    let (x, y) = (x as isize, y as isize);
    let first_day = NaiveDate::from_ymd(year, month, 1);
    let day_of_week = first_day.weekday().num_days_from_monday() as isize;
    let day_of_week = if day_of_week == 0 { 7 } else { day_of_week };

    let day = (x + 7 * y) + 1 - day_of_week;

    if day < 1 {
        let last_month = first_day.pred();
        (
            (num_days_of_month(last_month.year(), last_month.month()) as isize + day) as usize,
            -1,
        )
    } else if day > num_days_of_month(year, month) as isize {
        ((day - num_days_of_month(year, month) as isize) as usize, 1)
    } else {
        (day as usize, 0)
    }
}

/// Checks if the given year is a leap year.
fn is_leap_year(year: i32) -> bool {
    let mod4 = year % 4 == 0;
    let mod100 = year % 100 == 0;
    let mod400 = year % 400 == 0;

    mod4 && (!mod100 || mod400)
}

/// Gets the number of days the given month in the year has.
fn num_days_of_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

/// Gets the string representation of the year of the given date.
pub fn year_as_string(date: &NaiveDate) -> String {
    date.format("%Y").to_string()
}

/// Gets the string representation of the month of the given date.
pub fn month_as_string(date: &NaiveDate) -> String {
    date.format("%B").to_string()
}

lazy_static! {
    /// Gets the length of the longest month name.
    pub static ref MAX_MONTH_STR_LEN: usize = {
        let months = [
            NaiveDate::from_ymd(0, 1, 1),
            NaiveDate::from_ymd(0, 2, 1),
            NaiveDate::from_ymd(0, 3, 1),
            NaiveDate::from_ymd(0, 4, 1),
            NaiveDate::from_ymd(0, 5, 1),
            NaiveDate::from_ymd(0, 6, 1),
            NaiveDate::from_ymd(0, 7, 1),
            NaiveDate::from_ymd(0, 8, 1),
            NaiveDate::from_ymd(0, 9, 1),
            NaiveDate::from_ymd(0, 10, 1),
            NaiveDate::from_ymd(0, 11, 1),
            NaiveDate::from_ymd(0, 12, 1),
        ];

        let max = months.iter()
            .map(|m| month_as_string(m))
            .map(|s| s.len())
            .max().unwrap();

        max
    };

    /// Gets the labels of the weekdays containing the first two characters of
    /// the weekdays.
    pub static ref WEEKDAY_LABELS: Vec<String> = {
        let days = [
            // Monday
            NaiveDate::from_ymd(2020, 6, 1),
            // Tuesday
            NaiveDate::from_ymd(2020, 6, 2),
            // Wednesday
            NaiveDate::from_ymd(2020, 6, 3),
            // Thursday
            NaiveDate::from_ymd(2020, 6, 4),
            // Friday
            NaiveDate::from_ymd(2020, 6, 5),
            // Saturday
            NaiveDate::from_ymd(2020, 6, 6),
            // Sunday
            NaiveDate::from_ymd(2020, 6, 7),

        ];

        days.iter()
            .map(|d| d.format("%a").to_string())
            .map(|s| s[0..2].to_owned())
            .collect()
    };
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{
        is_leap_year, num_days_of_month, position_to_day, pred_month, pred_year, succ_month,
        succ_year,
    };

    #[test]
    fn pred_month_test() {
        let date = NaiveDate::from_ymd(2020, 5, 6);
        let result = pred_month(&date);
        let expected = NaiveDate::from_ymd(2020, 4, 6);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2020, 1, 24);
        let result = pred_month(&date);
        let expected = NaiveDate::from_ymd(2019, 12, 24);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2020, 3, 31);
        let result = pred_month(&date);
        let expected = NaiveDate::from_ymd(2020, 2, 29);
        assert_eq!(result, expected);
    }

    #[test]
    fn succ_month_test() {
        let date = NaiveDate::from_ymd(2020, 5, 6);
        let result = succ_month(&date);
        let expected = NaiveDate::from_ymd(2020, 6, 6);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2019, 12, 24);
        let result = succ_month(&date);
        let expected = NaiveDate::from_ymd(2020, 1, 24);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2020, 1, 31);
        let result = succ_month(&date);
        let expected = NaiveDate::from_ymd(2020, 2, 29);
        assert_eq!(result, expected);
    }

    #[test]
    fn pred_year_test() {
        let date = NaiveDate::from_ymd(2020, 5, 6);
        let result = pred_year(&date);
        let expected = NaiveDate::from_ymd(2019, 5, 6);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2020, 2, 29);
        let result = pred_year(&date);
        let expected = NaiveDate::from_ymd(2019, 2, 28);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2021, 2, 28);
        let result = pred_year(&date);
        let expected = NaiveDate::from_ymd(2020, 2, 28);
        assert_eq!(result, expected);
    }

    #[test]
    fn succ_year_test() {
        let date = NaiveDate::from_ymd(2020, 5, 6);
        let result = succ_year(&date);
        let expected = NaiveDate::from_ymd(2021, 5, 6);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2020, 2, 29);
        let result = succ_year(&date);
        let expected = NaiveDate::from_ymd(2021, 2, 28);
        assert_eq!(result, expected);

        let date = NaiveDate::from_ymd(2019, 2, 28);
        let result = succ_year(&date);
        let expected = NaiveDate::from_ymd(2020, 2, 28);
        assert_eq!(result, expected);
    }

    #[test]
    fn position_to_day_test() {
        let (day, is_in_month) = position_to_day(0, 0, 2020, 12);
        assert_eq!(day, 30);
        assert_eq!(is_in_month, -1);

        let (day, is_in_month) = position_to_day(1, 0, 2020, 12);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, 0);

        let (day, is_in_month) = position_to_day(3, 4, 2020, 12);
        assert_eq!(day, 31);
        assert_eq!(is_in_month, 0);

        let (day, is_in_month) = position_to_day(6, 5, 2020, 12);
        assert_eq!(day, 10);
        assert_eq!(is_in_month, 1);

        let (day, is_in_month) = position_to_day(0, 0, 2020, 11);
        assert_eq!(day, 26);
        assert_eq!(is_in_month, -1);

        let (day, is_in_month) = position_to_day(6, 0, 2020, 11);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, 0);

        let (day, is_in_month) = position_to_day(0, 5, 2020, 11);
        assert_eq!(day, 30);
        assert_eq!(is_in_month, 0);

        let (day, is_in_month) = position_to_day(6, 5, 2020, 11);
        assert_eq!(day, 6);
        assert_eq!(is_in_month, 1);

        let (day, is_in_month) = position_to_day(0, 0, 2021, 2);
        assert_eq!(day, 25);
        assert_eq!(is_in_month, -1);

        let (day, is_in_month) = position_to_day(0, 1, 2021, 2);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, 0);

        let (day, is_in_month) = position_to_day(6, 4, 2021, 2);
        assert_eq!(day, 28);
        assert_eq!(is_in_month, 0);

        let (day, is_in_month) = position_to_day(0, 5, 2021, 2);
        assert_eq!(day, 1);
        assert_eq!(is_in_month, 1);
    }

    #[test]
    fn is_leap_year_test() {
        assert_eq!(is_leap_year(2020), true);
        assert_eq!(is_leap_year(2019), false);
        assert_eq!(is_leap_year(2021), false);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(1000), false);
    }

    #[test]
    fn num_days_of_month_test() {
        assert_eq!(num_days_of_month(2020, 1), 31);
        assert_eq!(num_days_of_month(2020, 2), 29);
        assert_eq!(num_days_of_month(2019, 2), 28);
        assert_eq!(num_days_of_month(2020, 3), 31);
        assert_eq!(num_days_of_month(2020, 4), 30);
        assert_eq!(num_days_of_month(2020, 5), 31);
        assert_eq!(num_days_of_month(2020, 6), 30);
        assert_eq!(num_days_of_month(2020, 7), 31);
        assert_eq!(num_days_of_month(2020, 8), 31);
        assert_eq!(num_days_of_month(2020, 9), 30);
        assert_eq!(num_days_of_month(2020, 10), 31);
        assert_eq!(num_days_of_month(2020, 11), 30);
        assert_eq!(num_days_of_month(2020, 12), 31);
    }
}