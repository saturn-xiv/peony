extern crate chrono;
extern crate peony;

use chrono::{DateTime, Local};

#[test]
fn test_ntp_fetch() {
    // "time.google.com"
    // "pool.ntp.org"
    let time = peony::sys::ntp::Response::fetch("0.us.pool.ntp.org", None).unwrap();
    println!("{:?}", time);
    let time: DateTime<Local> = time.into();
    println!("{:?}", time);
}
