mod auth;

use auth::post;

#[tokio::main]
async fn main() {
    let res = post().await;

    println!("{:?}", res)
    
}
