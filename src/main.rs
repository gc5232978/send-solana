use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::signer::EncodableKey;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;

fn main() {
    let from = Keypair::read_from_file("from.json").expect("Failed to load keypair");
    let frompubkey = Signer::pubkey(&from);
    println!("FROM: {}", frompubkey);

    let topubkey = Pubkey::from_str("3S7aRRYec4QtbnGLdzanAHvKrZaWmQB4LR6N2MNpyK3Q")
        .expect("Failed to parse receivers pubkey");
    println!("TO: {}", topubkey);

    let lamports_to_send = 1_000_000;

    let rpc_url = String::from("https://api.devnet.solana.com");

    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let ix = system_instruction::transfer(&frompubkey, &topubkey, lamports_to_send);

    let recent_blockhash = connection
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash.");

    let txn =
        Transaction::new_signed_with_payer(&[ix], Some(&frompubkey), &[&from], recent_blockhash);

    match connection.send_and_confirm_transaction(&txn) {
        Ok(sig) => loop {
            if let Ok(confirmed) = connection.confirm_transaction(&sig) {
                if confirmed {
                    println!("TX: {}", sig);
                    println!("SUCCESS: {}", confirmed.to_string().to_uppercase());
                    break;
                }
            }
        },
        Err(e) => println!("ERROR: {}", e),
    }
}
