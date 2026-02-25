use chip_sdk::model::{ClientDetails, Product, Purchase, PurchaseDetails};
use chip_sdk::ChipClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ChipClient::builder()
        .base_url("https://gate.chip-in.asia/api/v1")
        .bearer_token("your-api-token-here")
        .build()?;

    // List available payment methods
    let methods = client.payment_methods("YOUR_BRAND_ID", "MYR", None).await?;
    println!(
        "Available payment methods: {:?}",
        methods.available_payment_methods
    );

    // Create a purchase
    let purchase = Purchase {
        brand_id: Some("YOUR_BRAND_ID".into()),
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
    println!("Purchase ID: {:?}", created.id);
    println!("Checkout URL: {:?}", created.checkout_url);

    if let Some(id) = &created.id {
        let fetched = client.get_purchase(id).await?;
        println!("Purchase status: {:?}", fetched.status);
    }

    Ok(())
}
