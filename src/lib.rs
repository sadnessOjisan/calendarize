use chrono::*;

/// generate calendar.
pub fn calendarize(date: NaiveDate) -> Vec<[u32; 7]> {
    let mut out: Vec<[u32; 7]> = Vec::with_capacity(7);
    let year = date.year();
    let month = date.month();
    let mut first_date_day = NaiveDate::from_ymd(year, month, 1)
        .weekday()
        .num_days_from_sunday();
    let end_date = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day();
    let mut date: u32 = 0;
    let mut week: [u32; 7] = [0; 7];
    loop {
        let mut day = 0 as usize;
        week = [0; 7];
        loop {
            if day < first_date_day as usize {
                // no op
            } else if day == 7 {
                first_date_day = 0;
                break;
            } else {
                date = date + 1;
                week[day] = date;
                first_date_day = 0;
                if date >= end_date {
                    break;
                }
            };
            day = day + 1;
        }

        out.push(week);
        if date >= end_date {
            break;
        }
    }
    return out;
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
