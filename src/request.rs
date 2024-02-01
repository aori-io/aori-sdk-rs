use crate::shared_types::AoriOrder;
use serde::{Deserialize, Serialize};
use alloy_serde_macro::bytes_as_string;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriPingParams(String);

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriMakeOrderParams {
    pub order: AoriOrder,
    #[serde(serialize_with = "bytes_as_string")]
    pub signature: Vec<u8>,
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "apiKey")]
    pub api_key: Option<String>, 
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAuthParams {
    pub address: String,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCheckAuthParams {
    pub auth: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriTakeOrderParams {
    pub order: AoriOrder,
    #[serde(serialize_with = "bytes_as_string")]
    pub signature: Vec<u8>,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seatId")]
    pub seat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "signedApprovalTx")]
    pub signed_approval_tx: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCancelOrderParams {
    pub order_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCancelAllOrdersParams {
    pub offerer: String,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAccountBalanceParams {
    pub address: String,
    pub token: String,
    pub chain_id: i64,
    pub signature: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAccountCreditParams {
    pub address: String,
    pub signature: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAccountOrdersParams {
    pub signature: Option<String>,
    pub offerer: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriQuoteParams {
    pub input_token: String,
    pub output_token: String,
    pub input_amount: Option<String>,
    pub output_amount: Option<String>,
    pub chain_id: i64,
    pub api_key: String,
    pub delay: Option<i64>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriRequestQuoteParams {
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub output_token: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "inputAmount")]
    pub input_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outputAmount")]
    pub output_amount: Option<String>,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}
