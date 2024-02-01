use crate::*;
use jsonrpsee::proc_macros::rpc;

#[rpc(client)]
pub trait AoriBackendRpc {
    #[method(name = "aori_ping")]
    async fn ping(&self, parameters: AoriPingParams) -> RpcResult<AoriPingResponse>;
    #[method(name = "aori_requestQuote")]
    async fn request_quote(&self, parameters: AoriRequestQuoteParams) -> RpcResult<()>;
    #[method(name = "aori_cancelOrder")]
    async fn cancel_order(&self, parameters: AoriCancelOrderParams) -> RpcResult<String>;
    #[method(name = "aori_cancelAllOrders")]
    async fn cancel_all_orders(&self, parameters: AoriCancelAllOrdersParams) -> RpcResult<()>;
    #[method(name = "aori_checkAuth")]
    async fn check_auth(&self, parameters: AoriCheckAuthParams) -> RpcResult<bool>;
    #[method(name = "aori_authWallet")]
    async fn auth_wallet(&self, parameters: AoriAuthParams) -> RpcResult<AoriAuthResponse>;
    #[method(name = "aori_makeOrder")]
    async fn make_order(&self, parameters: AoriMakeOrderParams) -> RpcResult<OrderView>;
    #[method(name = "aori_takeOrder")]
    async fn take_order(&self, parameters: AoriTakeOrderParams) -> RpcResult<()>;
    // aori_quote
    // #[method(name = "aori_viewOrderbook")]
    // async fn view_orderbook(&self, parameters: ViewOrderbookQuery) -> RpcResult<Vec<OrderView>>;

    // feed later, preferably in a separate aori-stream-rs
    // #[subscription(name = "aori_subscribeOrderbook", item = AoriSubscriptionEvent)]
    // async fn subscribe(&self, parameters:) -> RpcResult<()>;
}

// #[rpc(client)]
// pub trait AoriMatchingEngine {
//     #[method(name = "aori_takeOrder")]
//     async fn take_order(&self, parameters: AoriTakeOrderParams) -> RpcResult<()>;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builder::AoriRequestBuilder, AoriBackendErrors, AoriBackendRpcClient, AoriPingParams,
    };
    use jsonrpsee::http_client::HttpClientBuilder;
    use tracing::{error, info, Level};

    #[tokio::test]
    async fn test_ping() {
        let _ = tracing_subscriber::fmt().with_max_level(Level::INFO).init();

        let url = "https://v2.api.aori.io";
        let client = HttpClientBuilder::default().build(url).unwrap();

        let request = AoriPingParams::default();
        // Send the ping request
        let response = client.ping(request).await;

        match response {
            Ok(AoriPingResponse(pong_message)) => {
                if pong_message == "aori_pong" {
                    info!("Received pong!");
                } else {
                    error!("Received an unexpected response message: {}", pong_message);
                }
            }
            Err(e) => {
                let custom_error: AoriBackendErrors = e.into();
                error!("Backend Server Response: {:?}", custom_error);
            }
        }
    }
    #[tokio::test]
    async fn test_auth() {
        let url = "https://api.aori.io";
        let client = HttpClientBuilder::default().build(url).unwrap();

        // engage request builder here
        let pkey = "0000000000000000000000000000000000000000000000000000000000000001";
        let builder = AoriRequestBuilder::new(pkey).unwrap();
        let request = builder.create_auth_params(None).await.unwrap();

        let serialized_request = serde_json::to_string(&request).unwrap();
        info!("Serialized request: {}", serialized_request);

        // Send the ping request
        let response = client.auth_wallet(request).await;
        info!("{:?}", response);
        assert!(response.is_ok(), "Expected Ok response, got {:?}", response);
    }
}
