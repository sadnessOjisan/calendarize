use chrono::*;

/// Generate a calendar view of the given date's month.
///
/// Each vector element is an array of seven numbers representing weeks (starting on Sundays),
/// and each value is the numeric date.
/// A value of zero means a date that not exists in the current month.
///
/// # Examples
/// ```
/// use chrono::*;
/// use calendarize::calendarize;
///
/// let date = NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap();
/// println!("{:?}", calendarize(date));
/// ```
pub fn calendarize(date: NaiveDate) -> Vec<[u32; 7]> {
    // Q: NaiveDate を引数にとる I/F ってOK?
    let mut monthly_calendar: Vec<[u32; 7]> = Vec::with_capacity(6);
    let year = date.year();
    let month = date.month();
    let mut first_date_day = NaiveDate::from_ymd(year, month, 1)
        .weekday()
        .num_days_from_sunday();
    let end_date = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day();

    let mut date: u32 = 0;
    while date < end_date {
        let mut week: [u32; 7] = [0; 7];
        for day in first_date_day..7 {
            date += 1;
            week[day as usize] = date;

            if date >= end_date {
                break;
            }
        }
        first_date_day = 0;

        monthly_calendar.push(week);
    }

    monthly_calendar
}

#[test]
fn january() {
    let date = NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap();
    let actual = calendarize(date);
    assert_eq!(
        vec![
            [0, 0, 0, 0, 0, 1, 2],
            [3, 4, 5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14, 15, 16],
            [17, 18, 19, 20, 21, 22, 23],
            [24, 25, 26, 27, 28, 29, 30],
            [31, 0, 0, 0, 0, 0, 0]
        ],
        actual
    );
}

#[test]
fn april() {
    let date = NaiveDate::parse_from_str("2021-04-02", "%Y-%m-%d").unwrap();
    let actual = calendarize(date);
    assert_eq!(
        vec![
            [0, 0, 0, 0, 1, 2, 3],
            [4, 5, 6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24],
            [25, 26, 27, 28, 29, 30, 0]
        ],
        actual
    );
}

#[test]
fn uruudoshi() {
    let date = NaiveDate::parse_from_str("2020-02-02", "%Y-%m-%d").unwrap();
    let actual = calendarize(date);
    assert_eq!(
        vec![
            [0, 0, 0, 0, 0, 0, 1],
            [2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20, 21, 22],
            [23, 24, 25, 26, 27, 28, 29]
        ],
        actual
    );
}

#[test]
fn uruwanaidoshi() {
    let date = NaiveDate::parse_from_str("2021-02-02", "%Y-%m-%d").unwrap();
    let actual = calendarize(date);
    assert_eq!(
        vec![
            [0, 1, 2, 3, 4, 5, 6],
            [7, 8, 9, 10, 11, 12, 13],
            [14, 15, 16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25, 26, 27],
            [28, 0, 0, 0, 0, 0, 0]
        ],
        actual
    );
}
