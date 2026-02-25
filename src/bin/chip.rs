use chip_sdk::{ChipClient, ChipError};
use clap::Parser;

#[derive(Parser)]
#[command(name = "chip", version, about = "CHIP payment gateway CLI")]
struct Cli {
    /// API bearer token (or set CHIP_API_TOKEN env var)
    #[arg(long, env = "CHIP_API_TOKEN", global = true, hide_env_values = true)]
    token: Option<String>,

    /// API base URL
    #[arg(
        long,
        env = "CHIP_BASE_URL",
        global = true,
        default_value = "https://gate.chip-in.asia/api/v1"
    )]
    base_url: String,

    /// Output compact JSON (single line, for machine parsing)
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// List available payment methods
    PaymentMethods {
        /// Brand ID
        #[arg(long)]
        brand_id: String,
        /// Currency code (e.g. MYR, USD)
        #[arg(long)]
        currency: String,
        /// Country code (e.g. MY)
        #[arg(long)]
        country: Option<String>,
        /// Filter for recurring-capable methods
        #[arg(long)]
        recurring: bool,
        /// Filter for skip-capture methods
        #[arg(long)]
        skip_capture: bool,
        /// Filter for preauthorization methods
        #[arg(long)]
        preauthorization: bool,
    },
    /// Create a new purchase
    CreatePurchase {
        /// Brand ID
        #[arg(long)]
        brand_id: String,
        /// Buyer email address
        #[arg(long)]
        email: String,
        /// Product name
        #[arg(long)]
        product_name: String,
        /// Product price (in minor units, e.g. 100 = 1.00)
        #[arg(long)]
        product_price: f64,
        /// Product quantity
        #[arg(long, default_value = "1")]
        quantity: f64,
        /// Success redirect URL
        #[arg(long)]
        success_url: Option<String>,
        /// Failure redirect URL
        #[arg(long)]
        failure_url: Option<String>,
        /// Read full Purchase JSON from stdin (ignores other purchase flags)
        #[arg(long)]
        stdin: bool,
    },
    /// Get purchase by ID
    GetPurchase {
        /// Purchase ID
        id: String,
    },
    /// Cancel a purchase
    CancelPurchase {
        /// Purchase ID
        id: String,
    },
    /// Capture a pre-authorized purchase
    CapturePurchase {
        /// Purchase ID
        id: String,
        /// Amount to capture (omit for full capture)
        #[arg(long)]
        amount: Option<f64>,
    },
    /// Charge using a recurring token
    ChargePurchase {
        /// Purchase ID
        id: String,
        /// Recurring token
        #[arg(long)]
        recurring_token: String,
    },
    /// Refund a purchase
    RefundPurchase {
        /// Purchase ID
        id: String,
        /// Amount to refund (omit for full refund)
        #[arg(long)]
        amount: Option<f64>,
    },
    /// Release a pre-authorized purchase
    ReleasePurchase {
        /// Purchase ID
        id: String,
    },
    /// Delete a recurring token
    DeleteToken {
        /// Purchase ID
        id: String,
    },
    /// Verify a webhook signature
    VerifySignature {
        /// Content to verify (raw string)
        #[arg(long, group = "content_source")]
        content: Option<String>,
        /// File containing content to verify
        #[arg(long, group = "content_source")]
        content_file: Option<String>,
        /// Base64-encoded signature
        #[arg(long)]
        signature: String,
        /// Path to PEM public key file
        #[arg(long)]
        public_key_file: String,
    },
}

fn build_client(cli: &Cli) -> Result<ChipClient, ChipError> {
    let token = cli.token.as_deref().unwrap_or_default();
    if token.is_empty() {
        return Err(ChipError::Config(
            "API token required: set CHIP_API_TOKEN or use --token".into(),
        ));
    }
    ChipClient::builder()
        .base_url(&cli.base_url)
        .bearer_token(token)
        .build()
}

fn print_json<T: serde::Serialize>(value: &T, compact: bool) {
    if compact {
        println!("{}", serde_json::to_string(value).unwrap());
    } else {
        println!("{}", serde_json::to_string_pretty(value).unwrap());
    }
}

fn print_error(err: &ChipError, compact: bool) {
    if compact {
        let msg = match err {
            ChipError::Api { status, message } => {
                serde_json::json!({"error": message, "status": status})
            }
            _ => serde_json::json!({"error": err.to_string()}),
        };
        eprintln!("{}", msg);
    } else {
        eprintln!("Error: {}", err);
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = run(&cli).await;
    if let Err(ref e) = result {
        print_error(e, cli.json);
        std::process::exit(1);
    }
}

async fn run(cli: &Cli) -> Result<(), ChipError> {
    match &cli.command {
        Commands::PaymentMethods {
            brand_id,
            currency,
            country,
            recurring,
            skip_capture,
            preauthorization,
        } => {
            let client = build_client(cli)?;
            let opts = if country.is_some() || *recurring || *skip_capture || *preauthorization {
                Some(chip_sdk::PaymentMethodsOptions {
                    country: country.clone(),
                    recurring: if *recurring { Some(true) } else { None },
                    skip_capture: if *skip_capture { Some(true) } else { None },
                    preauthorization: if *preauthorization { Some(true) } else { None },
                })
            } else {
                None
            };
            let methods = client.payment_methods(brand_id, currency, opts).await?;
            print_json(&methods, cli.json);
        }
        Commands::CreatePurchase {
            brand_id,
            email,
            product_name,
            product_price,
            quantity,
            success_url,
            failure_url,
            stdin,
        } => {
            let client = build_client(cli)?;
            let purchase = if *stdin {
                let mut input = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)
                    .map_err(|e| ChipError::Config(format!("Failed to read stdin: {}", e)))?;
                serde_json::from_str(&input)?
            } else {
                chip_sdk::model::Purchase {
                    brand_id: Some(brand_id.clone()),
                    client: Some(chip_sdk::model::ClientDetails {
                        email: email.clone(),
                        ..Default::default()
                    }),
                    purchase: Some(chip_sdk::model::PurchaseDetails {
                        products: vec![chip_sdk::model::Product {
                            name: product_name.clone(),
                            price: *product_price,
                            quantity: Some(*quantity),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    success_redirect: success_url.clone(),
                    failure_redirect: failure_url.clone(),
                    ..Default::default()
                }
            };
            let created = client.create_purchase(&purchase).await?;
            print_json(&created, cli.json);
        }
        Commands::GetPurchase { id } => {
            let client = build_client(cli)?;
            let purchase = client.get_purchase(id).await?;
            print_json(&purchase, cli.json);
        }
        Commands::CancelPurchase { id } => {
            let client = build_client(cli)?;
            let purchase = client.cancel_purchase(id).await?;
            print_json(&purchase, cli.json);
        }
        Commands::CapturePurchase { id, amount } => {
            let client = build_client(cli)?;
            let purchase = client.capture_purchase(id, *amount).await?;
            print_json(&purchase, cli.json);
        }
        Commands::ChargePurchase {
            id,
            recurring_token,
        } => {
            let client = build_client(cli)?;
            let purchase = client.charge_purchase(id, recurring_token).await?;
            print_json(&purchase, cli.json);
        }
        Commands::RefundPurchase { id, amount } => {
            let client = build_client(cli)?;
            let purchase = client.refund_purchase(id, *amount).await?;
            print_json(&purchase, cli.json);
        }
        Commands::ReleasePurchase { id } => {
            let client = build_client(cli)?;
            let purchase = client.release_purchase(id).await?;
            print_json(&purchase, cli.json);
        }
        Commands::DeleteToken { id } => {
            let client = build_client(cli)?;
            let purchase = client.delete_recurring_token(id).await?;
            print_json(&purchase, cli.json);
        }
        Commands::VerifySignature {
            content,
            content_file,
            signature,
            public_key_file,
        } => {
            let content_bytes = if let Some(file_path) = content_file {
                std::fs::read(file_path)
                    .map_err(|e| ChipError::Config(format!("Failed to read content file: {}", e)))?
            } else if let Some(raw) = content {
                raw.as_bytes().to_vec()
            } else {
                return Err(ChipError::Config(
                    "Either --content or --content-file is required".into(),
                ));
            };
            let pem = std::fs::read_to_string(public_key_file)
                .map_err(|e| ChipError::Config(format!("Failed to read public key file: {}", e)))?;
            let valid = chip_sdk::verify_signature(&content_bytes, signature, &pem)?;
            print_json(&serde_json::json!({"valid": valid}), cli.json);
        }
    }
    Ok(())
}
