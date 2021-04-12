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
/// // Week = [Sun, Mon, Tue, Wed, Thu, Fri, Sat]
/// println!("{:?}", calendarize(date));
/// ```
pub fn calendarize(date: NaiveDate) -> Vec<[u32; 7]> {
    calendarize_with_offset(date, 0)
}

/// Generate a calendar view of the given date's month and offset.
///
/// Each vector element is an array of seven numbers representing weeks (starting on Sundays),
/// and each value is the numeric date.
/// A value of zero means a date that not exists in the current month.
///
/// Offset means the number of days from sunday.
/// For example, 1 means monday, 6 means saturday.
///
/// # Examples
/// ```
/// use chrono::*;
/// use calendarize::calendarize;
///
/// let date = NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap();
/// // Week = [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
/// println!("{:?}", calendarize(date, 1));
/// ```
pub fn calendarize_with_offset(date: NaiveDate, offset: u32) -> Vec<[u32; 7]> {
    let mut monthly_calendar: Vec<[u32; 7]> = Vec::with_capacity(6);
    let year = date.year();
    let month = date.month();
    let num_days_from_sunday = NaiveDate::from_ymd(year, month, 1)
        .weekday()
        .num_days_from_sunday();
    let mut first_date_day;
    if num_days_from_sunday < offset {
        first_date_day = num_days_from_sunday + (7 - offset);
    } else {
        first_date_day = num_days_from_sunday - offset;
    }
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
// Week = [Sun, Mon, Tue, Wed, Thu, Fri, Sat]
fn with_offset_from_sunday() {
    let date = NaiveDate::parse_from_str("2019-11-01", "%Y-%m-%d").unwrap();
    let actual = calendarize_with_offset(date, 0);
    assert_eq!(
        vec![
            [0, 0, 0, 0, 0, 1, 2],
            [3, 4, 5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14, 15, 16],
            [17, 18, 19, 20, 21, 22, 23],
            [24, 25, 26, 27, 28, 29, 30],
        ],
        actual
    );
}

#[test]
// Week = [Mon, Tue, Wed, Thu, Fri, Sat, Sun]
fn with_offset_from_monday() {
    let date = NaiveDate::parse_from_str("2019-11-01", "%Y-%m-%d").unwrap();
    let actual = calendarize_with_offset(date, 1);
    assert_eq!(
        vec![
            [0, 0, 0, 0, 1, 2, 3],
            [4, 5, 6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15, 16, 17],
            [18, 19, 20, 21, 22, 23, 24],
            [25, 26, 27, 28, 29, 30, 0],
        ],
        actual
    );
}

#[test]
// Week = [Sat, Sun, Mon, Tue, Wed, Thu, Fri]
fn with_offset_from_saturday() {
    let date = NaiveDate::parse_from_str("2019-11-01", "%Y-%m-%d").unwrap();
    let actual = calendarize_with_offset(date, 6);
    assert_eq!(
        vec![
            [0, 0, 0, 0, 0, 0, 1],
            [2, 3, 4, 5, 6, 7, 8],
            [9, 10, 11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20, 21, 22],
            [23, 24, 25, 26, 27, 28, 29],
            [30, 0 ,0,0,0,0,0]
        ],
        actual
    );
}

#[test]
// Week = [Sun, Mon, Tue, Wed, Thu, Fri, Sat]
fn with_offset_from_sunday_with7() {
    let date = NaiveDate::parse_from_str("2019-11-01", "%Y-%m-%d").unwrap();
    let actual = calendarize_with_offset(date, 7);
    assert_eq!(
        vec![
            [0, 0, 0, 0, 0, 1, 2],
            [3, 4, 5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14, 15, 16],
            [17, 18, 19, 20, 21, 22, 23],
            [24, 25, 26, 27, 28, 29, 30],
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
