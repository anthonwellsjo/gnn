use serde::Deserialize;

use crate::models::Notification;

use super::{ActionResponse,  Session};

impl Notification {
    pub async fn get_many(session: &mut Session, no: Option<String>) -> Vec<Notification> {
        let no = match no.unwrap_or("20".to_owned()).parse::<u8>() {
            Ok(no) => no,
            Err(_) => {
                session.action_responses.push(ActionResponse {
                    message: "Argument wasn't a valid u8".to_owned(),
                    res_type: super::ActionResponseType::Error,
                    content: None
                });
                20
            }
        };
        let access_token = &session.token;

        let client = reqwest::Client::builder()
            .user_agent("curl")
            .build()
            .unwrap();

        let res = client
            .get(
                "https://api.github.com/notifications?all=true&per_page=".to_owned()
                    + &no.to_string(),
            )
            .header("Accept", "application/vnd.github+json")
            .bearer_auth(&access_token)
            .send()
            .await;

        let res = match res {
            Ok(res) => Some(res.json::<Vec<Notification>>().await.unwrap()),
            Err(_) => {
                session.action_responses.push(ActionResponse {
                    message: "Error while getting user...".to_owned(),
                    res_type: super::ActionResponseType::Error,
                    content: None
                });
                None
            }
        };

        match res {
            Some(vec) => {
                session.action_responses.push(ActionResponse {
                    message: "Received notifications".to_owned(),
                    res_type: super::ActionResponseType::Content,
                    content: Some(super::ActionResponseContent { notifications: Some(vec.clone()), thread: None })
                });
                return vec;
            }
            None => todo!(),
        }
    }

    pub async fn fetch_url<Model: std::fmt::Debug + for<'de> serde::Deserialize<'de>>(
        session: &mut Session,
        api_url: &str,
    ) {
        let access_token = &session.token;

        let client = reqwest::Client::builder()
            .user_agent("curl")
            .build()
            .unwrap();

        let res = client
            .get(api_url)
            .header("Accept", "application/vnd.github+json")
            .bearer_auth(&access_token)
            .send()
            .await;

        let res = match res {
            Ok(res) => Some(res.json::<Model>().await.unwrap()),
            Err(_) => {
                session.action_responses.push(ActionResponse {
                    message: "Error while getting user...".to_owned(),
                    res_type: super::ActionResponseType::Error,
                    content: None
                });
                None
            }
        };

        println!("{:?}", res);
    }
}

pub struct Http {}

impl Http {
    pub async fn get<T: for<'de> Deserialize<'de>>(session: &mut Session, url: &str) -> Option<T> {
        let access_token = &session.token;

        let client = reqwest::Client::builder()
            .user_agent("curl")
            .build()
            .unwrap();

        let res = client
            .get(url)
            .header("Accept", "application/json")
            .bearer_auth(&access_token)
            .send()
            .await
            .unwrap();

        match res.status() {
            reqwest::StatusCode::OK => {
                match res.json::<T>().await {
                    Ok(obj) => return Some(obj),
                    Err(err) => {
                        panic!("{}", err)
                    }
                };
            }
            other => {
                session.action_responses.push(ActionResponse {
                    message: other.to_string(),
                    res_type: super::ActionResponseType::Error,
                    content: None
                });
                return None;
            }
        };
    }
}
