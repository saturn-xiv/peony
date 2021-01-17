extern crate peony;

use std::time::Duration;

use peony::tty::{g786, Tty};

#[test]
fn test_rs232() {
    for it in Tty::ports().unwrap() {
        println!("find {}", it);
    }
}
