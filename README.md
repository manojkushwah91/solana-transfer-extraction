# Solana Transfer Extraction Substreams

[![Substreams](https://img.shields.io/badge/Powered%20by-Substreams-blueviolet)](https://substreams.dev)
[![Solana](https://img.shields.io/badge/Solana-00d1b2?logo=solana&logoColor=white)](https://solana.com)

Extract clean **token transfers** (SPL + Token-2022) and **native SOL transfers** from Solana blockchain in real-time.

This project is a starter built from the `sol-hello-world` template via `substreams init`.  
It filters transactions by Program ID → parses instructions → outputs structured transfer events (Protobuf).

Perfect for building wallets, analytics dashboards, DeFi monitors, or on-chain indexing pipelines.

## ✨ Features

- Filter by Program IDs (SPL Token, Token-2022, System Program for native SOL)
- Exclude noisy vote transactions automatically
- Custom parsing logic for transfers (amount, from/to, mint, etc.)
- Ready to import foundational modules (e.g., `solana-spl-token`) for enriched data
- GUI testing, CLI runs, sinks (SQL, Kafka, Parquet), and registry publishing

## 🚀 Quick Start

```bash
# 1. Build the WASM binary
substreams build

# 2. Authenticate (needed for mainnet streaming)
substreams auth

# 3. Launch interactive GUI (best way to experiment!)
substreams gui
Publish to the Substreams Registry:
Bashsubstreams registry login
substreams registry publish
📦 Modules
map_filtered_transactions
Kind: map
Input: sf.solana.type.v1.Block
Output: Filtered sf.solana.type.v1.Transactions (or similar)
Purpose: Retains only transactions interacting with specified Program IDs. Drops votes (~80% of traffic).
Params examples (pass via -p or manifest):

Use CaseParam CommandSPL Token (classic)program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DAToken-2022 (extensions)program:TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEbNative SOL transfersprogram:11111111111111111111111111111111SPL + Token-2022`program:Tokenkeg...Specific token/DEXAdd custom logic in map_my_data (e.g., mint filter)
map_my_data
Kind: map
Input: Output from map_filtered_transactions
Output: Your custom Protobuf (e.g., TokenTransfer messages)
Purpose: Parse instructions → extract clean transfers.
Key parsing logic (in src/lib.rs):

SPL Transfer: Instruction data starts with 0x03
System Transfer (SOL): 0x02 or balance delta tracking
Decode: from/to (base58), amount (u64), mint, authority

Example Protobuf output (proto/transfers.proto):
protomessage TokenTransfer {
  string tx_signature  = 1;
  uint64 slot          = 2;
  uint64 block_time    = 3;
  string from          = 4;  // base58 address
  string to            = 5;  // base58 address
  uint64 amount        = 6;
  string mint          = 7;  // base58 or "native"
  string token_program = 8;
  bool   is_native_sol = 9;
}
Pro Tip: Enhance with foundational imports in substreams.yaml:
YAMLimports:
  solana_common: streamingfast/solana-common@v0.3.0
  spl_token:     streamingfast/solana-spl-token@v0.1.4  # resolves owners, ATAs, etc.
⚡ Example Commands
SPL Token filtering + JSON output:
Bashsubstreams run -e mainnet.sol.streamingfast.io:443 \
  substreams.yaml map_my_data \
  -s -1000 \
  -p map_filtered_transactions="program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" \
  --output json
🛠 Recommended Next Steps

Define your Protobuf messages in proto/
Update substreams.yaml → set output_type for map_my_data
Implement full decoding (use solana-program, borsh, or imported modules)
Test with high-volume tokens (USDC, SOL transfers)
Sink data: substreams-sink-sql, Kafka, Parquet, etc.
Check official refs:
Solana Token Tracker
SPL Token Module
Substreams Docs


Happy building on Solana! 🚀
