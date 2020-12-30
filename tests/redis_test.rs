extern crate peony;

use std::time::Duration;

use peony::{cache::Provider, env::Config, errors::Result, parser::from_toml};

#[test]
fn test_cache() {
    let cfg: Config = from_toml("config.toml").unwrap();
    let ch = cfg.redis.open().unwrap();
    let val = Provider::get(
        &ch,
        &"test.redis.cache".to_string(),
        || -> Result<String> { Ok("hello, peony!".to_string()) },
        Duration::from_secs(60 * 60),
    )
    .unwrap();
    println!("GET {}", val);
}
