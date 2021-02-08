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
    fn january() {
        let date = NaiveDate::parse_from_str("2021-01-02", "%Y-%m-%d").unwrap();
        let actual = calendrize(date);
        assert_eq!(vec![[0,0,0,0,0,1,2],[3,4,5,6,7,8,9],[10,11,12,13,14,15,16],[17,18,19,20,21,22,23],[24,25,26,27,28,29,30],[31,0,0,0,0,0,0]], actual);
    }

    #[test]
    fn april() {
        let date = NaiveDate::parse_from_str("2021-02-02", "%Y-%m-%d").unwrap();
        let actual = calendrize(date);
        assert_eq!(vec![[0,0,0,0,1,2,3],[4,5,6,7,8,9,10],[11,12,13,14,15,16,17],[18,19,20,21,22,23,24],[25,26,27,28,29,30,0]], actual);
    }

   #[test]
    fn uruwanaidoshi() {
        let date = NaiveDate::parse_from_str("2021-02-02", "%Y-%m-%d").unwrap();
        let actual = calendrize(date);
        assert_eq!(vec![[0,1,2,3,4,5,6],[7,8,9,10,11,12,13],[14,15,16,17,18,19,20],[21,22,23,24,25,26,27],[28,0,0,0,0,0,0]], actual);
    }

    #[test]
    fn uruudoshi() {
        let date = NaiveDate::parse_from_str("2020-02-02", "%Y-%m-%d").unwrap();
        let actual = calendrize(date);
        assert_eq!(vec![[0,0,0,0,0,0,1],[2,3,4,5,6,7,8],[9,10,11,12,13,14,15],[16,17,18,19,20,21,22],[23,24,25,26,27,28,29]], actual);
    }
}