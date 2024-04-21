use serde::{Deserialize, Serialize};

use super::{DetailsToExecute, OrderView};


#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriPingResponse(pub String);

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAuthResponse {
    pub auth: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriOrderbookData {
    pub orders: Vec<OrderView>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriViewOrderbookResponse {
    pub id: i64,
    pub result: AoriOrderbookData
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriCancelOrderResponse {
    pub id: i64,
    pub result: OrderView
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriCancelAllOrdersResponse {
    pub id: i64,
    pub result: Vec<OrderView>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAccountBalanceResponse {
    pub id: i64,
    pub result: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriAccountOrdersResponse {
    pub id: i64,
    pub result: Vec<OrderView>
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AoriRequestQuoteResponse {
    pub id: i64,
    pub result: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriMakeOrderResponse {
    pub id: i64,
    pub result: OrderView
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriTakeOrderResponse {
    pub id: i64,
    pub result: DetailsToExecute
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriErrorData {
    pub code: i64,
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AoriErrorResponse {
    pub id: i64,
    pub error: AoriErrorData,
}