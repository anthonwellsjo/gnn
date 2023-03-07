pub mod auth;
pub mod bash_driver;
pub mod misc;
use arw_brr::verify_argument_type;
use auth::token_is_valid;

use crate::{
    app::misc::Http,
    db::{self},
    models::{LocalNotification, Notification, Thread, User, DetailedNotification},
};

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
    pub content: Option<ActionResponseContent>,
}

#[derive(Debug)]
pub struct ActionResponseContent {
    pub notifications: Option<Vec<Notification>>,
    pub d_not: Option<DetailedNotification>,
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
                content: None,
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
                            content: None,
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
                            content: None,
                        });
                    }
                };
            }
            None => {
                self.action_responses.push(ActionResponse {
                    message: "no action?".to_string(),
                    res_type: ActionResponseType::Error,
                    content: None,
                });
            }
        }
    }

    async fn init(&mut self) {
        print!("init");
        match Http::get::<User>(self, "https://api.github.com/user").await {
            Some(user) => {
                User::save(&user);
                self.action_responses.push(ActionResponse {
                    message: "User set successfully.".to_owned(),
                    res_type: ActionResponseType::Success,
                    content: None,
                })
            }
            None => self.action_responses.push(ActionResponse {
                message: "No user found.".to_owned(),
                res_type: ActionResponseType::Error,
                content: None,
            }),
        }
    }

    async fn get_notifications(&mut self, no: Option<String>) {
        let no = verify_argument_type::<u8>(no, 20);
        // let notifications = Notification::get_many(self, no).await;
        let notifications = Http::get::<Vec<Notification>>(
            self,
            &("https://api.github.com/notifications?all=true&per_page=".to_owned()
                + &no.to_string()),
        )
        .await;

        match notifications {
            Some(nots) => {
                LocalNotification::save_many(self, nots.clone()).await;
                self.action_responses.push(ActionResponse {
                    message: "".to_owned(),
                    res_type: ActionResponseType::Content,
                    content: Some(ActionResponseContent {
                        notifications: Some(nots),
                        d_not: None,
                    }),
                })
            }
            None => todo!(),
        }
    }

    fn show_version(&mut self) {
        self.action_responses.push(ActionResponse {
            message: env!("CARGO_PKG_VERSION").to_string(),
            res_type: ActionResponseType::Silent,
            content: None,
        });
    }

    async fn goto_notification(&mut self, argument: String) {
        let not = LocalNotification::get_by_id(argument);
        println!("{:?}", not);
        match not {
            Ok(not) => {
                Notification::fetch_url::<Thread>(self, &not.url).await;
                // open::that(nots.unwrap().first().unwrap().url.as_ref().unwrap()).unwrap();
            }
            Err(err) => self.action_responses.push(ActionResponse {
                message: "Error: ".to_owned() + &err.to_string(),
                res_type: ActionResponseType::Error,
                content: None,
            }),
        }
    }

    async fn inspect_notification(&mut self, argument: String) {
        let not = LocalNotification::get_by_id(argument).unwrap();

        match not.get_detailed(self).await {
            Ok(t) => self.action_responses.push(ActionResponse {
                message: "".to_owned(),
                res_type: crate::app::ActionResponseType::Content,
                content: Some(ActionResponseContent {
                    d_not: Some(t),
                    notifications: None,
                }),
            }),
            Err(msg) => self.action_responses.push(ActionResponse {
                message: msg,
                res_type: crate::app::ActionResponseType::Error,
                content: None,
            }),
        }
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
            content: None
        });
    }
}
