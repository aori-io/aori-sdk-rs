use alloy_primitives::{keccak256, FixedBytes};
use alloy_sol_types::{sol, SolValue};
use anyhow::Result;
use ethers::{
    types::{Address, RecoveryMessage, Sign, Signature, H256},
    utils::{hash_message, hex},
};
use k256::{
    ecdsa::{self, RecoveryId, Signature as K256Signature, VerifyingKey},
    elliptic_curve::{consts::U32, generic_array::GenericArray, sec1::ToEncodedPoint},
    PublicKey as K256PublicKey,
};
use std::str::FromStr;
use tracing::{debug, info};

use super::{get_order_hash, AoriOrder};

pub async fn validate_raw_signature(signature: &str, hash: &str, address: &str) -> Result<bool> {
    let signature = Signature::from_str(signature)?;
    println!("Signature: {:?}", signature);

    let hash_bytes = hex::decode(hash)?;
    let message = RecoveryMessage::Data(hash_bytes);

    println!("Message: {:?}", message);

    let signer_address = signature.recover(message.clone())?;
    let signer_address = format!("0x{}", hex::encode(signer_address.as_bytes()));

    if address.to_lowercase() == signer_address.to_lowercase() {
        debug!(
            "For hash {}, the signer provided is {}, but signer would be {}",
            hash, address, signer_address
        );
        Ok(true)
    } else {
        info!(
            "For hash {}, the signer provided is {}, but signer would be {}",
            hash, address, signer_address
        );
        Err(anyhow::anyhow!("Ownership validation failed, ensure the signature is signed by the offerer, or in the case of a vault that the manager is the offerer."))
    }
}

pub async fn validate_signature_ownership(signature: &str, address: &str) -> Result<bool> {
    // TODO: @dev: ERROR HANDLING!
    // validating the signature of "\x19Ethereum Signed Message:\n $WALLET_ADDRESS"
    let signature = Signature::from_str(signature)?;

    let message = ethers::utils::hash_message(address);

    let signer_address = signature.recover(message)?;

    if Address::from_str(address)? == signer_address {
        Ok(true)
    } else {
        Ok(false)
        // anyhow::bail!("Ownership validation failed, ensure the signature is signed by the
        // offerer, or in the case of a vault that the manager is the offerer.")
    }
}

pub async fn get_signer_address(payload_hash: &[u8], signature: &str) -> Result<Address> {
    let signature = Signature::from_str(signature)?;
    let message = RecoveryMessage::Hash(H256::from_slice(payload_hash));
    let signer_address = signature.recover(message)?;
    Ok(signer_address)
}

pub async fn get_order_signer(order: AoriOrder, signature: &str) -> Result<Address> {
    let signature = Signature::from_str(signature)?;
    let message = RecoveryMessage::Hash(H256::from(get_order_hash(order).0));
    let signer_address = signature.recover(message)?;
    Ok(signer_address)
}

fn u256_to_uint(u: ethers::core::types::U256) -> alloy_primitives::Uint<256, 4> {
    let u_string = u.to_string();
    alloy_primitives::Uint::<256, 4>::from_str(&u_string).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::FixedBytes;
    use ethers::{
        prelude::rand,
        signers::{LocalWallet, Signer},
        types::H256,
    };

    #[tokio::test]
    async fn test_get_signer_address_with_hash() {
        // Generate a random wallet
        let wallet: LocalWallet = LocalWallet::new(&mut rand::thread_rng());

        // Create a payload and sign it
        let payload_bytes: [u8; 32] = rand::random();
        let payload: FixedBytes<32> = FixedBytes::from(payload_bytes);
        let payload_hash = H256::from_slice(payload.as_slice());
        let signature = wallet.sign_hash(payload_hash).expect("couldn't sign.");

        // Convert signature to string
        let signature_str = signature.to_string();

        // Test that the function correctly identifies the signer
        match get_signer_address(payload_hash.as_bytes(), &signature_str).await {
            Ok(result) => assert_eq!(result, wallet.address()),
            Err(e) => panic!("Error occurred: {}", e),
        };
    }
    #[tokio::test]
    async fn test_get_signer_address_with_wrong_signature() {
        // Generate a random wallet
        let wallet: LocalWallet = LocalWallet::new(&mut rand::thread_rng());

        // Create a payload and sign it
        let payload_bytes: [u8; 32] = rand::random();
        let payload: FixedBytes<32> = FixedBytes::from(payload_bytes);
        let payload_hash = H256::from_slice(payload.as_slice());
        let signature = wallet.sign_hash(payload_hash).expect("couldn't sign.");

        // Convert signature to string and modify it to be incorrect
        let mut signature_str = signature.to_string();
        signature_str.push('0');

        // Test that the function correctly identifies the error
        match get_signer_address(payload_hash.as_bytes(), &signature_str).await {
            Ok(_) => panic!("Expected an error, but didn't get one."),
            Err(_) => (), // Expected an error, so do nothing
        };
    }

    #[tokio::test]
    async fn test_get_signer_address_with_wrong_address() {
        // Generate a random wallet
        let wallet: LocalWallet = LocalWallet::new(&mut rand::thread_rng());

        // Create a payload and sign it
        let payload_bytes: [u8; 32] = rand::random();
        let payload: FixedBytes<32> = FixedBytes::from(payload_bytes);
        let payload_hash = H256::from_slice(payload.as_slice());
        let signature = wallet.sign_hash(payload_hash).expect("couldn't sign.");

        // Convert signature to string
        let signature_str = signature.to_string();

        // Generate a second random wallet and use its address
        let wrong_wallet: LocalWallet = LocalWallet::new(&mut rand::thread_rng());

        // Test that the function correctly identifies the error
        match get_signer_address(payload_hash.as_bytes(), &signature_str).await {
            Ok(result) => assert_ne!(result, wrong_wallet.address()),
            Err(e) => panic!("Error occurred: {}", e),
        };
    }
}
