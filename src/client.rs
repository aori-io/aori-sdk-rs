use crate::*;
use jsonrpsee::proc_macros::rpc;

#[rpc(client)]
pub trait AoriBackendRpc {
    #[method(name = "aori_ping")]
    async fn ping(&self, parameters: AoriPingParams) -> RpcResult<String>;
    #[method(name = "aori_requestQuote")]
    async fn request_quote(&self, parameters: AoriRequestQuoteParams) -> RpcResult<String>;
    #[method(name = "aori_cancelOrder")]
    async fn cancel_order(&self, parameters: AoriCancelOrderParams) -> RpcResult<String>;
    #[method(name = "aori_cancelAllOrders")]
    async fn cancel_all_orders(&self, parameters: AoriCancelAllOrdersParams) -> RpcResult<()>;
    #[method(name = "aori_makeOrder")]
    async fn make_order(&self, parameters: AoriMakeOrderParams) -> RpcResult<OrderView>;
    #[method(name = "aori_takeOrder")]
    async fn take_order(&self, parameters: AoriTakeOrderParams) -> RpcResult<String>;
    // aori_quote
    #[method(name = "aori_viewOrderbook")]
    async fn view_orderbook(&self, parameters: ViewOrderbookQuery) -> RpcResult<Vec<OrderView>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builder::AoriRequestBuilder, AoriBackendErrors, AoriBackendRpcClient, AoriPingParams,
    };
    use jsonrpsee::http_client::HttpClientBuilder;
    use tracing::{error, info, Level};

    ////////////////////////////////////////////////////////////////
    //                         AORI_PING
    ////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_ping() {
        let _ = tracing_subscriber::fmt().with_max_level(Level::INFO).init();

        let url = "https://v2.api.aori.io";
        let client = HttpClientBuilder::default().build(url).unwrap();

        let request = AoriPingParams::default();
        // Send the ping request
        let response = client.ping(request).await;

        match response {
            Ok(pong_message) => {
                if pong_message == "aori_pong" {
                    info!("Received pong!");
                } else {
                    error!("Received an unexpected response message: {:?}", pong_message);
                }
            }
            Err(e) => {
                let custom_error: AoriBackendErrors = e.into();
                error!("Backend Server Response: {:?}", custom_error);
            }
        }
    }

    ////////////////////////////////////////////////////////////////
    //                     AORI_REQUESTQUOTE
    ////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_rfq() {
        let url = "https://v2.api.aori.io";
        let client = HttpClientBuilder::default().build(url).unwrap();

        let pkey = "0000000000000000000000000000000000000000000000000000000000000001";
        let builder = AoriRequestBuilder::new(pkey).unwrap();

        let input_token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string();
        let output_token = "0xe3DBC4F88EAa632DDF9708732E2832EEaA6688AB".to_string();
        let input_amount = Some("1000000000000000000".to_string());
        let output_amount = None;
        let chain_id = 42161;
        let api_key = "test".to_string();

        let request = builder
            .build_rfq(input_token, output_token, input_amount, output_amount, chain_id, api_key)
            .await
            .unwrap();

        let response = client.request_quote(request).await;
        info!("RFQ RESPONSE: {:?}", response);
        assert!(response.is_ok(), "Expected Ok response, got {:?}", response);
    }

    ////////////////////////////////////////////////////////////////
    //                       AORI_MAKEORDER
    ////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_make_and_cancel() {}

    ////////////////////////////////////////////////////////////////
    //                       AORI_TAKEORDER
    ////////////////////////////////////////////////////////////////

    #[tokio::test]
    async fn test_make_and_take() {}
}
