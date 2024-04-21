use crate::{request::*, AoriOrder};
use alloy_primitives::keccak256;
use alloy_signer::{Signer, Wallet};
use alloy_sol_types::SolValue;
use k256::ecdsa::SigningKey;

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
        let address_str = address.to_string();
        let address_bytes = address_str.as_bytes();

        let signature = self.signer.sign_message(address_bytes).await?;
        let sig = signature.as_bytes();
        let sig_hex = hex::encode(sig);

        Ok(AoriAuthParams {
            address: address.to_checksum(None),
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
        input_chain_id: i64,
        output_chain_id: i64,
        api_key: String,
    ) -> Result<AoriRequestQuoteParams, Box<dyn std::error::Error>> {
        Ok(AoriRequestQuoteParams {
            input_token,
            output_token,
            input_amount,
            output_amount,
            input_chain_id,
            output_chain_id,
            api_key,
        })
    }

    pub async fn make_order(
        &self,
        order: AoriOrder,
        is_public: bool,
        seat_id: Option<i64>,
        api_key: String,
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
            seat_id,
            api_key: Some(api_key),
        })
    }

    pub async fn take_order(
        &self,
        order_hash: String,
        order: AoriOrder,
        seat_id: Option<i64>,
        api_key: String,
    ) -> Result<AoriTakeOrderParams, Box<dyn std::error::Error>> {
        let packed = order.abi_encode();
        let hash = keccak256(packed);

        let signature = self.signer.sign_hash(hash).await?;
        let sig = signature.as_bytes();
        let sig_hex = hex::encode(sig);

        Ok(AoriTakeOrderParams {
            order_hash,
            order,
            signature: format!("0x{}", sig_hex).into(),
            seat_id,
            api_key: Some(api_key),
        })
    }
}