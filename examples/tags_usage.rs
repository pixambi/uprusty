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

            // List all tags
            match client.list_tags(None).await {
                Ok(tags) => {
                    println!("\n=== All Tags ===");
                    if tags.data.is_empty() {
                        println!(
                            "No tags found. Tags are created when you add them to transactions."
                        );
                    } else {
                        for tag in &tags.data {
                            println!("Tag: \"{}\"", tag.id);
                            if let Some(links) = &tag.relationships.transactions.links {
                                println!("  Transactions URL: {}", links.related);
                            }
                        }
                    }

                    // List tags with pagination
                    println!("\n=== Tags with Pagination (limit 10) ===");
                    match client.list_tags(Some(10)).await {
                        Ok(paginated_tags) => {
                            println!("Showing {} tags", paginated_tags.data.len());
                            for tag in &paginated_tags.data {
                                println!("- {}", tag.id);
                            }

                            // Show pagination info
                            if paginated_tags.links.next.is_some() {
                                println!("\nMore tags available (next page exists)");
                            }
                            if paginated_tags.links.prev.is_some() {
                                println!("Previous page available");
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to list tags with pagination: {:?}", e);
                        }
                    }

                    // Example of how to add/remove tags from transactions
                    println!("\n=== Managing Transaction Tags ===");
                    println!("To add tags to a transaction:");
                    println!("client.add_tags_to_transaction(");
                    println!("    \"transaction-id\",");
                    println!("    vec![\"Holiday\", \"Queensland\", \"Beach Trip\"]");
                    println!(").await?;");

                    println!("\nTo remove tags from a transaction:");
                    println!("client.remove_tags_from_transaction(");
                    println!("    \"transaction-id\",");
                    println!("    vec![\"Queensland\"]");
                    println!(").await?;");

                    println!("\nNote: You can have up to 6 tags per transaction.");
                    println!("Tags are automatically created when you add them to a transaction.");

                    // If you have some existing tags, show some examples
                    if !tags.data.is_empty() {
                        println!("\n=== Example Tag Names You Could Use ===");
                        let example_tags = vec![
                            "Work Expense",
                            "Tax Deductible",
                            "Holiday",
                            "Subscription",
                            "Gift",
                            "Emergency",
                            "Monthly Bill",
                            "Entertainment",
                        ];

                        println!("Some tag ideas based on common usage:");
                        for (i, tag_idea) in example_tags.iter().enumerate() {
                            if i < 4 {
                                println!("- {}", tag_idea);
                            }
                        }
                        println!("...and more!");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to list tags: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create client: {:?}", e);
        }
    }
}
