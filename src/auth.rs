use reqwest::{self, Response, Error};

pub async fn post() -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://github.com/login/device/code")
        .send()
        .await?;
    Ok(res)
}
