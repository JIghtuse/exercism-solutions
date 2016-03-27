extern crate chrono;

use chrono::{DateTime, Duration, UTC};

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
    let ten = 10i64;
    date + Duration::seconds(ten.pow(9))
}
