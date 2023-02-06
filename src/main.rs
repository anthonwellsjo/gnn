mod auth;
mod db;
mod app;

use auth::{authenticate, has_valid_session};

#[tokio::main]
async fn main() {
    if !has_valid_session().await {
        authenticate().await.unwrap();
    }

    let action = arw_brr::get_argument_at(0).unwrap();
    let action = app::Action::from_string(&action);
    let argument = arw_brr::get_argument_at(1);

    let mut session = app::Session::new(); 
    session.run(action, argument);

    for res in session.action_responses.iter(){
        // display_action_response(res);
        println!("{:?}", res);
    }

}
