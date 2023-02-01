use std::collections::HashMap;

use reqwest::{self, Error};
use serde::Deserialize;
use arw_brr;

static CLIENT_ID: &str = "a12059d5dd1b97f61fcf";

#[derive(Deserialize, Debug)]
pub struct StepOneResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: usize,
    interval: usize,
}

pub async fn auth() -> Result<(), Error>{
    let res = step_one().await?;


    println!("Enter {:?} at https://github.com/login/device", res.user_code);
    Ok(())
}

///App requests the device and user verification codes from GitHub
pub async fn step_one() -> Result<StepOneResponse, Error> {
    let mut map = HashMap::new();
    map.insert("client_id", CLIENT_ID);
    reqwest::Client::new()
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .json(&map)
        .send()
        .await?
        .json::<StepOneResponse>().await
}

///Prompt the user to enter the user code in a browser
pub async fn step_two() -> Result<StepOneResponse, Error> {

}

