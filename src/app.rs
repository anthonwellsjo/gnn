mod setup;
pub mod auth;
use auth::token_is_valid;


#[derive(Debug, PartialEq)]
pub enum Action {
    Init,
    Help,
    Version,
}
impl Action {
    pub fn from_string(s: &str) -> Option<Action> {
        match s {
            "s" | "setup" => Some(Action::Init),
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
    pub token: String
}

impl Session {
    pub fn new(token: String) -> Self {
        Session {
            action_responses: vec![],
            token
        }
    }

    pub async fn run(&mut self, action: Option<Action>, argument: Option<String>) {
        if !token_is_valid(&self.token).await {
            self.action_responses.push(ActionResponse {
                message: "you are not authenticated".to_string(),
                res_type: ActionResponseType::Error,
                content_type: None,
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
            Some(Action::Init) => {
                self.init();
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

    async fn init(&mut self) {
        print!("init");
        let user = setup::get_user(self).await;       
        print!("user is {:?}", user);
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
