use serde::{Deserialize, Serialize};
use crate::shared_types::{OrderView};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriPingResponse(pub String);

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAuthResponse {
    pub auth: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAccountOrdersResponse {
    pub orders: Vec<OrderView>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAccountBalanceResponse {
    pub balance: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriViewOrderbookResponse { 
    pub orders: Vec<OrderView>,
    pub count: i64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriMakeOrderResponse {
    pub result: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriTakeOrderResponse {
    pub result: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCancelOrderResponse {
    pub result: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriRequestQuoteResponse {
    pub result: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriQuoteResponse {
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub output_token: String,
    #[serde(rename = "inputAmount")]
    pub input_amount: String,
    #[serde(rename = "outputAmount")]
    pub output_amount: String,
    #[serde(rename = "gasCost")]
    pub gas_cost: String,
    pub price: String,
    #[serde(rename = "expirationTimestampInSeconds")]
    pub expiration_timestamp_in_seconds: String,
    pub orders: Vec<OrderView>,
}