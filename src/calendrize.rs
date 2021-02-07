use chrono::*;

pub fn calendrize(date: NaiveDate) -> Vec<[u32; 7]>{
  let mut out: Vec<[u32; 7]> = Vec::with_capacity(7);
    println!("------------");
    let year = date.year(); 
    let month = date.month();
    let mut first = NaiveDate::from_ymd(year, month, 1).weekday().num_days_from_monday();
    let days = NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1)).pred().day();
    println!("{}",days);
    let mut i: u32 = 0;
    let mut week: [u32; 7] = [0;7];
    loop {
        let mut j=0 as usize;
        loop {
            week = [0;7];
            loop {
                if j < 7 {
                    break;
                } else if j < first as usize {
                    week[j] = i;
                    first = 0;
                } else {
                    break;
                };
            j = j + 1;
            }
            break
        }
        out.push(week);
        i = i + 1;
        println!("{}",i);
        println!("{}",days);
        if i < days {
            break;
        }
    }
    return out;
}

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let date = NaiveDate::parse_from_str("2018-11-07", "%Y-%m-%d").unwrap();
        let actual = calendrize(date);
        assert_eq!(vec![[0,0,0,0,0,0,0],[0,0,0,0,0,0,0]], actual);
    }
}