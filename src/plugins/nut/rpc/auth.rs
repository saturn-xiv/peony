use std::ops::Deref;

use chrono::Duration;
use diesel::Connection;
use validator::Validate;

use super::super::super::super::{
    crypto::Crypto,
    env::Context,
    errors::{Error, Result},
    i18n::I18n,
    jwt::Jwt,
    protos::{
        auth::{SignInRequest, SignInResponse, SignUpRequest},
        empty::Empty,
    },
};
use super::super::{
    models::{log::Dao as LogDao, user::Dao as UserDao},
    request::{Action, Token},
};
use super::Session;

#[derive(Debug, Validate)]
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

impl From<SignUpRequest> for SignUp {
    fn from(item: SignUpRequest) -> Self {
        Self {
            email: item.email,
            real_name: item.real_name,
            nickname: item.nickname,
            password: item.password,
        }
    }
}

impl SignUp {
    pub fn execute(&self, ctx: &Context, ss: &Session) -> Result<Empty> {
        self.validate()?;
        let db = ctx.db.get()?;
        let db = db.deref();

        Ok(Empty::default())
    }
}

#[derive(Debug, Validate)]
pub struct SignIn {
    #[validate(length(min = 1))]
    login: String,
    #[validate(length(min = 1))]
    password: String,
}

impl From<SignInRequest> for SignIn {
    fn from(item: SignInRequest) -> Self {
        Self {
            login: item.login,
            password: item.password,
        }
    }
}

impl SignIn {
    pub fn execute(&self, ctx: &Context, ss: &Session) -> Result<SignInResponse> {
        self.validate()?;
        let db = ctx.db.get()?;
        let db = db.deref();

        let mut user = UserDao::by_nickname(db, &self.login);
        if user.is_err() {
            user = UserDao::by_email(db, &self.login);
        }
        let user = user?;
        user.auth::<Crypto>(&self.password)?;
        user.available()?;

        let uid = user.uid.clone();
        let name = user.real_name.clone();

        db.transaction::<_, Error, _>(move || {
            UserDao::sign_in(db, user.id, &ss.peer)?;
            __i18n_l!(
                db,
                user.id,
                &ss.peer,
                &ss.locale,
                "nut.logs.user.sign-in.success"
            )?;
            Ok(())
        })?;
        let (nbf, exp) = Jwt::timestamps(Duration::weeks(1));
        let token = ctx.jwt.sum(
            None,
            &Token {
                uid,
                sub: name,
                act: Action::SignIn,
                nbf,
                exp,
            },
        )?;

        Ok(SignInResponse {
            token,
            ..Default::default()
        })
    }
}
