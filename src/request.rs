use crate::shared_types::AoriOrder;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriPingParams(String);

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriMakeOrderParams {
    pub order: AoriOrder,
    pub signature: String,
    pub is_public: Option<bool>,
    pub seat_id: Option<i64>,
    pub tag: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAuthParams {
    pub address: String,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
}

// #[derive(Default, Serialize, Deserialize, Debug)]
// pub struct AoriViewOrderbookParams {
//     pub signature: Option<String>,
//     pub chain_id: Option<i64>,
//     pub query: Option<HashMap<String, String>>,
//     pub side: Option<String>,
//     pub limit: Option<i64>,
//     pub previous_order_hash: Option<String>,
//     pub order_hash: Option<String>,
//     pub offerer: Option<String>,
//     pub sort_by: Option<String>,
//     pub input_amount: Option<String>,
//     pub output_amount: Option<String>,
// }

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCheckAuthParams {
    pub auth: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriTakeOrderParams {
    pub order: AoriOrder,
    pub signature: String,
    pub order_hash: String,
    pub seat_id: Option<i64>,
    pub signed_approval_tx: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCancelOrderParams {
    pub order_hash: String,
    pub signature: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriCancelAllOrdersParams {
    pub offerer: String,
    pub signature: Option<String>,
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
    pub input_token: String,
    pub output_token: String,
    pub input_amount: Option<String>,
    pub output_amount: Option<String>,
    pub chain_id: i64,
    pub api_key: String,
}
