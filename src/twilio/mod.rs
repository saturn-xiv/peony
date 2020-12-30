pub mod sms;

use std::default::Default;

use reqwest::{multipart::Form, Client, StatusCode};

use super::errors::{Error, Result};

// https://www.twilio.com/docs/api

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
        let mut form = Form::new()
            .text("To", to.to_string())
            .text("Body", body.to_string())
            .text("From", self.from.clone());
        if let Some(cb) = callback {
            form = form.text("StatusCallback", cb);
        }
        // application/x-www-form-urlencoded
        let res = Client::new()
            .post(&format!(
                "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
                self.account_sid
            ))
            .basic_auth(&self.account_sid, Some(self.auth_token.clone()))
            // .header(AUTHORIZATION, HeaderValue::from_str(&self.credentials())?)
            .multipart(form)
            .send()
            .await?;
        match res.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let it = res.json().await?;
                Ok(it)
            }
            v => Err(Error::Http(v, Some(res.text().await?))),
        }
    }
}
