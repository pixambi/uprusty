use dotenv::dotenv;
use uprusty::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        std::env::var("UP_API_TOKEN").expect("UP_API_TOKEN not found in environment or .env file");

    match Client::new(&token) {
        Ok(client) => {
            println!("Client created successfully!");

            // List all accounts
            match client.list_accounts(None, None, None).await {
                Ok(accounts) => {
                    println!("\n=== All Accounts ===");
                    for account in &accounts.data {
                        println!(
                            "Account: {} ({})",
                            account.attributes.display_name, account.id
                        );
                        println!("  Type: {:?}", account.attributes.account_type);
                        println!(
                            "  Balance: {} {}",
                            account.attributes.balance.value,
                            account.attributes.balance.currency_code
                        );
                        println!("  Created: {}", account.attributes.created_at);
                        println!();
                    }

                    // Get a specific account if we have any
                    if let Some(first_account) = accounts.data.first() {
                        println!("\n=== Getting specific account ===");
                        match client.get_account(&first_account.id).await {
                            Ok(account_response) => {
                                let account = &account_response.data;
                                println!("Retrieved account: {}", account.attributes.display_name);
                                println!(
                                    "Current balance: {} {}",
                                    account.attributes.balance.value,
                                    account.attributes.balance.currency_code
                                );
                            }
                            Err(e) => {
                                eprintln!("Failed to get specific account: {:?}", e);
                            }
                        }
                    }

                    // List only savers
                    println!("\n=== Saver Accounts Only ===");
                    match client
                        .list_accounts(Some(10), Some(AccountType::Saver), None)
                        .await
                    {
                        Ok(savers) => {
                            for account in &savers.data {
                                println!(
                                    "Saver: {} - Balance: {} {}",
                                    account.attributes.display_name,
                                    account.attributes.balance.value,
                                    account.attributes.balance.currency_code
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to list saver accounts: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to list accounts: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create client: {:?}", e);
        }
    }
}
