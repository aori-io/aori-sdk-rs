// request builder
// used for signing orders and stuff

use crate::{request::*, AoriOrder};
use alloy_primitives::{keccak256, B256};
use alloy_serde_macro::{bytes, bytes_from_string};
use alloy_sol_types::SolValue;
use ethers::signers::LocalWallet;
use ethers::{abi::parse_abi_str, utils::parse_bytes32_string};

use super::get_order_hash;

pub struct AoriRequestBuilder {
    signer: LocalWallet,
}

impl AoriRequestBuilder {
    /// Wraps around a Private Key / Wallet to sign off on trades
    pub fn new(pkey_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let wallet: LocalWallet = pkey_str.parse().unwrap();
        Ok(AoriRequestBuilder { signer: wallet })
    }

    /// Builds an RFQ request
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
        let packed = get_order_hash(order.clone());
        let hash = keccak256(packed);

        let signature = self.signer.sign_hash(hash.0.into())?;
        let sig_hex = hex::encode(signature.to_vec());

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
        order_hash: B256,
        seat_id: i64,
    ) -> Result<AoriTakeOrderParams, Box<dyn std::error::Error>> {
        let signature = self.signer.sign_hash(order_hash.0.into())?;
        let sig_hex = hex::encode(signature.to_vec());

        Ok(AoriTakeOrderParams {
            order,
            signature: format!("0x{}", sig_hex).into(),
            order_hash: order_hash.to_string(),
            seat_id: Some(seat_id),
            signed_approval_tx: None,
        })
    }
}
