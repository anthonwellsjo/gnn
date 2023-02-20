use serde::Deserialize;

use crate::db::AuthRequest;

use super::{ActionResponse, Session};

#[derive(Deserialize, Debug)]
pub struct User{
  name: String,
  email: String,
  login: String,
  avatar_url: String,
  html_url: String,
  subscriptions_url: String,
  organizations_url: String,
  repos_url: String,
  events_url: String,
  received_events_url: String
}

pub async fn get_user(session: &mut Session) -> Option<User> {
    let access_token = &session.token;

    let client = reqwest::Client::builder().user_agent("curl").build().unwrap();

    let res = client
        .get("https://api.github.com/user")
        .bearer_auth(&access_token)
        .header("Accept", "application/json")
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
