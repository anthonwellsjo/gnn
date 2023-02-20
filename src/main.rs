mod db;
mod app;
mod models;

use app::auth::create_session;

#[tokio::main]
async fn main() {

    let session = create_session().await;

    let action = arw_brr::get_argument_at(0).unwrap();
    let action = app::Action::from_string(&action);
    let argument = arw_brr::get_argument_at(1);

    let mut session = app::Session::new(session.access_token.unwrap()); 
    println!("run");
    session.run(action, argument).await;

    println!("after run");
    for res in session.action_responses.iter(){
        // display_action_response(res);
        println!("{:?}", res);
    }

}
