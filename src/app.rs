pub mod auth;
pub mod bash_driver;
mod misc;
use auth::token_is_valid;

use crate::db::{self, Notification};

#[derive(Debug, PartialEq)]
pub enum Action {
    Init,
    Help,
    Version,
    GetNotifications,
    Inspect,
    Goto,
}
impl Action {
    pub fn from_string(s: &str) -> Option<Action> {
        match s {
            "init" => Some(Action::Init),
            "i" | "inspect" => Some(Action::Inspect),
            "g" | "goto" => Some(Action::Goto),
            "gn" | "get-notifications" => Some(Action::GetNotifications),
            "h" | "help" => Some(Action::Help),
            "v" | "version" => Some(Action::Version),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ActionResponse {
    pub message: String,
    pub res_type: ActionResponseType,
    pub content_type: Option<ContentType>,
    pub notifications: Option<Vec<Notification>>,
}

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Notification,
}

#[derive(Debug, PartialEq)]
pub enum ActionResponseType {
    Error,
    Success,
    Silent,
    Content,
}
pub struct Session {
    pub action_responses: Vec<ActionResponse>,
    pub token: String,
}

impl Session {
    pub fn new(token: String) -> Self {
        Session {
            action_responses: vec![],
            token,
        }
    }

    pub async fn run(&mut self, action: Option<Action>, argument: Option<String>) {
        if !token_is_valid(&self.token).await {
            self.action_responses.push(ActionResponse {
                message: "you are not authenticated".to_string(),
                res_type: ActionResponseType::Error,
                content_type: None,
                notifications: None,
            });
        }
        match action {
            Some(Action::Init) => {
                self.init().await;
            }
            Some(Action::Help) => {
                self.show_help();
            }
            Some(Action::Version) => {
                self.show_version();
            }
            Some(Action::GetNotifications) => {
                self.get_notifications(argument).await;
            }
            Some(Action::Goto) => {
                match argument {
                    Some(arg) => {
                        self.goto_notification(arg).await;
                    }
                    None => {
                        self.action_responses.push(ActionResponse {
                            message: "this action requires an argument.".to_owned(),
                            res_type: ActionResponseType::Error,
                            content_type: None,
                            notifications: None,
                        });
                    }
                };
            }
            Some(Action::Inspect) => {
                match argument {
                    Some(arg) => {
                        self.inspect_notification(arg).await;
                    }
                    None => {
                        self.action_responses.push(ActionResponse {
                            message: "this action requires an argument.".to_owned(),
                            res_type: ActionResponseType::Error,
                            content_type: None,
                            notifications: None,
                        });
                    }
                };
            }
            None => {
                self.action_responses.push(ActionResponse {
                    message: "no action?".to_string(),
                    res_type: ActionResponseType::Error,
                    content_type: None,
                    notifications: None,
                });
            }
        }
    }

    async fn init(&mut self) {
        print!("init");
        match misc::get_user(self).await {
            Some(user) => {
                db::User::save(&user);
                self.action_responses.push(ActionResponse {
                    message: "User set successfully.".to_owned(),
                    res_type: ActionResponseType::Success,
                    content_type: None,
                    notifications: None,
                })
            }
            None => self.action_responses.push(ActionResponse {
                message: "No user found.".to_owned(),
                res_type: ActionResponseType::Error,
                content_type: None,
                notifications: None,
            }),
        }
    }

    async fn get_notifications(&mut self, no: Option<String>) {
        let notifications = Notification::get_many(self, no).await;
        Notification::save_many(self, notifications).await;
    }

    fn show_version(&mut self) {
        self.action_responses.push(ActionResponse {
            message: env!("CARGO_PKG_VERSION").to_string(),
            res_type: ActionResponseType::Silent,
            content_type: None,
            notifications: None,
        });
    }

    async fn goto_notification(&mut self, argument: String) {
        let nots = db::Notification::get_by_id(argument);
        match nots {
            Ok(nots) => {
                open::that(nots.unwrap().first().unwrap().url.as_ref().unwrap()).unwrap();
            }
            Err(err) => self.action_responses.push(ActionResponse {
                message: "Error: ".to_owned() + &err.to_string(),
                res_type: ActionResponseType::Error,
                content_type: None,
                notifications: None,
            }),
        }
    }

    async fn inspect_notification(&self, argument: String) {
        // let nots = db::Notification::get_by_id(argument);
        // match nots{
        //     Ok(nots) => {
        //         self.action_responses.push(ActionResponse { message: "".to_owned(), res_type: ActionResponseType::Content, content_type: Some(ContentType::Notification), notifications: nots});
        //     },
        //     Err(err) => {
        //         self.action_responses.push(ActionResponse { message: "Error: ".to_owned() + &err.to_string(), res_type: ActionResponseType::Error, content_type: None , notifications: None })
        //     },
        // }
        todo!()
    }

    fn show_help(&mut self) {
        self.action_responses.push(ActionResponse {
            message: "
command:        argument:

init            -                   initialize
i, inspect      id                  inspect notification
g, goto         id                  goto notification url in browser
gn, get-not     x?                  get <x> notifications (default 10)
v, version      -                   current version
h, help         -                   what you are doing now
            "
            .to_string(),
            res_type: ActionResponseType::Silent,
            content_type: None,
            notifications: None,
        });
    }
}
