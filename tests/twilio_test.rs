extern crate actix;
extern crate peony;

async fn sms() -> peony::errors::Result<()> {
    let cfg: peony::env::Config = peony::parser::from_toml("config.toml")?;
    let it = cfg.twilio.sms(env!("TO"), "Hello, peony!", None).await?;
    println!("{:?}", it);
    Ok(())
}

#[test]
fn test_inbound() {}

#[test]
fn test_sms() {
    let mut ctx = actix::System::new("test-sms");
    ctx.block_on(sms()).unwrap();
}
