use crate::db::{self, AuthRequest};

use arboard::Clipboard;
use open;
use reqwest::{self, StatusCode};
use serde::Deserialize;
use std::{collections::HashMap, thread, time::Duration};
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

pub async fn token_is_valid(token: &str) -> bool {
    let res = validate_session(token).await;
    match res {
        Ok(validation) => validation,
        Err(err) => {
            panic!("{}", err)
        }
    }
}

pub async fn create_session() -> AuthRequest {
    let last_session = match db::Auth::get_last_session() {
        Ok(it) => it,
        Err(err) => {
            panic!("Error while getting last session: {}", err)
        }
    };

    match last_session {
        Some(auth_req) => {
            if !token_is_valid(&auth_req.access_token.unwrap()).await {
                authenticate().await;
            }}
        None => {
            println!("found no session");
            authenticate().await;
        }
    };
    db::Auth::get_last_session().unwrap().unwrap()
}

async fn validate_session(token: &str) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::builder().user_agent("curl").build()?;

    let res = client
        .get("https://api.github.com/user")
        .bearer_auth(&token)
        .send()
        .await?;

    match res.status() {
        StatusCode::OK | StatusCode::FOUND => return Ok(true),
        _ => return Ok(false),
    }
}

pub async fn authenticate() -> AuthRequest {
    let res = step_one().await;
    println!("step two");
    step_two(&res.user_code).await;
    let session = step_three(&res).await;

    let res = match db::Auth::save_token(session) {
        Ok(session) => session,
        Err(err) => panic!("{:?}", err),
    };

    res
}

///App requests the device and user verification codes from GitHub
pub async fn step_one() -> StepOneResponse {
    println!("step one");
    let mut json = HashMap::new();
    json.insert("client_id", CLIENT_ID);
    reqwest::Client::new()
        .post("https://github.com/login/device/code?scope=notifications%20repo")
        .header("Accept", "application/json")
        .json(&json)
        .send()
        .await
        .expect("Auth step one req")
        .json::<StepOneResponse>()
        .await
        .expect("Auth step json to parse")
}

///Prompt the user to enter the user code in a browser
pub async fn step_two(user_code: &str) {
    println!("Please log in first.");
    println!("{:?}", user_code);
    println!(
        "The cod above has been copied to your clipboard. Paste it in the browser window opening."
    );

    thread::sleep(Duration::from_secs(2));

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(user_code).unwrap();
    open::that("https://github.com/login/device").unwrap();
}

// /// Poll GitHub to check if the user authorized the device
pub async fn step_three(res: &StepOneResponse) -> AuthRequest {
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
            .await
            .expect("Auth step three req")
            .json::<StepThreeResponse>()
            .await
            .expect("Auth step three to parse");

        if let Some(err) = &res.error {
            println!("{:?}", err);
        }

        if let Some(ref error_description) = res.error_description {
            println!("{}", error_description);
        } else {
            return AuthRequest {
                access_token: res.access_token,
                token_type: res.token_type,
                scope: res.scope,
            };
        }

        println!("end of loop, waiting");
        interval.tick().await;
        interval.tick().await;
    }
}
