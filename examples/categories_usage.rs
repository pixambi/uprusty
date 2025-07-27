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

            // List all categories
            match client.list_categories(None).await {
                Ok(categories) => {
                    println!("\n=== All Categories ===");
                    print_categories(&categories.data, 0);

                    // Find parent categories (those with no parent)
                    let parent_categories: Vec<_> = categories
                        .data
                        .iter()
                        .filter(|cat| cat.relationships.parent.data.is_none())
                        .collect();

                    println!("\n=== Parent Categories ===");
                    for parent in &parent_categories {
                        println!("- {} ({})", parent.attributes.name, parent.id);
                    }

                    // Get children of a specific parent category if available
                    if let Some(first_parent) = parent_categories.first() {
                        println!("\n=== Children of '{}' ===", first_parent.attributes.name);
                        match client.list_categories(Some(&first_parent.id)).await {
                            Ok(children) => {
                                for child in &children.data {
                                    println!("- {} ({})", child.attributes.name, child.id);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to get children categories: {:?}", e);
                            }
                        }
                    }

                    // Get a specific category
                    if let Some(category) = categories.data.iter()
                        .find(|cat| cat.relationships.children.data.is_empty() &&
                            cat.relationships.parent.data.is_some()) {
                        println!("\n=== Getting specific category: {} ===", category.id);
                        match client.get_category(&category.id).await {
                            Ok(category_response) => {
                                let cat = &category_response.data;
                                println!("Name: {}", cat.attributes.name);
                                println!("ID: {}", cat.id);

                                if let Some(parent_data) = &cat.relationships.parent.data {
                                    println!("Parent: {}", parent_data.id);
                                }

                                if !cat.relationships.children.data.is_empty() {
                                    println!("Children: {} categories", cat.relationships.children.data.len());
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to get specific category: {:?}", e);
                            }
                        }
                    }

                    // Example of how to categorize a transaction (would need a real transaction ID)
                    println!("\n=== Transaction Categorization Example ===");
                    println!("To categorize a transaction, you would use:");
                    println!("client.categorize_transaction(\"transaction-id\", Some(\"restaurants-and-cafes\")).await?;");
                    println!("\nTo remove a category:");
                    println!("client.categorize_transaction(\"transaction-id\", None).await?;");
                }
                Err(e) => {
                    eprintln!("Failed to list categories: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create client: {:?}", e);
        }
    }
}

fn print_categories(categories: &[CategoryResource], indent: usize) {
    for category in categories {
        let indent_str = "  ".repeat(indent);
        println!("{}• {} ({})", indent_str, category.attributes.name, category.id);

        // Print parent info if available
        if let Some(parent) = &category.relationships.parent.data {
            println!("{}  Parent: {}", indent_str, parent.id);
        }

        // Print children count if any
        if !category.relationships.children.data.is_empty() {
            println!("{}  Children: {} categories", indent_str, category.relationships.children.data.len());
        }
    }
}