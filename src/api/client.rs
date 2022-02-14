use crate::api::app_auth::API_KEY;
use lazy_static::lazy_static;
use log::error;
use reqwest::{
    header::{self, HeaderMap},
    Client, ClientBuilder,
};
use serde_json::{json, Value};

use super::API_ROOT;

lazy_static! {
    pub static ref APP_CLIENT: Client = create_client();
}

fn create_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-Key",
        header::HeaderValue::from_str(&API_KEY).expect("API Key not a valid header value."),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("Failed to build client!")
}

pub async fn send_get(api_path: &str) -> Value {
    let result = match APP_CLIENT
        .get(format!("{}{}", API_ROOT, api_path))
        .send()
        .await
    {
        Ok(r) => r,
        Err(why) => {
            error!("Request to {} failed: {}", api_path, why);
            return Value::Null;
        }
    };

    let body = match result.text().await {
        Ok(t) => t,
        Err(why) => {
            error!("Failed to get response body: {}", why);
            return Value::Null;
        }
    };

    let response: Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(why) => {
            error!("API returned invalid JSON: {}", why);
            return Value::Null;
        }
    };

    if response["ErrorCode"] != json!(1usize) {
        error!("API returned error code: {}", response["ErrorCode"]);
    }

    let data: Value = response["Response"].to_owned();

    data
}
