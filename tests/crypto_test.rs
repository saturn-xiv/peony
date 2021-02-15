extern crate base64;
extern crate peony;

use peony::crypto::{random, ssha512, Aes, Hmac, Password, Secret};

#[test]
fn test_random() {
    for _ in 0..3 {
        println!("random bytes: {:?}", random::bytes(8));
        println!("random string: {}", random::string(8));
        println!("random uuid: {}", random::uuid());
    }
}

#[test]
fn test_ssha512() {
    let salt = random::bytes(8);
    let plain = random::bytes(128);

    println!("ssha512 salt: {:?}", salt);
    println!("ssha512 plain: {:?}", plain);

    let cipher = ssha512::sum(&plain, &salt);
    println!("ssha512 cipher: {}", cipher);
    assert!(ssha512::verify(&cipher, &plain));
}

#[test]
fn test_hmac() {
    let key = random::bytes(24);
    let plain = random::bytes(128);

    let hmac = Hmac::new(&base64::encode(key)).unwrap();

    println!("hmac plain: {:?}", plain);
    let cipher = hmac.sum(&plain).unwrap();
    println!("hmac cipher: {:?}", cipher);
    assert!(hmac.verify(&cipher, &plain));
}

#[test]
fn test_aes() {
    let key = random::bytes(32);
    // let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";

    let aes = Aes::new(&base64::encode(key)).unwrap();

    for plain in vec!["hi", "hello, aes!", "中文"] {
        for i in 1..5 {
            println!("######## {} ########", i);
            println!("aes plain: {:?}", plain);
            let (cipher, salt) = aes.encrypt(&plain.as_bytes()).unwrap();
            println!("aes cipher: {:?}", cipher);
            println!("aes salt: {:?}", salt);

            let value = aes.decrypt(&cipher, &salt).unwrap();
            let value = String::from_utf8(value).unwrap();
            println!("aes decode: {:?}", value);
            assert_eq!(plain, value);
        }
    }
}
