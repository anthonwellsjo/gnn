use std::collections::HashMap;

use reqwest::{self, Error, Response, Url};
use serde::Deserialize;

static CLIENT_ID: &str = "a12059d5dd1b97f61fcf";

#[derive(Deserialize, Debug)]
struct GHResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: usize,
    interval: usize,
}

pub async fn post() -> Result<(), Error> {
    let mut map = HashMap::new();
    map.insert("client_id", CLIENT_ID);
    let client = reqwest::Client::new();
    let res: Response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .json(&map)
        .send()
        .await?;

    let json: reqwest::Result<GHResponse> = res.json().await;
    println!("{:?}", json);
    Ok(())
}
