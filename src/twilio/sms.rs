use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub account_sid: String,
    pub api_version: String,
    pub body: String,
    pub date_created: String,
    pub date_sent: Option<String>,
    pub date_updated: String,
    pub direction: String,
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
    pub from: String,
    pub messaging_service_sid: Option<String>,
    pub num_media: String,
    pub num_segments: String,
    pub price: Option<String>,
    pub price_unit: Option<String>,
    pub sid: String,
    pub status: String,
    pub subresource_uris: HashMap<String, String>,
    pub to: String,
    pub uri: String,
}

// https://www.twilio.com/docs/usage/webhooks/sms-webhooks
// Your status delivery URL will receive an HTTP POST request with the application/x-www-form-urlencoded content type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackForm {
    #[serde(rename(deserialize = "To"))]
    pub to: String,
    #[serde(rename(deserialize = "AccountSid"))]
    pub account_sid: String,
    #[serde(rename(deserialize = "ApiVersion"))]
    pub api_version: String,
    #[serde(rename(deserialize = "MessageStatus"))]
    pub message_status: String,
    #[serde(rename(deserialize = "SmsSid"))]
    pub sms_sid: String,
    #[serde(rename(deserialize = "From"))]
    pub from: String,
    #[serde(rename(deserialize = "MessageSid"))]
    pub message_sid: String,
    #[serde(rename(deserialize = "SmsStatus"))]
    pub sms_status: String,
    #[serde(rename(deserialize = "Body"))]
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InboundForm {
    #[serde(rename(deserialize = "ToCountry"))]
    pub to_country: String,
    #[serde(rename(deserialize = "ToState"))]
    pub to_state: String,
    #[serde(rename(deserialize = "SmsMessageSid"))]
    pub sms_message_sid: String,
    #[serde(rename(deserialize = "NumMedia"))]
    pub num_media: String,
    #[serde(rename(deserialize = "ToCity"))]
    pub to_city: String,
    #[serde(rename(deserialize = "FromZip"))]
    pub from_zip: String,
    #[serde(rename(deserialize = "SmsSid"))]
    pub sms_sid: String,
    #[serde(rename(deserialize = "FromState"))]
    pub from_state: String,
    #[serde(rename(deserialize = "SmsStatus"))]
    pub sms_status: String,
    #[serde(rename(deserialize = "FromCity"))]
    pub from_city: String,
    #[serde(rename(deserialize = "Body"))]
    pub body: String,
    #[serde(rename(deserialize = "FromCountry"))]
    pub from_country: String,
    #[serde(rename(deserialize = "To"))]
    pub to: String,
    #[serde(rename(deserialize = "ToZip"))]
    pub to_zip: String,
    #[serde(rename(deserialize = "NumSegments"))]
    pub num_segments: String,
    #[serde(rename(deserialize = "MessageSid"))]
    pub message_sid: String,
    #[serde(rename(deserialize = "AccountSid"))]
    pub account_sid: String,
    #[serde(rename(deserialize = "From"))]
    pub from: String,
    #[serde(rename(deserialize = "ApiVersion"))]
    pub api_version: String,
}
