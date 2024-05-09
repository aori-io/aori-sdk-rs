use anyhow::Result;
use ethers::{
    signers::{LocalWallet, Signer}, types::{Address, RecoveryMessage, Signature, H256}, utils::{hash_message, hex}
};
use std::str::FromStr;

use super::{get_order_hash, AoriOrder};

pub async fn sign_order(order: AoriOrder, key: &str) -> Result<String> {
    let wallet = LocalWallet::from_str(key).unwrap();
    let order_hash = get_order_hash(order.clone());
    let signature = wallet.sign_message(H256::from_slice(order_hash.as_slice())).await.unwrap();
    Ok(signature.to_string())
}

pub async fn get_order_signer(order: AoriOrder, signature: &str) -> Result<Address> {
    let order_hash = get_order_hash(order.clone());

    let signature = Signature::from_str(signature)?;
    println!("Signature: {:?}", signature);

    let hash_bytes = hex::decode(order_hash)?;
    let message = RecoveryMessage::Data(hash_bytes);

    println!("Message: {:?}", message);

    let signer_address = signature.recover(message.clone())?;
    let signer_address = format!("0x{}", hex::encode(signer_address.as_bytes()));

    Ok(Address::from_str(&signer_address)?)
}