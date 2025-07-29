# uprusty

**uprusty** is an unofficial Rust client library for the [Up Banking API](https://developer.up.com.au/).

This library provides complete coverage of the Up API (beta release) for all available endpoints.

## ⚠️ Important Disclaimer

This is an **unofficial** library. When authenticated with your API token, this library can perform live transactions on your real bank account.

**The authors accept no responsibility or liability for any issues arising from the use of this library.** Please review the code thoroughly before use and test carefully in a safe environment.

## 🚀 Quick Start

First, obtain your API token by following the [Up Banking API authentication guide](https://developer.up.com.au/#welcome). Remember to revoke the token when finished.

Add to your `Cargo.toml`:
```toml
[dependencies]
uprusty = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

Basic usage:
```rust
use uprusty::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("up:yeah:your_token_here")?;
    
    // Test connection
    let ping = client.ping().await?;
    println!("Connected: {:?}", ping);
    
    // List accounts
    let accounts = client.list_accounts(None, None, None).await?;
    for account in accounts.data {
        println!("{}: ${}", 
            account.attributes.display_name, 
            account.attributes.balance.value
        );
    }
    
    // Get recent transactions
    let transactions = client.list_transactions(Some(10), None).await?;
    for tx in transactions.data {
        println!("{}: ${}", 
            tx.attributes.description, 
            tx.attributes.amount.value
        );
    }
    
    Ok(())
}
```

## 📖 Examples

The library includes examples demonstrating all features:

```bash
# Set your token (create a .env file)
echo "UP_API_TOKEN=up:yeah:your_token_here" > .env

# Run examples
cargo run --example accounts_usage
cargo run --example transactions_usage  
cargo run --example categories_usage
cargo run --example tags_usage
cargo run --example attachments_usage
cargo run --example webhooks_usage
```

## 🎯 API Coverage

| Feature | Status | Description |
|---------|--------|-------------|
| ✅ Accounts | Complete | List accounts, get account details |
| ✅ Transactions | Complete | List transactions, get transaction details, filter by account |
| ✅ Categories | Complete | List categories, get category details, categorize transactions |
| ✅ Tags | Complete | List tags, add/remove tags from transactions |
| ✅ Attachments | Complete | List attachments, get attachment details |
| ✅ Webhooks | Complete | Full CRUD operations, ping, logs, signature verification |
| ✅ Utils | Partial | Ping endpoint implemented |


## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🔗 Related

- [Up Banking API Documentation](https://developer.up.com.au/)
- [Up Banking](https://up.com.au/) - The bank this API is for
