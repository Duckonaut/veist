use std::{fs::File, io::Read};

use reqwest::{self, Error};
use serde_json::{self, Value};
use tokio;

mod api;

const API_ROOT: &str = "https://www.bungie.net/Platform/";

#[tokio::main]
async fn main() {
    let mut file = match File::open("resources/secret/api-key.txt") {
        Ok(f) => f,
        Err(_) => {
            println!("No api key found! Are you sure it's at resources/secret/api-key.txt?");
            return ();
        }
    };

    let mut api_key = String::new();

    match file.read_to_string(&mut api_key) {
        Ok(_) => (),
        Err(why) => println!("Error reading key: {}", why),
    }

    let api_key = api_key.trim();

    match example_req(api_key).await {
        Ok(_) => (),
        Err(why) => println!("Error in request: {}", why),
    }
}

async fn example_req(api_key: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let result = client
        .get(format!("{}App/FirstParty", API_ROOT))
        .header("X-API-Key", api_key)
        .send()
        .await?;

    let body = result.text().await?;

    let json: Value = serde_json::from_str(body.as_str()).expect("Not a JSON!");

    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())
}
