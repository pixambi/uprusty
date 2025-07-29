use dotenv::dotenv;
use uprusty::prelude::*;
use uprusty::webhook::{WebhookEventHandler, events};
use uprusty::{WebhookEventResource, WebhookEventType};

// Example webhook event handler implementation
struct MyWebhookHandler;

impl WebhookEventHandler for MyWebhookHandler {
    fn on_transaction_created(&self, transaction_id: &str, event: &WebhookEventResource) {
        println!("🆕 New transaction created: {}", transaction_id);
        println!("   Event ID: {}", event.id);
        println!("   Created at: {}", event.attributes.created_at);
    }

    fn on_transaction_settled(&self, transaction_id: &str, event: &WebhookEventResource) {
        println!("✅ Transaction settled: {}", transaction_id);
        println!("   Event ID: {}", event.id);
    }

    fn on_transaction_deleted(&self, transaction_id: &str, event: &WebhookEventResource) {
        println!("❌ Transaction deleted: {}", transaction_id);
        println!("   Event ID: {}", event.id);
    }

    fn on_ping(&self, event: &WebhookEventResource) {
        println!("🏓 Webhook ping received!");
        println!("   Event ID: {}", event.id);
        println!("   Webhook ID: {}", event.relationships.webhook.data.id);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        std::env::var("UP_API_TOKEN").expect("UP_API_TOKEN not found in environment or .env file");

    match Client::new(&token) {
        Ok(client) => {
            println!("Client created successfully!");

            // List existing webhooks
            match client.list_webhooks(None).await {
                Ok(webhooks) => {
                    println!("\n=== Existing Webhooks ===");
                    if webhooks.data.is_empty() {
                        println!("No webhooks found.");
                    } else {
                        for webhook in &webhooks.data {
                            println!("Webhook ID: {}", webhook.id);
                            println!("  URL: {}", webhook.attributes.url);
                            if let Some(description) = &webhook.attributes.description {
                                println!("  Description: {}", description);
                            }
                            println!("  Created: {}", webhook.attributes.created_at);
                            println!();
                        }
                    }

                    // Example: Create a new webhook (uncomment to test)
                    /*
                    println!("\n=== Creating New Webhook ===");
                    match client.create_webhook(
                        "https://your-server.com/webhook",
                        Some("Example webhook for testing")
                    ).await {
                        Ok(webhook_response) => {
                            let webhook = &webhook_response.data;
                            println!("✅ Webhook created successfully!");
                            println!("Webhook ID: {}", webhook.id);
                            println!("URL: {}", webhook.attributes.url);

                            // IMPORTANT: Store this secret key securely!
                            if let Some(secret_key) = &webhook.attributes.secret_key {
                                println!("🔑 Secret Key: {}", secret_key);
                                println!("⚠️  Store this secret key securely - it's only shown once!");

                                // Test the webhook with a ping
                                println!("\n=== Testing Webhook with Ping ===");
                                match client.ping_webhook(&webhook.id).await {
                                    Ok(event_response) => {
                                        println!("🏓 Ping sent successfully!");
                                        println!("Event ID: {}", event_response.data.id);
                                        println!("Event Type: {:?}", event_response.data.attributes.event_type);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to ping webhook: {:?}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to create webhook: {:?}", e);
                        }
                    }
                    */

                    // Show webhook logs for existing webhooks
                    if let Some(first_webhook) = webhooks.data.first() {
                        println!("\n=== Webhook Delivery Logs for {} ===", first_webhook.id);
                        match client.list_webhook_logs(&first_webhook.id, Some(5)).await {
                            Ok(logs) => {
                                if logs.data.is_empty() {
                                    println!("No delivery logs found.");
                                } else {
                                    for log in &logs.data {
                                        println!("Log ID: {}", log.id);
                                        println!("  Status: {:?}", log.attributes.delivery_status);
                                        println!("  Created: {}", log.attributes.created_at);

                                        if let Some(response) = &log.attributes.response {
                                            println!("  Response Code: {}", response.status_code);
                                        }
                                        println!();
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to get webhook logs: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to list webhooks: {:?}", e);
                }
            }

            // Demonstrate webhook processing utilities
            println!("\n=== Webhook Processing Examples ===");

            // Example 1: Basic signature verification
            let example_secret = "vWmPcH853fS9OAyaspxqzxHoTKVNlDYByBC7F3NA403ETNL1vpkxHSxUlul2Gs8P";
            let example_body = r#"{"data":{"type":"webhook-events","id":"test","attributes":{"eventType":"PING","createdAt":"2025-07-29T10:00:00+10:00"},"relationships":{"webhook":{"data":{"type":"webhooks","id":"test-webhook"}}}}}"#;

            println!("Example webhook event processing:");
            println!("```rust");
            println!("// 1. Using WebhookHandler for easy processing");
            println!("let handler = WebhookHandler::new(secret_key);");
            println!("let event = handler.process_request(signature_header, raw_body)?;");
            println!();
            println!("// 2. Type-safe event handling");
            println!("let my_handler = MyWebhookHandler;");
            println!("handler.handle_typed_event(signature_header, raw_body, my_handler)?;");
            println!();
            println!("// 3. Manual verification");
            println!("use uprusty::webhook::verify_signature;");
            println!(
                "let is_valid = verify_signature(secret_key, signature_header, raw_body.as_bytes())?;"
            );
            println!("```");

            // Example 2: Webhook handler usage
            let webhook_handler = WebhookHandler::new(example_secret);
            let my_event_handler = MyWebhookHandler;

            println!("\n=== Event Handler Example ===");
            println!("This is how you would handle webhook events in your server:");
            println!();
            println!("```rust");
            println!("// In your webhook endpoint handler:");
            println!("pub async fn handle_webhook(");
            println!("    headers: HeaderMap,");
            println!("    body: String,");
            println!(") -> Result<StatusCode, WebhookError> {{");
            println!("    let signature = extract_signature_from_headers(&headers)?;");
            println!("    let handler = WebhookHandler::new(&secret_key);");
            println!("    let event_handler = MyWebhookHandler;");
            println!("    ");
            println!("    handler.handle_typed_event(&signature, &body, event_handler)?;");
            println!("    ");
            println!("    Ok(StatusCode::OK)");
            println!("}}");
            println!("```");

            // Example 3: Integration patterns
            println!("\n=== Integration Patterns ===");
            println!("📝 Common integration patterns:");
            println!();
            println!("1. **Simple Event Processing**:");
            println!("   - Use WebhookHandler with WebhookEventHandler trait");
            println!("   - Implement only the events you care about");
            println!();
            println!("2. **Advanced Processing**:");
            println!("   - Use parse_and_verify_event for custom logic");
            println!("   - Extract transaction IDs and fetch full transaction data");
            println!();
            println!("3. **Security Best Practices**:");
            println!("   - Always verify signatures before processing");
            println!("   - Store secret keys securely (environment variables)");
            println!("   - Respond with HTTP 200 quickly, process async");
            println!();
            println!("4. **Error Handling**:");
            println!("   - Handle VerificationError and WebhookProcessingError");
            println!("   - Log failures for debugging");
            println!("   - Implement exponential backoff for retries");

            // Example webhook server setup suggestions
            println!("\n=== Server Setup Suggestions ===");
            println!("🚀 For production webhook servers:");
            println!();
            println!("```rust");
            println!("// Example with axum web framework");
            println!("use axum::{{");
            println!("    extract::{{HeaderMap, Request}},");
            println!("    http::StatusCode,");
            println!("    response::Json,");
            println!("    routing::post,");
            println!("    Router,");
            println!("}};");
            println!("use uprusty::webhook::{{WebhookHandler, WebhookEventHandler}};");
            println!();
            println!("async fn webhook_endpoint(");
            println!("    headers: HeaderMap,");
            println!("    body: String,");
            println!(") -> Result<StatusCode, StatusCode> {{");
            println!("    // Extract signature");
            println!("    let signature = headers");
            println!("        .get(\"x-up-authenticity-signature\")");
            println!("        .and_then(|v| v.to_str().ok())");
            println!("        .ok_or(StatusCode::BAD_REQUEST)?;");
            println!();
            println!("    // Process event");
            println!("    let handler = WebhookHandler::new(&secret_key);");
            println!("    let event_handler = MyWebhookHandler;");
            println!();
            println!("    handler");
            println!("        .handle_typed_event(signature, &body, event_handler)");
            println!("        .map_err(|_| StatusCode::BAD_REQUEST)?;");
            println!();
            println!("    Ok(StatusCode::OK)");
            println!("}}");
            println!("```");

            println!("\n✨ Webhook integration complete!");
            println!("Check the webhook models and utilities for full functionality.");
        }
        Err(e) => {
            eprintln!("Failed to create client: {:?}", e);
        }
    }
}
