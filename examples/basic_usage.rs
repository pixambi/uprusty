use uprusty::*;

#[tokio::main]
async fn main() {
    match Client::new("your_api_token_here") {
        Ok(client) => {
            println!("Client created successfully!");
            // Use your client here for API calls
        }
        Err(e) => {
            eprintln!("Failed to create client: {}", e);
        }
    }
}

