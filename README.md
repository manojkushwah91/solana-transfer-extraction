# solana_transfer_extraction Substreams modules

This package was initialized via `substreams init` using the `sol-hello-world` template.

It serves as a starter for extracting **token transfers** (SPL Token Program + Token-2022) and **native SOL transfers** (System Program) from Solana.  
It filters relevant transactions first, then applies custom parsing logic to output clean transfer events.

## Quick Start

```bash
# Build the WASM binary
substreams build

# Authenticate for mainnet (required for streaming)
substreams auth

# Launch GUI to test interactively (recommended!)
substreams gui
To publish to the Substreams Registry:
Bashsubstreams registry login
substreams registry publish
Modules
map_filtered_transactions
Kind: map
Input: Solana blocks (sf.solana.type.v1.Block)
Output: Filtered transactions (typically sf.solana.type.v1.Transactions or similar)
Purpose: Keeps only transactions that call one or more specified Program IDs.
Automatically excludes vote transactions (~80% of Solana traffic).
Params (configure via -p or in manifest):
Use program: prefix with || for OR logic.
Examples:

SPL Token (classic tokens):Bash-p map_filtered_transactions="program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
Token-2022 (extensions like transfer fees):Bash-p map_filtered_transactions="program:TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
Native SOL transfers (System Program):Bash-p map_filtered_transactions="program:11111111111111111111111111111111"
SPL + Token-2022 combined:Bash-p map_filtered_transactions="program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA || program:TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"

map_my_data
Kind: map
Input: Output of map_filtered_transactions (or raw blocks)
Output: Custom Protobuf (define your own, e.g. TokenTransfer messages)
Purpose: Your main logic layer — parse instructions, decode data, extract transfers, enrich with metadata.
Typical implementation for transfers (in src/lib.rs):

Loop over instructions in filtered transactions.
Check program ID and instruction discriminator:
SPL Transfer: data[0] == 3 (0x03)
Token-2022 Transfer: similar, but check extensions
System Transfer: data[0] == 2 (0x02) or track balance deltas

Decode accounts: source, destination, mint, authority, amount (u64).
Output clean messages like:

proto// Example in proto/transfers.proto
message TokenTransfer {
  string tx_signature     = 1;
  uint64 slot             = 2;
  uint64 block_time       = 3;
  string from             = 4;      // base58
  string to               = 5;      // base58
  uint64 amount           = 6;
  string mint             = 7;      // base58, or "So11111111111111111111111111111111111111112" for wrapped SOL
  string token_program    = 8;
  bool   is_native_sol    = 9;
}
Pro tip: Import foundational modules from substreams.dev for easier parsing:
YAML# In substreams.yaml → imports section
imports:
  solana_common: streamingfast/solana-common@v0.3.0  # or latest
  spl_token:     streamingfast/solana-spl-token@v0.1.4
Then consume their outputs/stores in your map.
Example Run Commands
Basic test with SPL Token filtering:
Bashsubstreams run -e mainnet.sol.streamingfast.io:443 \
  substreams.yaml map_my_data \
  -s -1000 \
  -p map_filtered_transactions="program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" \
  --output json
Recommended Next Steps

Define output Protobuf in proto/ folder (e.g. transfers.proto).
Update substreams.yaml:
Set map_my_data output_type to your message
Document params

Implement decoding in Rust (use solana-program, borsh, or foundational modules).
Test on transfer-heavy slots (e.g. around big USDC/USDT moves).
Sink options: Use substreams-sink-sql, substreams-sink-kafka, Parquet, etc.
Explore official examples:
https://github.com/streamingfast/solana-token-tracker
https://substreams.dev/packages (search for spl-token or transfers)


Happy streaming! 🚀
