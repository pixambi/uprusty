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

            // List all attachments
            match client.list_attachments(None).await {
                Ok(attachments) => {
                    println!("\n=== All Attachments ===");
                    for attachment in &attachments.data {
                        println!("Attachment ID: {}", attachment.id);
                        if let Some(created_at) = &attachment.attributes.created_at {
                            println!("  Created: {}", created_at);
                        }
                        if let Some(file_extension) = &attachment.attributes.file_extension {
                            println!("  File Extension: {}", file_extension);
                        }
                        if let Some(content_type) = &attachment.attributes.file_content_type {
                            println!("  Content Type: {}", content_type);
                        }
                        println!(
                            "  Related Transaction: {}",
                            attachment.relationships.transaction.data.id
                        );
                        println!(
                            "  File URL Expires: {}",
                            attachment.attributes.file_url_expires_at
                        );
                        println!();
                    }

                    // Get a specific attachment if we have any
                    if let Some(first_attachment) = attachments.data.first() {
                        println!("\n=== Getting specific attachment ===");
                        match client.get_attachment(&first_attachment.id).await {
                            Ok(attachment_response) => {
                                let attachment = &attachment_response.data;
                                println!("Retrieved attachment: {}", attachment.id);
                                if let Some(file_url) = &attachment.attributes.file_url {
                                    println!("File URL: {}", file_url);
                                }
                                println!(
                                    "Related to transaction: {}",
                                    attachment.relationships.transaction.data.id
                                );
                            }
                            Err(e) => {
                                eprintln!("Failed to get specific attachment: {:?}", e);
                            }
                        }
                    }

                    // List attachments with pagination limit
                    println!("\n=== Attachments with Pagination (limit 5) ===");
                    match client.list_attachments(Some(5)).await {
                        Ok(limited_attachments) => {
                            for attachment in &limited_attachments.data {
                                println!(
                                    "Attachment: {} ({})",
                                    attachment.id,
                                    attachment
                                        .attributes
                                        .file_extension
                                        .as_deref()
                                        .unwrap_or("unknown")
                                );
                            }

                            // Show pagination info
                            if limited_attachments.links.next.is_some() {
                                println!("More attachments available (next page exists)");
                            }
                            if limited_attachments.links.prev.is_some() {
                                println!("Previous page available");
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to list limited attachments: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to list attachments: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create client: {:?}", e);
        }
    }
}
