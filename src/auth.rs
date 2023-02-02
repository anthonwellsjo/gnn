use arboard::Clipboard;
use open;
use reqwest::{self, Error};
use serde::Deserialize;
use std::{collections::HashMap, thread, time::Duration};
use tokio::time::{interval, timeout};

static CLIENT_ID: &str = "a12059d5dd1b97f61fcf";

#[derive(Deserialize, Debug)]
pub struct StepOneResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: usize,
    interval: usize,
}

#[derive(Deserialize, Debug)]
pub struct StepThreeResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

pub async fn auth() -> Result<(), Error> {
    let res = step_one().await?;
    step_two(&res.user_code).await;

    step_three(&res).await?;

    Ok(())
}

///App requests the device and user verification codes from GitHub
pub async fn step_one() -> Result<StepOneResponse, Error> {
    let mut json = HashMap::new();
    json.insert("client_id", CLIENT_ID);
    reqwest::Client::new()
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .json(&json)
        .send()
        .await?
        .json::<StepOneResponse>()
        .await
}

///Prompt the user to enter the user code in a browser
pub async fn step_two(user_code: &str) {
    println!("This is your code: {:?}", user_code);
    println!("It's been copied to your clipboard. Paste it in the browser window opening.");

    thread::sleep(Duration::from_secs(2));

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(user_code).unwrap();
    open::that("https://github.com/login/device").unwrap();
}

// /// Poll GitHub to check if the user authorized the device
pub async fn step_three(res: &StepOneResponse) -> Result<StepThreeResponse, Error> {
    println!("Waiting for authentication...");
    thread::sleep(Duration::from_secs(3));
    let mut json = HashMap::new();
    json.insert("client_id", CLIENT_ID);
    json.insert("device_code", &res.device_code);
    json.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");

    let mut interval = interval(Duration::from_secs(5));
    let client = reqwest::Client::new();
    loop {
        println!("call");

        let res = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .json(&json)
            .send()
            .await?
            .json::<StepThreeResponse>()
            .await;

        println!("{:?}", res);

        match res {
            Ok(res) => {
                println!("token: {:?}", res);
                return Ok(res);
            }
            Err(err) => {
                println!("error {:?}", err)
            }
        }

        interval.tick().await;
    }
}
