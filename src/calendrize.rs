use chrono::NaiveDate;

pub fn calendrize(date: NaiveDate) -> NaiveDate{
    println!("------------");
    return date;
}

mod tests {
    use chrono::NaiveDate;
    use super::*;
    #[test]
    fn it_works() {
        let date = NaiveDate::parse_from_str("2018-12-07", "%Y-%m-%d").unwrap();
        let actual = calendrize(date);
        assert_eq!(NaiveDate::parse_from_str("2018-12-01", "%Y-%m-%d").unwrap(), actual);
    }
}