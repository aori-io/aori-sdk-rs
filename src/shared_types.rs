use alloy_primitives::{bytes::buf::Take, keccak256, Address, B256, U256};
use alloy_serde_macro::{
    bytes_as_string, bytes_from_string, U256_as_String, U256_as_u32, U256_from_String,
    U256_from_u32,
};
use alloy_sol_types::{sol, SolValue};
use chrono::{Local, Utc};
use serde::{Deserialize, Deserializer, Serialize};

sol!(
    #[derive(Default, Debug, Deserialize, Serialize)]
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
    // #[derive(Default, Debug, Deserialize, Serialize)]
    // struct OrderView {
    //     bytes32 orderHash;
    //     address offerer;

    //     AoriOrder order;
    //     bytes signature;

    //     address inputToken;
    //     uint256 inputAmount;
    //     uint256 inputChainId;
    //     address inputZone;

    //     address outputToken;
    //     uint256 outputAmount;
    //     uint256 outputChainId;
    //     address outputZone;

    //     #[serde(deserialize_with = "deserialize_rate")]
    //     string rate;
    //     uint256 createdAt;
    //     uint256 lastUpdatedAt;
    //     bool isActive;
    //     bool isPublic;
    // }

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriMatchingDetails {
        AoriOrder makerOrder;
        AoriOrder takerOrder;
        
        bytes makerSignature;
        bytes takerSignature;
        uint256 blockDeadline;

        uint256 seatNumber;
        address seatHolder;
        uint256 seatPercentOfFees;
    }

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriMatchingDetailsHashingData {
        bytes makerSignature;
        bytes takerSignature;
        uint256 blockDeadline;

        uint256 seatNumber;
        address seatHolder;
        uint256 seatPercentOfFees;
    }

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct Query {
        address base;
        // #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        address quote;
    }

    #[derive(Debug, Deserialize, Serialize)]
    enum SortBy { createdAtAsc, createdAtDesc, rateAsc, rateDesc }

    #[derive(Debug, Deserialize, Serialize)]
    struct ViewOrderbookQuery {

        #[serde(serialize_with = "U256_as_u32")]
        uint256 chainId;

        Query query;
        #[serde(serialize_with = "U256_as_u32")]
        uint256 limit;

        bytes32 orderHash;
        address offerer;

        SortBy sortBy;

        #[serde(serialize_with = "U256_as_String")]
        uint256 inputAmount;
        #[serde(serialize_with = "U256_as_String")]
        uint256 outputAmount;

        address zone;
        bool allowStaleQuotes;
    }
);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderView {
    #[serde(rename = "orderHash")]
    pub order_hash: B256,
    pub offerer: Address,

    pub order: AoriOrder,
    pub signature: String,

    #[serde(rename = "inputToken")]
    pub input_token: Address,
    #[serde(rename = "inputAmount")]
    pub input_amount: U256,
    #[serde(rename = "inputChainId")]
    pub input_chain_id: U256,
    #[serde(rename = "inputZone")]
    pub input_zone: Address,
    #[serde(rename = "outputToken")]
    pub output_token: Address,
    #[serde(rename = "outputAmount")]
    pub output_amount: U256,
    #[serde(rename = "outputChainId")]
    pub output_chain_id: U256,
    #[serde(rename = "outputZone")]
    pub output_zone: Address,

    pub rate: String,
    #[serde(rename = "createdAt")]
    pub created_at: U256,
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: U256,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}

// export interface DetailsToExecute {
//     matchingHash: string;
//     matching: AoriMatchingDetails;
//     matchingSignature: string;
//     makerOrderHash: string;
//     makerChainId: number;
//     makerZone: string;
//     takerOrderHash: string;
//     takerChainId: number;
//     takerZone: string;
//     chainId: number;
//     to: string;
//     value: number;
//     data: string;
//     takerPermitSignature?: string;
//     maker: string;
//     taker: string;
//     inputToken: string;
//     inputAmount: string;
//     outputToken: string;
//     outputAmount: string;
// }

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DetailsToExecute {
    #[serde(rename = "matchingHash")]
    pub matching_hash: B256,
    pub matching: AoriMatchingDetails,
    #[serde(rename = "matchingSignature")]
    pub matching_signature: String,
    #[serde(rename = "makerOrderHash")]
    pub maker_order_hash: B256,
    #[serde(rename = "makerChainId")]
    pub maker_chain_id: U256,
    #[serde(rename = "makerZone")]
    pub maker_zone: Address,
    #[serde(rename = "takerOrderHash")]
    pub taker_order_hash: B256,
    #[serde(rename = "takerChainId")]
    pub taker_chain_id: U256,
    #[serde(rename = "takerZone")]
    pub taker_zone: Address,
    #[serde(rename = "chainId")]
    pub chain_id: U256,
    pub to: Address,
    pub value: U256,
    pub data: String,
    #[serde(rename = "takerPermitSignature", skip_serializing_if = "Option::is_none")]
    pub taker_permit_signature: Option<String>,
    pub maker: Address,
    pub taker: Address,
    #[serde(rename = "inputToken")]
    pub input_token: Address,
    #[serde(rename = "inputAmount")]
    pub input_amount: U256,
    #[serde(rename = "outputToken")]
    pub output_token: Address,
    #[serde(rename = "outputAmount")]
    pub output_amount: U256,
}

pub fn deserialize_rate<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let rate: f64 = Deserialize::deserialize(deserializer)?;
    Ok(rate.to_string())
}

pub fn get_order_hash(order: AoriOrder) -> B256 {
    keccak256(order.abi_encode_packed())
}

pub fn to_order_view(
    order: AoriOrder,
    signature: String,
    is_active: bool,
    is_public: bool
) -> OrderView {

    let order2 = order.clone();

    OrderView {
        offerer: order.offerer.clone(),
        signature: signature.clone(),
        input_token: order.inputToken.clone(),
        input_amount: order.inputAmount.clone(),
        input_chain_id: order.inputChainId.clone(),
        input_zone: order.inputZone.clone(),
        output_token: order.outputToken.clone(),
        output_amount: order.outputAmount.clone(),
        output_chain_id: order.outputChainId.clone(),
        output_zone: order.outputZone.clone(),

        rate: (order.outputAmount.clone() / order.inputAmount.clone()).to_string(),
        order: order2,
        order_hash: get_order_hash(order),

        created_at: U256::from(Utc::now().timestamp()),
        last_updated_at: U256::from(Utc::now().timestamp()),
        is_public: is_public,
        is_active: is_active,
    }
}

pub fn to_details_to_execute(
    matching: AoriMatchingDetails,
    matching_signature: String,
    to: Address,
    value: U256,
    data: String,
) -> DetailsToExecute {
    let matching2 = matching.clone();
    let maker_order = matching.makerOrder.clone();
    let taker_order = matching.takerOrder.clone();

    DetailsToExecute {
        matching_hash: get_matching_hash(
            matching.makerSignature, 
            matching.takerSignature,
            matching.blockDeadline,
            matching.seatNumber,
            matching.seatHolder,
            matching.seatPercentOfFees
        ),
        matching_signature,
        maker_chain_id: maker_order.inputChainId,
        maker_zone: maker_order.inputZone,
        taker_chain_id: taker_order.inputChainId,
        taker_zone: taker_order.inputZone,
        chain_id: matching.blockDeadline,
        to,
        value,
        data,
        taker_permit_signature: None,
        maker: maker_order.clone().offerer,
        taker: taker_order.clone().offerer,
        input_token: maker_order.clone().inputToken,
        input_amount: maker_order.clone().inputAmount,
        output_token: taker_order.outputToken,
        output_amount: taker_order.outputAmount,
        maker_order_hash: get_order_hash(maker_order),
        taker_order_hash: get_order_hash(taker_order),
        matching: matching2,
    }
}

pub fn get_matching_hash(
    maker_signature: Vec<u8>,
    taker_signature: Vec<u8>,
    block_deadline: U256,
    seat_number: U256,
    seat_holder: Address,
    seat_percent_of_fees: U256
) -> B256 {

    keccak256((AoriMatchingDetailsHashingData {
        makerSignature: maker_signature,
        takerSignature: taker_signature,
        blockDeadline: block_deadline,

        seatNumber: seat_number,
        seatHolder: seat_holder,
        seatPercentOfFees: seat_percent_of_fees,
    }).abi_encode_packed())
}

pub fn calldata_to_settle_orders(matching: AoriMatchingDetails) -> Vec<u8> {
    // TODO: fix
    let mut calldata = Vec::new();
    calldata.extend(matching.abi_encode_packed());
    calldata
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
            offerer: "0x0000000000000000000000000000000000000001".parse::<Address>().unwrap(),
            orderHash: alloy_primitives::FixedBytes([0u8; 32]), /* Assuming a 32-byte order hash
                                                                 * placeholder */
            query: Query {
                base: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
                quote: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
            },
            chainId: U256::from(1),
            sortBy: SortBy::createdAtAsc,
            inputAmount: U256::from(1000000000000000000_u64),
            outputAmount: U256::from(2000000000000000000_u64),
            limit: U256::from(10),
            zone: "0x0000000000000000000000000000000000000004".parse::<Address>().unwrap(),
            allowStaleQuotes: false,
        };

        let serialized = serde_json::to_string(&query).unwrap();
        println!("Serialized ViewOrderbookQuery: {}", serialized);

        // Deserialize the query
        let deserialized: ViewOrderbookQuery = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized ViewOrderbookQuery: {:?}", deserialized);
    }
}

