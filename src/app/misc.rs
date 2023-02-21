use serde::Deserialize;

use crate::db::{AuthRequest, User};

use super::{ActionResponse, Session};


pub async fn get_user(session: &mut Session) -> Option<User> {
    let access_token = &session.token;

    let client = reqwest::Client::builder().user_agent("curl").build().unwrap();

    let res = client
        .get("https://api.github.com/user")
        .header("Accept", "application/json")
        .bearer_auth(&access_token)
        .send()
        .await;

    let res: Option<User> = match res{
        Ok(res) => {
            Some(res.json::<User>()
            .await
            .unwrap())
        },
        Err(_) => {
            session.action_responses.push(ActionResponse { message: "Error while getting user...".to_owned(), res_type: super::ActionResponseType::Error, content_type: None });
            None
        }
    };
    res
}

pub async fn get_notifications(session: &mut Session) -> Option<Vec<Notification>> {

}
