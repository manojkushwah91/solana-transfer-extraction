#[allow(unused)]
mod pb;

use pb::{mydata, sf::substreams::solana::v1 as solana};
use bs58;
use substreams::errors::Error;

const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

#[substreams::handlers::map]
fn map_extract_transfers(transactions: solana::Transactions) -> Result<mydata::Transfers, Error> {
    let mut transfers = Vec::new();

    for txn in transactions.transactions.iter() {
        // Get the inner transaction
        let inner_tx = txn.transaction.as_ref().ok_or_else(|| Error::msg("missing transaction"))?;
        let message = inner_tx.message.as_ref().ok_or_else(|| Error::msg("missing message"))?;
        let account_keys = &message.account_keys;

        for instr in message.instructions.iter() {
            // Get program ID
            let program_id_index = instr.program_id_index as usize;
            if program_id_index >= account_keys.len() {
                continue;
            }
            let prog_id = &account_keys[program_id_index];
            let prog_id_bs58 = bs58::encode(prog_id).into_string();

            if prog_id_bs58 != TOKEN_PROGRAM_ID {
                continue;
            }

            let data = &instr.data;

            // Need at least 1 (type) + 8 (amount) = 9 bytes
            if data.is_empty() || data.len() < 9 {
                continue;
            }

            let instr_type = data[0];

            // Only Transfer (3) and TransferChecked (12)
            if instr_type != 3 && instr_type != 12 {
                continue;
            }

            // Amount from bytes 1–8 (little-endian u64)
            let amount_bytes: [u8; 8] = match data[1..9].try_into() {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let amount = u64::from_le_bytes(amount_bytes);

            // Need at least source (0) and destination (1)
            if instr.accounts.len() < 2 {
                continue;
            }

            let from_idx = instr.accounts[0] as usize;
            let to_idx = instr.accounts[1] as usize;

            if from_idx >= account_keys.len() || to_idx >= account_keys.len() {
                continue;
            }

            let from = bs58::encode(&account_keys[from_idx]).into_string();
            let to = bs58::encode(&account_keys[to_idx]).into_string();

            transfers.push(mydata::Transfer {
                from,
                to,
                amount,
            });
        }
    }

    Ok(mydata::Transfers { transfers })
}