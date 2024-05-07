use std::env;

use reqwest::Client;
use serde_json::Value;

pub enum SubscriptionEvents {
    OrderCreated,
    OrderCancelled,
    OrderTaken,
    OrderFulfilled,
    OrderToExecute,
    QuoteRequested,
}

impl ToString for SubscriptionEvents {
    fn to_string(&self) -> String {
        match self {
            SubscriptionEvents::OrderCreated => "OrderCreated",
            SubscriptionEvents::OrderCancelled => "OrderCancelled",
            SubscriptionEvents::OrderTaken => "OrderTaken",
            SubscriptionEvents::OrderFulfilled => "OrderFulfilled",
            SubscriptionEvents::OrderToExecute => "OrderToExecute",
            SubscriptionEvents::QuoteRequested => "QuoteRequested",
        }
        .to_string()
    }
}

pub async fn broadcast_subscription_event(event: SubscriptionEvents, data: serde_json::Value) {
    let data = serde_json::json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "aori_broadcast",
        "params": vec![serde_json::json!({
            "secret": "",
            "data": serde_json::json!({
                "type": event.to_string(),
                "data": data
            })
        })]
    });

    let client = Client::new();
    let _ = client
        .post(env::var("AORI_BROADCAST_URL").expect("missing AORI_BROADCAST_URL"))
        .json::<Value>(&data)
        .send()
        .await;
}
