# CHIP SDK CLI Design

**Goal:** Add a CLI binary (`chip`) to the existing `chip-sdk` crate so AI agents and developers can call CHIP payment API methods from the terminal.

## Architecture

Single crate with both `lib.rs` and `src/bin/chip.rs`. The CLI wraps the existing async SDK methods using clap for argument parsing. Library users are unaffected - Cargo only compiles the binary when explicitly building it.

## Authentication

- `CHIP_API_TOKEN` env var (or `--token` flag override)
- `CHIP_BASE_URL` env var, defaults to `https://gate.chip-in.asia/api/v1` (or `--base-url` flag override)
- Flags take precedence over env vars

## Commands

```
chip [--token TOKEN] [--base-url URL] [--json] <COMMAND>

Commands:
  payment-methods    List available payment methods
  create-purchase    Create a new purchase
  get-purchase       Get purchase by ID
  cancel-purchase    Cancel a purchase
  capture-purchase   Capture a pre-authorized purchase
  charge-purchase    Charge with recurring token
  refund-purchase    Refund a purchase
  release-purchase   Release a pre-authorized purchase
  delete-token       Delete a recurring token
  verify-signature   Verify a webhook signature
```

## Output Format

- Default: pretty-printed JSON (human-readable)
- `--json` flag: compact single-line JSON (machine-parseable for AI agents)
- Errors to stderr, non-zero exit code on failure
- Error JSON on stderr when `--json` active: `{"error": "message", "status": 404}`

## Command Arguments

### payment-methods
- `--brand-id` (required)
- `--currency` (required)
- `--country` (optional)
- `--recurring` (optional flag)
- `--skip-capture` (optional flag)
- `--preauthorization` (optional flag)

### create-purchase
- `--brand-id` (required)
- `--email` (required)
- `--product-name` (required)
- `--product-price` (required)
- `--quantity` (optional, default 1)
- `--success-url` (optional)
- `--failure-url` (optional)
- `--stdin` (optional: read full Purchase JSON from stdin for advanced use)

### get-purchase, cancel-purchase, release-purchase, delete-token
- `<ID>` positional argument

### capture-purchase
- `<ID>` positional argument
- `--amount` (optional, for partial capture)

### charge-purchase
- `<ID>` positional argument
- `--recurring-token` (required)

### refund-purchase
- `<ID>` positional argument
- `--amount` (optional, for partial refund)

### verify-signature
- `--content` (required, or `--content-file`)
- `--signature` (required, base64)
- `--public-key-file` (required, path to PEM file)

## Dependencies

- `clap = { version = "4", features = ["derive", "env"] }` - argument parsing with env var support
- Existing: `tokio`, `serde_json`, `reqwest`

## Example Usage

```bash
# AI agent flow
export CHIP_API_TOKEN=sk_live_xxx
chip create-purchase --brand-id b-123 --email buyer@test.com --product-name "Widget" --product-price 100 --json
chip get-purchase abc-123 --json
chip payment-methods --brand-id b-123 --currency MYR --json

# Webhook verification
chip verify-signature --content '{"id":"..."}' --signature "base64sig" --public-key-file key.pem
```
