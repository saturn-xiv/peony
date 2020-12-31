extern crate peony;

use chrono::Duration;
use peony::{
    env::Config,
    jwt::Jwt,
    parser::from_toml,
    plugins::nut::request::{Action, Token},
};

#[test]
fn test_hs512() {
    let cfg: Config = from_toml("config.toml").unwrap();
    let uid = "who-am-i";

    let jwt = Jwt::new(cfg.secrets.0.clone());
    let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
    let token = jwt
        .sum(
            None,
            &Token {
                uid: uid.to_string(),
                sub: "hi".to_string(),
                act: Action::SignIn,
                nbf,
                exp,
            },
        )
        .unwrap();
    println!("{}", token);
    let token = jwt.parse::<Token>(&token).unwrap();
    assert_eq!(token.claims.uid, uid);
}
