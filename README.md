# chip-sdk

Rust SDK for the [CHIP](https://chip-in.asia) payment gateway API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
chip-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use chip_sdk::model::{ClientDetails, Product, Purchase, PurchaseDetails};
use chip_sdk::ChipClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ChipClient::builder()
        .base_url("https://gate.chip-in.asia/api/v1")
        .bearer_token("your-api-token")
        .build()?;

    // List payment methods
    let methods = client.payment_methods("BRAND_ID", "MYR", None).await?;
    println!("{:?}", methods.available_payment_methods);

    // Create a purchase
    let purchase = Purchase {
        brand_id: Some("BRAND_ID".into()),
        client: Some(ClientDetails {
            email: "buyer@example.com".into(),
            ..Default::default()
        }),
        purchase: Some(PurchaseDetails {
            products: vec![Product {
                name: "Test Product".into(),
                price: 100.0,
                quantity: Some(1.0),
                ..Default::default()
            }],
            ..Default::default()
        }),
        success_redirect: Some("https://yoursite.com/success".into()),
        failure_redirect: Some("https://yoursite.com/failure".into()),
        ..Default::default()
    };

    let created = client.create_purchase(&purchase).await?;
    println!("Checkout URL: {:?}", created.checkout_url);

    Ok(())
}
```

## API Methods

All methods are async and return `Result<T, ChipError>`.

| Method | Description |
|--------|-------------|
| `payment_methods(brand_id, currency, opts)` | List available payment methods |
| `create_purchase(purchase)` | Create a new purchase |
| `get_purchase(id)` | Get purchase details |
| `cancel_purchase(id)` | Cancel a purchase |
| `capture_purchase(id, amount)` | Capture a pre-authorized purchase |
| `charge_purchase(id, recurring_token)` | Charge using a recurring token |
| `refund_purchase(id, amount)` | Refund a purchase |
| `release_purchase(id)` | Release a pre-authorized purchase |
| `delete_recurring_token(id)` | Delete a recurring token |

## Webhook Signature Verification

Verify incoming webhook signatures using RSA PKCS1v15 SHA-256:

```rust
use chip_sdk::verify_signature;

let is_valid = verify_signature(
    request_body.as_bytes(),
    &signature_header,       // base64-encoded signature
    &public_key_pem,         // PEM-encoded public key from CHIP dashboard
)?;
```

## Client Configuration

```rust
use std::time::Duration;

let client = ChipClient::builder()
    .base_url("https://gate.chip-in.asia/api/v1")
    .bearer_token("your-token")
    .timeout(Duration::from_secs(30))  // default: 60s
    .build()?;
```

## Payment Method Options

```rust
use chip_sdk::PaymentMethodsOptions;

let opts = PaymentMethodsOptions {
    country: Some("MY".into()),
    recurring: Some(true),
    skip_capture: Some(true),
    preauthorization: Some(false),
};
let methods = client.payment_methods("BRAND_ID", "MYR", Some(opts)).await?;
```

## Error Handling

```rust
use chip_sdk::ChipError;

match client.get_purchase("invalid-id").await {
    Ok(purchase) => println!("Found: {:?}", purchase.id),
    Err(ChipError::Api { status, message }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## License

[Unlicense](LICENSE)
