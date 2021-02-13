use std::ops::Deref;

use chrono::Duration;
use diesel::Connection;
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto,
    errors::{Error, Result},
    i18n::I18n,
    jwt::Jwt,
    orm::postgresql::Connection as Db,
};
use super::super::{
    models::{log::Dao as LogDao, user::Dao as UserDao},
    request::{Action, Token},
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignUp {
    #[validate(email)]
    email: String,
    #[validate(length(min = 2, max = 32))]
    real_name: String,
    #[validate(length(min = 2, max = 32))]
    nickname: String,
    #[validate(length(min = 6))]
    password: String,
}

impl SignUp {
    pub fn execute(&self, db: &Db) -> Result<()> {
        self.validate()?;
        let db = db.deref();
        UserDao::sign_up::<Crypto>(
            db,
            &self.real_name,
            &self.nickname,
            &self.email,
            &self.password,
        )?;
        // TODO
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignInForm {
    #[validate(length(min = 1))]
    user: String,
    #[validate(length(min = 1))]
    password: String,
}

impl SignInForm {
    pub fn execute(&self, db: &Db, jwt: &Jwt, lang: &str, peer: &str) -> Result<String> {
        self.validate()?;
        let db = db.deref();

        let mut user = UserDao::by_nickname(db, &self.user);
        if user.is_err() {
            user = UserDao::by_email(db, &self.user);
        }
        let user = user?;
        user.auth::<Crypto>(&self.password)?;
        user.available()?;

        let uid = user.uid.clone();
        let name = user.real_name.clone();

        db.transaction::<_, Error, _>(move || {
            UserDao::sign_in(db, user.id, peer)?;
            __i18n_l!(db, user.id, peer, lang, "nut.logs.user.sign-in.success")?;
            Ok(())
        })?;
        let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
        let token = jwt.sum(
            None,
            &Token {
                uid,
                sub: name,
                act: Action::SignIn,
                nbf,
                exp,
            },
        )?;

        Ok(token)
    }
}
