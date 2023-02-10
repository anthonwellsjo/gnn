mod setup;
pub mod auth;
use auth::has_valid_session;

#[derive(Debug, PartialEq)]
pub enum Action {
    Setup,
    Help,
    Version,
}
impl Action {
    pub fn from_string(s: &str) -> Option<Action> {
        match s {
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
}

#[derive(Debug, PartialEq)]
pub enum ContentType {
    RepositoryComment,
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
}

impl Session {
    pub fn new() -> Self {
        Session {
            action_responses: vec![],
        }
    }

    pub async fn run(&mut self, action: Option<Action>, argument: Option<String>) {
        if !has_valid_session().await {
            self.action_responses.push(ActionResponse {
                message: "you are not authenticated".to_string(),
                res_type: ActionResponseType::Error,
                content_type: None,
            });
        }
        match action {
            Some(Action::Help) => {
                self.show_help();
            }
            Some(Action::Version) => {
                self.show_version();
            }
            Some(Action::Setup) => {
                self.setup();
            }
            None => {
                self.action_responses.push(ActionResponse {
                    message: "no action?".to_string(),
                    res_type: ActionResponseType::Error,
                    content_type: None,
                });
            }
        }
    }

    fn setup(&self) {
        let user = setup::get_user();       
        todo!()
    }

    fn show_version(&mut self) {
        self.action_responses.push(ActionResponse {
            message: env!("CARGO_PKG_VERSION").to_string(),
            res_type: ActionResponseType::Silent,
            content_type: None,
        });
    }

    fn show_help(&mut self) {
        self.action_responses.push(ActionResponse {
            message: "
command:        argument:

start           -                   start crl daemon
s, set          crl id              sets crl to os clipboard
health          -                   check daemon health
k, kill         -                   kill crl daemon
l, list         -, limit            lists crls 
c, clean        -                   deletes all crl
g, get          query               queries crls and lists them
h, help         -                   what you are doing now
            "
            .to_string(),
            res_type: ActionResponseType::Silent,
            content_type: None,
        });
    }

}
