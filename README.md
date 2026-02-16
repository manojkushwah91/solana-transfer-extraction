# Solana Transfer Extraction Substreams

[![Substreams](https://img.shields.io/badge/Powered%20by-Substreams-blueviolet)](https://substreams.dev)
[![Solana](https://img.shields.io/badge/Solana-00d1b2?logo=solana&logoColor=white)](https://solana.com)

Extract clean **token transfers** (SPL + Token-2022) and **native SOL transfers** from Solana blockchain in real-time.

Built from `sol-hello-world` template. Filters transactions by Program ID → parses instructions → outputs structured transfer events (Protobuf).

Perfect for wallets, analytics dashboards, DeFi monitors, and on-chain indexing.

## ✨ Features

- Filters by Program IDs (SPL Token, Token-2022, System Program)
- Automatically excludes noisy vote transactions (~80% of traffic)
- Custom parsing for transfers (amount, from/to, mint, authority)
- Ready for foundational modules (`solana-spl-token`, etc.)
- GUI testing, CLI runs, SQL/Kafka/Parquet sinks
- Registry publishing ready

## 🚀 Quick Start

```bash
# 1. Build WASM binary
substreams build

# 2. Authenticate for mainnet
substreams auth

# 3. Launch GUI (recommended for testing)
substreams gui
📦 Modules
map_filtered_transactions
Kind: map
Input: sf.solana.type.v1.Block
Output: Filtered sf.solana.type.v1.Transactions
Purpose: Retains transactions for specified Program IDs, drops votes.

Params:| Use Case         | Param Command                                       |
| ---------------- | --------------------------------------------------- |
| SPL Token        | program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA |
| Token-2022       | program:TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb |
| Native SOL       | program:11111111111111111111111111111111            |
| SPL + Token-2022 | Multiple program:... params                         |

map_my_data
Kind: map
Input: map_filtered_transactions output
Output: Custom TokenTransfer Protobuf
Purpose: Parse instructions → extract clean transfers.

Key Parsing:

SPL Transfer: Instruction 0x03

System Transfer (SOL): 0x02 or balance deltas

Decodes: from/to (base58), amount (u64), mint, authority

Protobuf Output (proto/transfers.proto):message TokenTransfer {
  string tx_signature  = 1;
  uint64 slot          = 2;
  uint64 block_time    = 3;
  string from          = 4;  // base58
  string to            = 5;  // base58
  uint64 amount        = 6;
  string mint          = 7;  // base58 or "native"
  string token_program = 8;
  bool   is_native_sol = 9;
}

⚡ Example Commands
SPL Token filtering + JSON:substreams run -e mainnet.sol.streamingfast.io:443 \
  substreams.yaml map_my_data \
  -s -1000 \
  -p map_filtered_transactions="program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" \
  --output json

Publish to Registry:substreams registry login
substreams registry publish

🛠 Next Steps
Define Protobuf messages in proto/

Update substreams.yaml → set output_type

Implement decoding (solana-program, borsh, imported modules)

Test with high-volume tokens (USDC, SOL)

Add sinks: substreams-sink-sql, Kafka, Parquet

Enhance with imports (substreams.yaml):imports:
  solana_common: streamingfast/solana-common@v0.3.0
  spl_token: streamingfast/solana-spl-token@v0.1.4

📚 Resources
Solana Token Program

SPL Token Module

Substreams Docs

Happy building on Solana! 🚀
