use crate::shared_types::{AoriOrder, Query};
use alloy_serde_macro::bytes_as_string;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriPingParams(String);

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriMakeOrderParams {
    pub order: AoriOrder,
    #[serde(serialize_with = "bytes_as_string")]
    pub signature: Vec<u8>,
    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seatId")]
    pub seat_id: Option<i64>,
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
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    pub order: AoriOrder,
    #[serde(serialize_with = "bytes_as_string")]
    pub signature: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seatId")]
    pub seat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "apiKey")]
    pub api_key: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCancelOrderParams {
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAccountOrdersParams {
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAccountBalanceParams {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub token: String,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriViewOrderbookParams {
    #[serde(skip_serializing_if = "Option::is_none", rename = "chainId")]
    pub chain_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Query>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "apiKey")]
    pub api_key: String,
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
    #[serde(rename = "inputChainId")]
    pub input_chain_id: i64,
    #[serde(rename = "outputChainId")]
    pub output_chain_id: i64,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}