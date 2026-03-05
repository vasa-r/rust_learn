use chrono::{Local, NaiveDate, Utc};
pub fn chrono_test() {
    let utc_now = Utc::now();
    println!("{}", utc_now);
    let local_now = Local::now();
    println!("{}", local_now);
    println!("format date: {}", local_now.format("%d/%m/%y %H:%M"));

    let date = NaiveDate::parse_from_str("2026-03-05", "%Y-%m-%d");
    match date {
        Ok(date) => println!("{}", date),
        Err(e) => println!("{:?}", e),
    }

    // println!("{}", date);
}
