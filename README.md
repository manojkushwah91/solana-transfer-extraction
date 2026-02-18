# Solana Transfer Extraction — Substreams

[![Powered by Substreams](https://img.shields.io/badge/Powered%20by-Substreams-blueviolet)](https://substreams.dev)
[![Solana](https://img.shields.io/badge/Solana-00d1b2?logo=solana\&logoColor=white)](https://solana.com)

Extract **clean, structured transfer events** from the Solana blockchain using **Substreams**.

This Substreams package indexes:

* **SPL Token transfers**
* **Token-2022 transfers**
* **Native SOL transfers**

It filters transactions by **Program ID**, parses instructions deterministically, and emits **Protobuf-encoded transfer events** suitable for analytics, wallets, dashboards, and sinks (SQL, Kafka, Parquet, etc.).

Built from the `sol-hello-world` template and hardened for real-world Solana traffic.

---

## ✨ Features

* Program-ID–based transaction filtering
  (SPL Token, Token-2022, System Program)
* Automatic exclusion of vote transactions (~80% of chain traffic)
* Deterministic instruction parsing (no RPC calls)
* Unified output for SPL + Token-2022 + native SOL
* Protobuf-first design (JSON supported for testing)
* GUI-friendly for rapid iteration
* Registry-publishable Substreams package

---

## 🧱 Architecture Overview

```
Solana Block
   ↓
map_filtered_transactions
   ↓
map_my_data
   ↓
TokenTransfer (Protobuf)
```

---

## 📦 Modules

### `map_filtered_transactions`

**Kind:** `map`
**Input:** `sf.solana.type.v1.Block`
**Output:** `sf.solana.type.v1.Transactions`

**Purpose**

* Keeps only transactions involving selected Program IDs
* Drops vote transactions early to reduce noise and cost

**Parameters**

| Use Case          | Parameter Value                                       |
| ----------------- | ----------------------------------------------------- |
| SPL Token         | `program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` |
| Token-2022        | `program:TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` |
| Native SOL        | `program:11111111111111111111111111111111`            |
| Multiple Programs | Repeat `program:` parameter                           |

---

### `map_my_data`

**Kind:** `map`
**Input:** `map_filtered_transactions`
**Output:** `TokenTransfer` (custom Protobuf)

**Purpose**

* Parses instructions
* Extracts normalized transfer events

**Parsing Logic**

* **SPL / Token-2022**

  * Instruction: `Transfer` (`0x03`)
* **Native SOL**

  * System Program `Transfer` (`0x02`)
  * Balance deltas (fallback)

**Decoded Fields**

* Sender / recipient (base58)
* Amount (`u64`)
* Mint address or `"native"`
* Token program
* Slot, block time, transaction signature

---

## 📜 Protobuf Schema

`proto/transfers.proto`

```proto
message TokenTransfer {
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
```

---

## 🚀 Quick Start

### 1. Build the WASM

```bash
substreams build
```

### 2. Authenticate (mainnet)

```bash
substreams auth
```

### 3. Launch GUI (recommended)

```bash
substreams gui
```

---

## ⚡ Example CLI Run

Filter SPL Token transfers and output JSON:

```bash
substreams run -e mainnet.sol.streamingfast.io:443 \
  substreams.yaml map_my_data \
  -s -1000 \
  -p map_filtered_transactions="program:TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" \
  --output json
```

---

## 📤 Publishing to Registry

```bash
substreams registry login
substreams registry publish
```

---

## 🧩 Imports (Recommended)

```yaml
imports:
  solana_common: streamingfast/solana-common@v0.3.0
  spl_token: streamingfast/solana-spl-token@v0.1.4
```

---

## 🛠 Next Steps

* Expand instruction coverage (Approve, Burn, Mint)
* Add multi-sink examples:

  * `substreams-sink-sql`
  * Kafka
  * Parquet / Lakehouse
* Validate against high-volume tokens (USDC, USDT)
* Add replayable test vectors
* Optional: aggregate per-block or per-wallet stats

---

## 📚 Resources

* Solana Token Program Docs
* Substreams Documentation
* SPL Token Reference Modules

---

Happy building on Solana 🚀
This Substreams is designed to stay **fast, deterministic, and indexer-friendly**.
