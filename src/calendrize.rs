use chrono::*;

pub fn calendrize(date: NaiveDate) -> (NaiveDate, u32){
    println!("------------");
    let year = date.year(); 
    let month = date.month();
    let first = NaiveDate::from_ymd(year, month, 1);
    let days = NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1)).pred().day();
    println!("{}",days);
    
    return (first, days);
}

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let date = NaiveDate::parse_from_str("2018-11-07", "%Y-%m-%d").unwrap();
        let actual = calendrize(date);
        assert_eq!((NaiveDate::parse_from_str("2018-12-01", "%Y-%m-%d").unwrap(), 12), actual);
    }
}