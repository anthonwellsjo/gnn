use super::{ActionResponse, Session, ContentType};
use crate::db::{User, Notification};

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
                notifications: None
            });
            None
        }
    };
    res
}

pub async fn get_notifications(session: &mut Session) {
    let access_token = &session.token;

    let client = reqwest::Client::builder()
        .user_agent("curl")
        .build()
        .unwrap();

    let res = client
        .get("https://api.github.com/notifications?all=true")
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
                notifications: None
            });
            None
        }
    };

    match res {
        Some(vec) => session.action_responses.push(ActionResponse {
            message: "Received notifications".to_owned(),
            res_type: super::ActionResponseType::Content,
            content_type: Some(ContentType::Notification),
            notifications: Some(vec)
        }),
        None => todo!(),
    }
}
