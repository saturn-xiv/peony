extern crate tokio;

async fn hello(ttl: u64) {
    tokio::time::sleep(tokio::time::Duration::from_secs(ttl)).await;
    println!("hi, worker!");
}

#[test]
fn test_tokio() {
    tokio::join!(
        sleep_then_print(1),
        sleep_then_print(2),
        sleep_then_print(3),
    );
}
