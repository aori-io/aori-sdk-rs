use crate::shared_types::{AoriOrder, OrderView};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AoriFeedEventWrapper {
    pub id: Option<String>,
    pub result: AoriFeedEvents,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum AoriFeedEvents {
    OrderCreated(Box<OrderCreatedData>),
    OrderTaken(Box<OrderTakenData>),
    OrderCancelled(Box<OrderCancelledData>),
    QuoteRequested(Box<QuoteRequestedData>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreatedData {
    pub order: OrderView,
    pub order_hash: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderTakenData {
    pub order_hash: String,
    pub taker: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancelledData {
    pub order_hash: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequestedData {
    pub input_token: String,
    pub output_token: String,
    pub input_amount: String,
    pub output_amount: String,
    pub chain_id: u64,
}

pub fn deserialize_aori_feed_event(json_data: &str) -> Result<AoriFeedEvents, serde_json::Error> {
    let wrapper = serde_json::from_str::<AoriFeedEventWrapper>(json_data)?;
    Ok(wrapper.result)
}
