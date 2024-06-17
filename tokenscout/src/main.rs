use dotenv::dotenv;
use ethers::providers::{Provider, Ws};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let ws_url = env::var("WS_TOKENSCOUT_API_KEY").expect("WS_TOKENSCOUT_API_KEY not set");

    let _provider = Provider::<Ws>::connect(ws_url.as_str())
        .await
        .expect("could not connect to the provider");
}
