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
    pub orders: Vec<OrderView>,
}