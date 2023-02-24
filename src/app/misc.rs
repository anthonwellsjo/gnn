use super::{ActionResponse, ContentType, Session};
use crate::db::{Notification, User};

pub async fn get_user(session: &mut Session) -> Option<User> {
    let access_token = &session.token;

    let client = reqwest::Client::builder()
        .user_agent("curl")
        .build()
        .unwrap();

    let res = client
        .get("https://api.github.com/user")
        .header("Accept", "application/json")
        .bearer_auth(&access_token)
        .send()
        .await;

    let res: Option<User> = match res {
        Ok(res) => Some(res.json::<User>().await.unwrap()),
        Err(_) => {
            session.action_responses.push(ActionResponse {
                message: "Error while getting user...".to_owned(),
                res_type: super::ActionResponseType::Error,
                content_type: None,
                notifications: None,
            });
            None
        }
    };
    res
}

impl Notification {
    pub async fn get_many(session: &mut Session, no: Option<String>) -> Vec<Notification> {
        let no = match no.unwrap_or("20".to_owned()).parse::<u8>() {
            Ok(no) => no,
            Err(_) => {
                session.action_responses.push(ActionResponse {
                    message: "Argument wasn't a valid u8".to_owned(),
                    res_type: super::ActionResponseType::Error,
                    content_type: None,
                    notifications: None,
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
                    content_type: None,
                    notifications: None,
                });
                None
            }
        };

        match res {
            Some(vec) => {
                session.action_responses.push(ActionResponse {
                    message: "Received notifications".to_owned(),
                    res_type: super::ActionResponseType::Content,
                    content_type: Some(ContentType::Notification),
                    notifications: Some(vec.clone()),
                });
                return vec;
            }
            None => todo!(),
        }
    }

    pub fn get_spec_id(id: &str) -> String {
        let len = id.len();
        id[len - 3..].to_owned()
    }
}
