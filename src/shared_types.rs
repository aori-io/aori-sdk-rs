// here goes aori order and others
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriOrder {
    pub offerer: String,
    pub input_token: String,
    pub input_amount: String,
    pub input_chain_id: i64,
    pub input_zone: String,
    pub output_token: String,
    pub output_amount: String,
    pub output_chain_id: i64,
    pub output_zone: String,
    pub start_time: String,
    pub end_time: String,
    pub salt: String,
    pub counter: i64,
    pub to_withdraw: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriMatchingDetails {
    pub maker_order: AoriOrder,
    pub taker_order: AoriOrder,
    pub maker_signature: String,
    pub taker_signature: String,
    pub block_deadline: i64,
    pub seat_number: i64,
    pub seat_holder: String,
    pub seat_percent_of_fees: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteRequested {
    pub input_token: String,
    pub output_token: String,
    pub input_amount: Option<String>,
    pub output_amount: Option<String>,
    pub chain_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderView {
    pub order_hash: String,
    pub order: AoriOrder,
    pub signature: Option<String>,
    pub input_token: String,
    pub input_amount: String,
    pub input_chain_id: i64,
    pub input_zone: String,
    pub output_token: String,
    pub output_amount: String,
    pub output_chain_id: i64,
    pub output_zone: String,
    pub rate: f64,
    pub created_at: i64,
    pub last_updated_at: i64,
    pub taken_at: Option<i64>,
    pub cancelled_at: Option<i64>,
    pub fulfilled_at: Option<i64>,
    pub system_cancelled: Option<bool>,
    pub is_active: bool,
    pub is_public: bool,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AoriViewOrderbookParams {
    pub signature: Option<String>,
    pub offerer: Option<String>,
    pub order_hash: Option<String>,
    pub query: Option<BaseQuoteQuery>,
    pub chain_id: Option<i64>,
    pub sort_by: Option<String>,
    pub input_amount: Option<String>,
    pub output_amount: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseQuoteQuery {
    pub base: String,
    pub quote: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailsToExecute {
    pub matching_hash: String,
    pub maker_order: AoriOrder,
    pub maker_order_hash: String,
    pub maker_chain_id: i64,
    pub maker_zone: String,
    pub taker_order: AoriOrder,
    pub taker_order_hash: String,
    pub taker_chain_id: i64,
    pub taker_zone: String,
    pub chain_id: i64, // this is generally just takerChainId
    pub to: String,
    pub value: f64,
    pub data: String,
    pub block_deadline: i64,
    pub maker: String,
    pub taker: String,
    pub input_token: String,
    pub input_amount: String,
    pub output_token: String,
    pub output_amount: String,
}
