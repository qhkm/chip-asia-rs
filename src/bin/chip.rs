use clap::Parser;

#[derive(Parser)]
#[command(name = "chip", about = "CHIP payment gateway CLI")]
struct Cli {
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

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
    println!("TODO");
}
