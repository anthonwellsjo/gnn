mod auth;

use auth::{auth, has_valid_session};

#[tokio::main]
async fn main() {
    if !has_valid_session().await {
        auth().await;
    }
}
