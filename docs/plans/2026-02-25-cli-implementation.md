# CLI Feature Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a `chip` CLI binary to the existing chip-sdk crate so AI agents and developers can call CHIP payment API methods from the terminal.

**Architecture:** Single crate with `[[bin]]` target at `src/bin/chip.rs`. Uses clap derive for subcommand parsing. Auth via env vars (`CHIP_API_TOKEN`, `CHIP_BASE_URL`) with `--token`/`--base-url` flag overrides. Output defaults to pretty JSON, `--json` flag for compact single-line JSON. All commands delegate to existing `ChipClient` async methods.

**Tech Stack:** clap 4 (derive + env features), existing tokio/serde_json/reqwest, chip-sdk library

---

### Task 1: Add clap dependency and binary target

**Files:**
- Modify: `Cargo.toml:1-22`
- Create: `src/bin/chip.rs`

**Step 1: Add clap to Cargo.toml dependencies**

In `Cargo.toml`, add after the `base64` line (line 17):

```toml
clap = { version = "4", features = ["derive", "env"] }
```

**Step 2: Create minimal binary that compiles**

Create `src/bin/chip.rs`:

```rust
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
```

**Step 3: Verify it compiles**

Run: `cargo build --bin chip`
Expected: Compiles successfully

**Step 4: Verify help works**

Run: `cargo run --bin chip -- --help`
Expected: Shows help text with "CHIP payment gateway CLI" and `get-purchase` subcommand

**Step 5: Commit**

```bash
git add Cargo.toml src/bin/chip.rs
git commit -m "feat(cli): add clap dependency and minimal binary scaffold"
```

---

### Task 2: Implement global options and client builder helper

**Files:**
- Modify: `src/bin/chip.rs`

**Step 1: Add global options and client helper**

Replace `src/bin/chip.rs` with:

```rust
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
```

**Step 2: Verify it compiles**

Run: `cargo build --bin chip`
Expected: Compiles successfully

**Step 3: Verify help shows global options**

Run: `cargo run --bin chip -- --help`
Expected: Shows `--token`, `--base-url`, `--json` global options and `get-purchase` subcommand

**Step 4: Verify error without token**

Run: `cargo run --bin chip -- get-purchase abc-123 2>&1`
Expected: Error message about missing API token (unless CHIP_API_TOKEN is set)

**Step 5: Commit**

```bash
git add src/bin/chip.rs
git commit -m "feat(cli): add global options, client builder, JSON output helpers"
```

---

### Task 3: Add all purchase subcommands

**Files:**
- Modify: `src/bin/chip.rs`

**Step 1: Expand Commands enum with all purchase operations**

Replace the `Commands` enum and expand the `run` function match arms. The full `Commands` enum should be:

```rust
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
```

**Step 2: Implement all match arms in the `run` function**

```rust
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
```

**Step 3: Verify it compiles**

Run: `cargo build --bin chip`
Expected: Compiles successfully

**Step 4: Verify all subcommands appear in help**

Run: `cargo run --bin chip -- --help`
Expected: Lists all 10 subcommands: payment-methods, create-purchase, get-purchase, cancel-purchase, capture-purchase, charge-purchase, refund-purchase, release-purchase, delete-token, verify-signature

**Step 5: Verify subcommand help**

Run: `cargo run --bin chip -- create-purchase --help`
Expected: Shows all flags for create-purchase including --brand-id, --email, --product-name, --product-price, --stdin

**Step 6: Commit**

```bash
git add src/bin/chip.rs
git commit -m "feat(cli): implement all 10 subcommands"
```

---

### Task 4: Add CLI integration tests

**Files:**
- Create: `tests/cli_tests.rs`

**Step 1: Write CLI integration tests using assert_cmd**

First, add `assert_cmd` to dev-dependencies in `Cargo.toml`:

```toml
assert_cmd = "2"
```

Create `tests/cli_tests.rs`:

```rust
use assert_cmd::Command;

#[test]
fn cli_shows_help() {
    Command::cargo_bin("chip")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("CHIP payment gateway CLI"));
}

#[test]
fn cli_shows_version() {
    Command::cargo_bin("chip")
        .unwrap()
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn cli_errors_without_token() {
    Command::cargo_bin("chip")
        .unwrap()
        .env_remove("CHIP_API_TOKEN")
        .args(["get-purchase", "abc-123"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("token"));
}

#[test]
fn cli_errors_without_subcommand() {
    Command::cargo_bin("chip")
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn cli_create_purchase_requires_flags() {
    Command::cargo_bin("chip")
        .unwrap()
        .env("CHIP_API_TOKEN", "test")
        .args(["create-purchase"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("--brand-id"));
}

#[test]
fn cli_verify_signature_requires_content_source() {
    Command::cargo_bin("chip")
        .unwrap()
        .args([
            "verify-signature",
            "--signature", "dGVzdA==",
            "--public-key-file", "nonexistent.pem",
        ])
        .assert()
        .failure();
}
```

Also add `predicates` to dev-dependencies:

```toml
predicates = "3"
```

**Step 2: Run tests to verify they pass**

Run: `cargo test --test cli_tests`
Expected: All 6 tests pass

**Step 3: Commit**

```bash
git add Cargo.toml tests/cli_tests.rs
git commit -m "test(cli): add CLI integration tests"
```

---

### Task 5: Update README and run final checks

**Files:**
- Modify: `README.md`

**Step 1: Add CLI section to README.md**

Add a "## CLI Usage" section after the "## Quick Start" section in `README.md`, documenting:
- Installation via `cargo install chip-sdk`
- Authentication (env vars + flags)
- All subcommands with examples
- `--json` flag for AI agents
- Verify-signature usage

**Step 2: Run clippy**

Run: `cargo clippy --all-targets -- -D warnings`
Expected: No warnings

**Step 3: Run fmt**

Run: `cargo fmt -- --check`
Expected: No formatting issues

**Step 4: Run all tests**

Run: `cargo test`
Expected: All tests pass (existing 46 + new CLI tests)

**Step 5: Commit**

```bash
git add README.md
git commit -m "docs: add CLI usage section to README"
```

---

### Task 6: Publish v0.2.0 to crates.io

**Files:**
- Modify: `Cargo.toml:3` (version bump)

**Step 1: Bump version to 0.2.0**

In `Cargo.toml`, change `version = "0.1.0"` to `version = "0.2.0"`.

**Step 2: Dry run publish**

Run: `cargo publish --dry-run`
Expected: Packages successfully, no errors

**Step 3: Commit and push**

```bash
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0 for CLI feature"
git push
```

**Step 4: Publish**

Run: `cargo publish`
Expected: Published chip-sdk v0.2.0 to crates.io
