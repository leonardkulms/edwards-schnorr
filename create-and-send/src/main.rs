use hex;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::Hash,
    message::Message,
    pubkey::Pubkey,
    signature::{Signature, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::{
    io::{self, Write},
    str::FromStr,
};

fn verify_pubkey_matches(hex_pubkey: &str, expected_solana_address: &str) -> bool {
    // Convert hex public key to bytes
    let pubkey_bytes = hex::decode(hex_pubkey).expect("Invalid hex public key");

    // Create Solana pubkey from bytes
    let pubkey = Pubkey::new(&pubkey_bytes);

    // Compare with expected address
    let expected = Pubkey::from_str(expected_solana_address).expect("Invalid Solana address");

    if pubkey == expected {
        println!("✅ Public key matches Solana address!");
        true
    } else {
        println!("❌ Public key does NOT match Solana address!");
        println!("Expected address: {}", expected_solana_address);
        println!("Got address: {}", pubkey);
        false
    }
}

fn create_and_submit_transaction(
    unsigned_tx: Transaction,
    signature_hex: &str,
) -> Result<Signature, Box<dyn std::error::Error>> {
    // Connect to Solana devnet
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Create transaction with signature
    let signature_bytes = hex::decode(signature_hex)?;
    println!("Debug: Signature length: {} bytes", signature_bytes.len());
    println!(
        "Debug: First few bytes of signature: {:?}",
        &signature_bytes[..8]
    );

    // Create the signature
    let signature = Signature::try_from(signature_bytes.as_slice())?;
    println!(
        "Debug: Account that should sign: {}",
        unsigned_tx.message.account_keys[0]
    );

    // Create the final transaction with the signature
    let mut signed_tx = unsigned_tx.clone();
    signed_tx.signatures = vec![signature];

    // Print the full transaction details
    println!("Debug: Transaction details:");
    println!("  From: {}", signed_tx.message.account_keys[0]);
    println!("  To: {}", signed_tx.message.account_keys[1]);
    println!("  Recent blockhash: {}", signed_tx.message.recent_blockhash);
    println!("  Raw signature bytes: {:?}", signature.as_ref());
    println!("  Signature (base58): {}", signature);

    // Send transaction
    let signature = rpc_client.send_transaction(&signed_tx)?;
    println!("Transaction submitted to devnet!");
    println!(
        "View transaction: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );

    Ok(signature)
}

fn main() {
    // Connect to Solana devnet
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Using test accounts
    let sender = "GiiYEuJBPXnnZYCganyUvygDLdtWBGnx3LVjS5ZBmds1";
    let receiver = "4CJM1T9TupZvXSKYHdh4NE61dXPp18bmkxB7gNFYDcK9";
    let lamports = 1_000_000; // 0.001 SOL in lamports

    println!("\nFirst, let's verify you have the correct key pair.");
    println!("Please run the signing script with any message and paste the public key (hex) here:");
    print!("Enter public key (hex): ");
    io::stdout().flush().unwrap();

    let mut pubkey_hex = String::new();
    io::stdin()
        .read_line(&mut pubkey_hex)
        .expect("Failed to read public key");
    let pubkey_hex = pubkey_hex.trim();

    if !verify_pubkey_matches(pubkey_hex, sender) {
        println!("Please make sure you're using the correct key pair!");
        return;
    }

    // Get recent blockhash first
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Create the sender and receiver pubkeys
    let sender_pubkey = Pubkey::from_str(sender).expect("Invalid sender pubkey");
    let receiver_pubkey = Pubkey::from_str(receiver).expect("Invalid receiver pubkey");

    // Create the transfer instruction
    let instruction = system_instruction::transfer(&sender_pubkey, &receiver_pubkey, lamports);

    // Create the message
    let message = Message::new(&[instruction], Some(&sender_pubkey));

    // Create the unsigned transaction
    let mut unsigned_tx = Transaction::new_unsigned(message);
    unsigned_tx.message.recent_blockhash = recent_blockhash;

    // Get the exact bytes that need to be signed
    let message_bytes = unsigned_tx.message_data();

    println!("\n=== Message to sign ===");
    println!("Message (hex): {}", hex::encode(&message_bytes));
    println!("Sender pubkey: {}", sender);
    println!("Recent blockhash: {}", recent_blockhash);
    println!("\nPlease sign this message using the signing script and paste the signature here.");
    println!(
        "Command to use: cargo run -- YOUR_SECRET_KEY_HEX {}\n",
        hex::encode(&message_bytes)
    );

    print!("Enter signature: ");
    io::stdout().flush().unwrap();

    let mut signature_hex = String::new();
    io::stdin()
        .read_line(&mut signature_hex)
        .expect("Failed to read signature");
    let signature_hex = signature_hex.trim();

    // Create and submit the transaction
    match create_and_submit_transaction(unsigned_tx, signature_hex) {
        Ok(sig) => println!("Transaction signature: {}", sig),
        Err(err) => eprintln!("Failed to submit transaction: {}", err),
    }
}
