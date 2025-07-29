use dotenv::dotenv;
use uprusty::prelude::*;
use uprusty::{TransactionFilters, TransactionStatus};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        std::env::var("UP_API_TOKEN").expect("UP_API_TOKEN not found in environment or .env file");

    match Client::new(&token) {
        Ok(client) => {
            println!("Client created successfully!");

            // List all transactions
            match client.list_transactions(Some(10), None).await {
                Ok(transactions) => {
                    println!("\n=== Recent Transactions (All Accounts) ===");
                    for transaction in &transactions.data {
                        println!(
                            "Transaction: {} - {}",
                            transaction.attributes.description, transaction.attributes.amount.value
                        );
                        println!("  Status: {:?}", transaction.attributes.status);
                        println!("  Created: {}", transaction.attributes.created_at);
                        if let Some(message) = &transaction.attributes.message {
                            println!("  Message: {}", message);
                        }
                        if !transaction.relationships.tags.data.is_empty() {
                            let tags: Vec<&str> = transaction
                                .relationships
                                .tags
                                .data
                                .iter()
                                .map(|tag| tag.id.as_str())
                                .collect();
                            println!("  Tags: {}", tags.join(", "));
                        }
                        println!();
                    }

                    // Get a specific transaction if we have any
                    if let Some(first_transaction) = transactions.data.first() {
                        println!("\n=== Getting specific transaction ===");
                        match client.get_transaction(&first_transaction.id).await {
                            Ok(transaction_response) => {
                                let transaction = &transaction_response.data;
                                println!(
                                    "Retrieved transaction: {}",
                                    transaction.attributes.description
                                );
                                println!(
                                    "  Amount: {} {}",
                                    transaction.attributes.amount.value,
                                    transaction.attributes.amount.currency_code
                                );

                                // Check if it's a foreign transaction
                                if let Some(foreign_amount) = &transaction.attributes.foreign_amount
                                {
                                    println!(
                                        "  Foreign amount: {} {}",
                                        foreign_amount.value, foreign_amount.currency_code
                                    );
                                }

                                // Check for round up
                                if let Some(round_up) = &transaction.attributes.round_up {
                                    println!("  Round up: {}", round_up.amount.value);
                                    if let Some(boost) = &round_up.boost_portion {
                                        println!("  Boost portion: {}", boost.value);
                                    }
                                }

                                // Check for cashback
                                if let Some(cashback) = &transaction.attributes.cashback {
                                    println!(
                                        "  Cashback: {} - {}",
                                        cashback.amount.value, cashback.description
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to get specific transaction: {:?}", e);
                            }
                        }
                    }

                    // List only HELD transactions
                    println!("\n=== HELD Transactions Only ===");
                    let held_filters = TransactionFilters {
                        status: Some(TransactionStatus::Held),
                        ..Default::default()
                    };

                    match client.list_transactions(Some(5), Some(held_filters)).await {
                        Ok(held_transactions) => {
                            if held_transactions.data.is_empty() {
                                println!("No HELD transactions found");
                            } else {
                                for transaction in &held_transactions.data {
                                    println!(
                                        "HELD: {} - {}",
                                        transaction.attributes.description,
                                        transaction.attributes.amount.value
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to list HELD transactions: {:?}", e);
                        }
                    }

                    // List transactions for a specific account
                    match client.list_accounts(None, None, None).await {
                        Ok(accounts) => {
                            if let Some(first_account) = accounts.data.first() {
                                println!(
                                    "\n=== Transactions for Account: {} ===",
                                    first_account.attributes.display_name
                                );

                                match client
                                    .list_account_transactions(&first_account.id, Some(5), None)
                                    .await
                                {
                                    Ok(account_transactions) => {
                                        for transaction in &account_transactions.data {
                                            println!(
                                                "{}: {} {}",
                                                transaction.attributes.description,
                                                transaction.attributes.amount.value,
                                                transaction.attributes.amount.currency_code
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to list account transactions: {:?}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to list accounts: {:?}", e);
                        }
                    }

                    // Example with date filters
                    println!("\n=== Transactions with Date Filters ===");
                    println!("To filter by date range, use ISO 8601 format:");
                    println!("let date_filters = TransactionFilters {{");
                    println!("    since: Some(\"2025-01-01T00:00:00+10:00\".to_string()),");
                    println!("    until: Some(\"2025-01-31T23:59:59+10:00\".to_string()),");
                    println!("    ..Default::default()");
                    println!("}};");

                    // Example with category filter
                    println!("\n=== Filter by Category ===");
                    println!("To filter by category:");
                    println!("let category_filters = TransactionFilters {{");
                    println!("    category: Some(\"groceries\".to_string()),");
                    println!("    ..Default::default()");
                    println!("}};");

                    // Example with tag filter
                    println!("\n=== Filter by Tag ===");
                    println!("To filter by tag:");
                    println!("let tag_filters = TransactionFilters {{");
                    println!("    tag: Some(\"Holiday\".to_string()),");
                    println!("    ..Default::default()");
                    println!("}};");
                }
                Err(e) => {
                    eprintln!("Failed to list transactions: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create client: {:?}", e);
        }
    }
}
