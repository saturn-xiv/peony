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
