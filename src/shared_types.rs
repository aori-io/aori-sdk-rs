use crate::*; 
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

sol!(
    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriOrder {
        address offerer;
        // input
        address inputToken;
        #[serde(serialize_with = "serialize_u256_as_string", deserialize_with = "deserialize_u256_from_string")]
        uint256 inputAmount;
        #[serde(serialize_with = "serialize_u256_as_u32", deserialize_with = "deserialize_u256_from_u32")]
        uint256 inputChainId;
        address inputZone;
        // output
        address outputToken;
        #[serde(serialize_with = "serialize_u256_as_string", deserialize_with = "deserialize_u256_from_string")]
        uint256 outputAmount;
        #[serde(serialize_with = "serialize_u256_as_u32", deserialize_with = "deserialize_u256_from_u32")]
        uint256 outputChainId;
        address outputZone;
        // other
        #[serde(serialize_with = "serialize_u256_as_string", deserialize_with = "deserialize_u256_from_string")]
        uint256 startTime;
        #[serde(serialize_with = "serialize_u256_as_string", deserialize_with = "deserialize_u256_from_string")]
        uint256 endTime;
        uint256 salt;
        #[serde(serialize_with = "serialize_u256_as_u32", deserialize_with = "deserialize_u256_from_u32")]
        uint256 counter;
        bool toWithdraw;
    }
);

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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{keccak256, Address, U256};
    use alloy_sol_types::SolValue;
    use serde_json;
    #[test]
    fn sign_order() {
        let order = AoriOrder {
            offerer: "0x0000000000000000000000000000000000000001".parse::<Address>().unwrap(),
            inputToken: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
            inputAmount: U256::from(1000000000000000000_u64),
            inputChainId: U256::from(1),
            inputZone: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
            outputToken: "0x0000000000000000000000000000000000000004".parse::<Address>().unwrap(),
            outputAmount: U256::from(2000000000000000000_u64),
            outputChainId: U256::from(1),
            outputZone: "0x0000000000000000000000000000000000000000".parse::<Address>().unwrap(),
            startTime: U256::from(1619827200),
            endTime: U256::from(1622428800),
            salt: U256::from(1),
            counter: U256::from(1),
            toWithdraw: false,
        };
        let packed = order.abi_encode_packed();
        let hash = keccak256(packed);
        println!("rust order hash {}", hash);
        assert_eq!(
            hash.to_string(),
            "0x214356d7e7b271d965916a29d61e111e8106f54a2b805a742c18bf93f9f2372e",
            "Order hash does not match expected value"
        );
    }

    #[test]
    fn serialize_aori_order() {
        let order = AoriOrder {
            offerer: "0x0000000000000000000000000000000000000001".parse::<Address>().unwrap(),
            inputToken: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
            inputAmount: U256::from(1000000000000000000_u64),
            inputChainId: U256::from(1),
            inputZone: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
            outputToken: "0x0000000000000000000000000000000000000004".parse::<Address>().unwrap(),
            outputAmount: U256::from(2000000000000000000_u64),
            outputChainId: U256::from(1),
            outputZone: "0x0000000000000000000000000000000000000000".parse::<Address>().unwrap(),
            startTime: U256::from(1619827200),
            endTime: U256::from(1622428800),
            salt: U256::from(1),
            counter: U256::from(1),
            toWithdraw: false,
        };

        let serialized = serde_json::to_string(&order).unwrap();
        println!("Serialized AoriOrder: {}", serialized);

        // Deserialize the order
        let deserialized: AoriOrder = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized AoriOrder: {:?}", deserialized);
    }
}
