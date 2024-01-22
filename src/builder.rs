// request builder
// used for signing orders and stuff

use crate::request::*;
use alloy_signer::{Signer, Wallet};
use k256::ecdsa::SigningKey;
use nom::AsBytes;

pub struct AoriRequestBuilder {
    signer: Wallet<SigningKey>,
}

impl AoriRequestBuilder {
    pub fn new(pkey_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let wallet: Wallet<SigningKey> = pkey_str.parse().unwrap();
        Ok(AoriRequestBuilder { signer: wallet })
    }
    pub async fn create_auth_params(
        &self,
        manager: Option<String>,
    ) -> Result<AoriAuthParams, Box<dyn std::error::Error>> {
        let address = self.signer.address();
        let address_bytes = address.as_bytes();

        let signature = self.signer.sign_message(address_bytes).await?;
        let sig = signature.as_bytes();
        let sig_hex = hex::encode(sig);

        Ok(AoriAuthParams {
            address: address.to_checksum(None), /* possibly change this later to incorporate
                                                 * chain ids */
            signature: format!("0x{}", sig_hex),
            manager,
        })
    }
}
