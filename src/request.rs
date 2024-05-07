use crate::shared_types::AoriOrder;
use alloy_serde_macro::{bytes_as_string, bytes_from_string};
use serde::{Deserialize, Serialize};

use super::{OrderView, Query, ViewOrderbookQuery};

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct AoriPingParams(String);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriMakeOrderParams {
    pub order: AoriOrder,
    // #[serde(serialize_with = "bytes_as_string")]
    pub signature: String,
    pub is_public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "apiKey")]
    pub api_key: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAuthParams {
    pub address: String,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriCheckAuthParams {
    pub auth: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriTakeOrderParams {
    pub order: AoriOrder,
    pub signature: String,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seatId")]
    pub seat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "signedApprovalTx")]
    pub signed_approval_tx: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriCancelOrderParams {
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriCancelAllOrdersParams {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAccountBalanceParams {
    pub address: String,
    pub token: String,
    pub chain_id: i64,
    pub signature: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAccountCreditParams {
    pub address: String,
    pub signature: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAccountOrdersParams {
    pub signature: Option<String>,
    pub offerer: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriQuoteParams {
    pub input_token: String,
    pub output_token: String,
    pub input_amount: Option<String>,
    pub output_amount: Option<String>,
    pub chain_id: i64,
    pub api_key: String,
    pub delay: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRequestQuoteParams {
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub output_token: String,
    // #[serde(skip_serializing_if = "Option::is_none", rename = "inputAmount")]
    #[serde(rename = "inputAmount")]
    pub input_amount: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outputAmount")]
    pub output_amount: Option<String>,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriViewOrderbookQueryPair {
    pub base: String,
    pub quote: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriViewOrderbookParams {
    pub chain_id: Option<i64>,
    pub query: Option<AoriViewOrderbookQueryPair>,
    pub limit: Option<i64>,

    pub order_hash: Option<String>,
    pub offerer: Option<String>,

    pub sort_by: Option<String>,
    pub input_amount: Option<String>,
    pub output_amount: Option<String>,

    pub zone: Option<String>,
    pub allow_stale_quotes: Option<bool>,
}

///
///  Response Types
///

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriPingRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriViewOrderbookRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriViewOrderbookParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriCancelOrderRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriCancelOrderParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriCancelAllOrdersRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriCancelAllOrdersParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAccountBalanceRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriAccountBalanceParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAccountOrdersRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriAccountOrdersParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRequestQuoteRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriRequestQuoteParams>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriMakeOrderRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriMakeOrderParams>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriTakeOrderRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriTakeOrderParams>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriBroadcastParams {
    pub secret: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriBroadcastRequest {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<AoriBroadcastParams>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AoriRequest {
    Ping(AoriPingRequest),
    AccountBalance(AoriAccountBalanceRequest),
    AccountOrders(AoriAccountOrdersRequest),
    RequestQuote(AoriRequestQuoteRequest),
    MakeOrder(AoriMakeOrderRequest),
    TakeOrder(AoriTakeOrderRequest),
    CancelOrder(AoriCancelOrderRequest),
    CancelAllOrders(AoriCancelAllOrdersRequest),
    ViewOrderbook(AoriViewOrderbookRequest),
}

// TODO: rename from AoriRequestin to AoriGeneralRequest
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRequestin {
    pub id: i64,
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
}
