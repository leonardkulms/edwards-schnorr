use hex;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::{io::Write, str::FromStr};

fn verify_signature(message: &[u8], signature: &Signature, pubkey: &Pubkey) -> bool {
    signature.verify(pubkey.as_ref(), message)
}

fn main() {
    // Get the public key
    println!("Enter Solana public key (base58): ");
    let mut pubkey_str = String::new();
    std::io::stdin().read_line(&mut pubkey_str).unwrap();
    let pubkey = Pubkey::from_str(pubkey_str.trim()).expect("Invalid Solana public key");

    // Get the message
    println!("Enter message (plain text): ");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();
    let message = message.trim().as_bytes();

    // Get the signature
    println!("Enter signature (hex): ");
    let mut signature_hex = String::new();
    std::io::stdin().read_line(&mut signature_hex).unwrap();
    let signature_bytes = hex::decode(signature_hex.trim()).expect("Invalid hex signature");
    let signature =
        Signature::try_from(signature_bytes.as_slice()).expect("Invalid signature format");

    // Verify
    match verify_signature(message, &signature, &pubkey) {
        true => println!("✅ Valid signature! Message was signed by this public key"),
        false => println!("❌ Invalid signature! Message was NOT signed by this public key"),
    }
}
