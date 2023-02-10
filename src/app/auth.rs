use crate::db::{AuthSession, self};

use arboard::Clipboard;
use open;
use reqwest::{self, header::HeaderMap};
use serde::Deserialize;
use std::{collections::HashMap, thread, time::Duration, error::Error};
use tokio::time::interval;

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
struct StepThreeResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    scope: Option<String>,
    error_description: Option<String>,
    error: Option<String>,
}

pub async fn has_valid_session() -> bool {
    let session = match db::Auth::get_last_session() {
        Ok(it) => it,
        Err(err) => {panic!("Error while getting last session: {}", err)},
    };
    println!("validate");
    let res = validate_session(session).await;
    match res {
        Ok(validation) => validation,
        Err(err) => {panic!("{}", err)},
    }
}

#[derive(Deserialize, Debug)]
struct ValRes{
    message: Option<String>
}

async fn validate_session(session: Option<AuthSession>) -> Result<bool, reqwest::Error> {

    let access_token = match session {
        Some(s) => s.access_token.unwrap(),
        None => {return Ok(false);},
    };


    println!("gonna api {:?}", access_token);

    println!("{:?}", &access_token);
    let res = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header("Accept", "application/json")
        .header("Authorization", "Bearer ".to_owned() + &access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await;


    println!("got here");

    println!("result: {:?}",res);
    // let res = match res{
    //     Ok(res) => res,
    //     Err(err) => {panic!("{}", err)},
    // };
    //     
    // match res {
    //     Ok(res) => println!("{:?}",res),
    //     Err(err) => println!("{}",err),
    // }
    return Ok(true);

}

pub async fn authenticate() -> Result<(), reqwest::Error> {
    let res = step_one().await?;
    step_two(&res.user_code).await;
    let session = step_three(&res).await?;

    match db::Auth::save_token(session){
        Ok(_) => return Ok(()),
        Err(err) => todo!()
    }
}

///App requests the device and user verification codes from GitHub
pub async fn step_one() -> Result<StepOneResponse, reqwest::Error> {
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
    println!("Please log in first.");
    println!("{:?}", user_code);
    println!("The cod above has been copied to your clipboard. Paste it in the browser window opening.");

    thread::sleep(Duration::from_secs(2));

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(user_code).unwrap();
    open::that("https://github.com/login/device").unwrap();
}

// /// Poll GitHub to check if the user authorized the device
pub async fn step_three(res: &StepOneResponse) -> Result<AuthSession, reqwest::Error> {
    println!("Waiting for authentication...");
    thread::sleep(Duration::from_secs(2));
    let mut json = HashMap::new();
    json.insert("client_id", CLIENT_ID);
    json.insert("device_code", &res.device_code);
    json.insert("grant_type", "urn:ietf:params:oauth:grant-type:device_code");

    let mut interval = interval(Duration::from_secs(6));
    let mut client;
    loop {
        println!("call");
        client = reqwest::Client::new();

        let res = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .json(&json)
            .send()
            .await?
            .json::<StepThreeResponse>()
            .await?;

        if let Some(err) = &res.error {
            println!("{:?}", err);
        }

        if let Some(ref error_description) = res.error_description {
            println!("{}", error_description);
        } else {
            return Ok(AuthSession {
                access_token: res.access_token,
                token_type: res.token_type,
                scope: res.scope,
            });
        }

        println!("end of loop, waiting");
        interval.tick().await;
        interval.tick().await;
    }
}
