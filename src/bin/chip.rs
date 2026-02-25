use clap::Parser;
use chip_sdk::{ChipClient, ChipError};

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
    /// Get purchase by ID
    GetPurchase {
        /// Purchase ID
        id: String,
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
        Commands::GetPurchase { id } => {
            let client = build_client(cli)?;
            let purchase = client.get_purchase(id).await?;
            print_json(&purchase, cli.json);
        }
    }
    Ok(())
}
