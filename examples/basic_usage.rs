use uprusty::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    //let token = std::env::var("UP_API_TOKEN").expect("UP_API_TOKEN environment variable not set");
    dotenv().ok();
    let token = std::env::var("UP_API_TOKEN").expect("UP_API_TOKEN not found in environment or .env file");

    match Client::new(&token) {
        Ok(client) => {
            println!("Client created successfully!");
            match client.ping().await {
                Ok(ping_response) => {
                    println!("Successfully pinged {:?}", ping_response);
                }
                Err(e) => {
                    eprintln!("Failed to ping: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create client: {:?}", e);
        }
    }
}
