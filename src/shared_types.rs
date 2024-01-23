use crate::*;
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

sol!(
    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriOrder {
        address offerer;
        // input
        address inputToken;
        #[serde(serialize_with = "u256_as_string", deserialize_with = "u256_from_string")]
        uint256 inputAmount;
        #[serde(serialize_with = "u256_as_u32", deserialize_with = "u256_from_u32")]
        uint256 inputChainId;
        address inputZone;
        // output
        address outputToken;
        #[serde(serialize_with = "u256_as_string", deserialize_with = "u256_from_string")]
        uint256 outputAmount;
        #[serde(serialize_with = "u256_as_u32", deserialize_with = "u256_from_u32")]
        uint256 outputChainId;
        address outputZone;
        // other
        #[serde(serialize_with = "u256_as_string", deserialize_with = "u256_from_string")]
        uint256 startTime;
        #[serde(serialize_with = "u256_as_string", deserialize_with = "u256_from_string")]
        uint256 endTime;
        uint256 salt;
        #[serde(serialize_with = "u256_as_u32", deserialize_with = "u256_from_u32")]
        uint256 counter;
        bool toWithdraw;
    }
);

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
