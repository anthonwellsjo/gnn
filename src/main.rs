mod auth;

use auth::auth;

#[tokio::main]
async fn main() {
    auth().await;
}
