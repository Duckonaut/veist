use super::api::client::{send_get, APP_CLIENT};
use std::path::Path;
use std::fs::File;
use std::io::{Write, Cursor};
use log::error;
use serde_json::Value;

const CACHED_DB: &str = "cache/manifest.db";

pub async fn cache() {
    let mut result = send_get("Destiny2/Manifest").await;
    if result == Value::Null {
        error!("Failed to access the manifest list.");
        return;
    }

    let url = match result["mobileWorldContentPaths"]["en"].take() {
        Value::String(u) => u,
        _ => {
            error!("Failed to read the manifest list.");
            return;
        }
    };

    let mut file = match File::create(Path::new(CACHED_DB)) {
        Ok(f) => f,
        Err(why) => {
            error!("Failed to create db cache: {}", why);
            return;
        }
    };

    let response = match APP_CLIENT.get(format!("https://bungie.net{}", url))
        .send()
        .await {
            Ok(r) => r,
            Err(why) => {
                error!("Failed to download db cache: {}", why);
                return;
            }
        };

    let mut content = Cursor::new(match response.bytes().await {
        Ok(b) => b,
        Err(why) => {
            error!("Failed to access db bytes cache: {}", why);
            return;
        }
    });

    match std::io::copy(&mut content, &mut file) {
        Ok(_) => (),
        Err(why) => {
            error!("Failed to write db cache: {}", why);
            return;
        }
    };
}