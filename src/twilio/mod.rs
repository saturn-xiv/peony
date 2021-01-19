pub mod sms;
pub mod voice;

use std::collections::HashMap;
use std::default::Default;

use actix_web::http::StatusCode;

use super::{
    errors::{Error, Result},
    request::https_client,
};

// https://www.twilio.com/docs/api
// https://support.twilio.com/hc/en-us/articles/223136047-Configure-a-Twilio-Phone-Number-to-Receive-and-Respond-to-Messages

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub from: String,
    #[serde(rename = "account-sid")]
    pub account_sid: String,
    #[serde(rename = "auth-token")]
    pub auth_token: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            from: "+1xxxxxxxxxx".to_string(),
            account_sid: "Account Sid".to_string(),
            auth_token: "Auth Token".to_string(),
        }
    }
}

impl Config {
    pub fn credentials(&self) -> String {
        format!("{}:{}", self.account_sid, self.auth_token)
    }

    pub async fn sms(
        &self,
        to: &str,
        body: &str,
        callback: Option<String>,
    ) -> Result<sms::Response> {
        // application/x-www-form-urlencoded
        let mut form = HashMap::new();
        form.insert("To", to.to_string());
        form.insert("Body", body.to_string());
        form.insert("From", self.from.clone());
        if let Some(cb) = callback {
            form.insert("StatusCallback", cb);
        }
        let mut res = https_client()?
            .basic_auth(&self.account_sid, Some(&self.auth_token.clone()))
            .finish()
            .post(&format!(
                "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
                self.account_sid
            ))
            .send_form(&form)
            .await?;

        match res.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let it = res.json().await?;
                Ok(it)
            }
            v => {
                let it = res.body().await?;
                let it = std::str::from_utf8(&it)?;
                Err(Error::Http(v, Some(it.to_string())))
            }
        }
    }
}
