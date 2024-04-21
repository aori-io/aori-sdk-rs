// request builder
// used for signing orders and stuff

use crate::{request::*, AoriOrder};
use alloy_primitives::keccak256;
use alloy_signer::{Signer, Wallet};
use alloy_sol_types::SolValue;
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
    pub async fn build_rfq(
        &self,
        input_token: String,
        output_token: String,
        input_amount: Option<String>,
        output_amount: Option<String>,
        chain_id: i64,
        api_key: String,
    ) -> Result<AoriRequestQuoteParams, Box<dyn std::error::Error>> {
        Ok(AoriRequestQuoteParams {
            input_token,
            output_token,
            input_amount: input_amount.unwrap(),
            output_amount,
            chain_id,
            api_key,
        })
    }
    pub async fn make_order(
        &self,
        order: AoriOrder,
        is_public: bool,
        seat_id: i64,
        tag: String,
    ) -> Result<AoriMakeOrderParams, Box<dyn std::error::Error>> {
        let packed = order.abi_encode();
        let hash = keccak256(packed);

        let signature = self.signer.sign_hash(hash).await?;
        let sig = signature.as_bytes();
        let sig_hex = hex::encode(sig);

        Ok(AoriMakeOrderParams {
            order,
            signature: format!("0x{}", sig_hex).into(),
            is_public: Some(is_public),
            seat_id: Some(seat_id),
            tag: Some(tag),
            api_key: None,
        })
    }
    pub async fn take_order(
        &self,
        order: AoriOrder,
        order_hash: &str,
        seat_id: i64,
    ) -> Result<AoriTakeOrderParams, Box<dyn std::error::Error>> {
        let packed = order.abi_encode();
        let hash = keccak256(packed);

        let signature = self.signer.sign_hash(hash).await?;
        let sig = signature.as_bytes();
        let sig_hex = hex::encode(sig);

        Ok(AoriTakeOrderParams {
            order,
            signature: format!("0x{}", sig_hex).into(),
            order_hash: order_hash.to_string(),
            seat_id: Some(seat_id),
            signed_approval_tx: None,
        })
    }
}