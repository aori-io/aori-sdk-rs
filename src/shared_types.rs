use alloy_serde_macro::{
    bytes_as_string, bytes_from_string, U256_as_String, U256_as_u32, U256_from_String,
    U256_from_u32,
};
use alloy_sol_types::sol;
use serde::{Deserialize, Deserializer, Serialize};

sol!(
    #[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
    struct AoriOrder {
        address offerer;
        // input
        address inputToken;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 inputAmount;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 inputChainId;
        address inputZone;
        // output
        address outputToken;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 outputAmount;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 outputChainId;
        address outputZone;
        // other
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 startTime;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 endTime;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 salt;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 counter;
        bool toWithdraw;
    }

    #[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
    struct OrderView {
        AoriOrder order;
        bytes32 orderHash;
        address inputToken;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 inputAmount;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 inputChainId;
        address outputToken;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 outputAmount;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 outputChainId;
        #[serde(deserialize_with = "deserialize_rate")]
        string rate;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 createdAt;
        bool isPublic;
    }

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct Query {
        address base;
        address quote;
    }

    #[derive(Debug, Deserialize, Serialize)]
    enum SortBy { createdAtAsc, createdAtDesc, rateAsc, rateDesc }

    #[derive(Debug, Deserialize, Serialize)]
    struct ViewOrderbookQuery {
        #[serde(serialize_with = "U256_as_u32")]
        uint256 chainId;
        Query query;
        string side;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 amount;
        SortBy sortBy;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 page;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 perPage;
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct MakeOrderQuery {
        #[serde(serialize_with = "bytes_as_string", deserialize_with = "bytes_from_string")]
        bytes signature;
        AoriOrder order;
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct TakeOrderQuery {
        #[serde(serialize_with = "bytes_as_string", deserialize_with = "bytes_from_string")]
        bytes signature;
        bytes32 orderHash;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 amount;
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct CancelOrderQuery {
        #[serde(serialize_with = "bytes_as_string", deserialize_with = "bytes_from_string")]
        bytes signature;
        bytes32 orderHash;
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct RequestForQuoteQuery {
        #[serde(serialize_with = "bytes_as_string", deserialize_with = "bytes_from_string")]
        bytes signature;
        address inputToken;
        address outputToken;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 inputAmount;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 outputAmount;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 inputChainId;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 outputChainId;
    }
);

pub fn deserialize_rate<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let rate: f64 = Deserialize::deserialize(deserializer)?;
    Ok(rate.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{keccak256, Address, U256};
    use alloy_signer::{Signer, SignerSync, Wallet};
    use alloy_sol_types::SolValue;
    use k256::ecdsa::SigningKey;
    use serde_json;

    #[test]
    fn hash_order() {
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

    #[test]
    fn serialize_view_orderbook_query() {
        // hash wallet address and sign first
        let key = Wallet::<SigningKey>::random_with(&mut rand::thread_rng());
        let address = key.address().to_string();
        let signature = key.sign_message_sync(address.as_bytes()).unwrap();

        let query = ViewOrderbookQuery {
            chainId: U256::from(1),
            query: Query {
                base: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
                quote: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
            },
            side: "buy".to_string(),
            amount: U256::from(1000000000000000000_u64),
            sortBy: SortBy::createdAtAsc,
            page: U256::from(1),
            perPage: U256::from(10),
        };

        let serialized = serde_json::to_string(&query).unwrap();
        println!("Serialized ViewOrderbookQuery: {}", serialized);

        // Deserialize the query
        let deserialized: ViewOrderbookQuery = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized ViewOrderbookQuery: {:?}", deserialized);
    }
}