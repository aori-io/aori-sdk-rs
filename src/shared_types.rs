use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_serde_macro::{
    bytes_as_string, bytes_from_string, U256_as_String, U256_as_u32, U256_from_String,
    U256_from_u32,
};
use alloy_sol_types::{sol, SolValue};
use bson::Bson;
use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use tracing::error;

use super::{
    constants::{ChainId, SUPPORTED_AORI_CHAINS},
    get_order_signer, get_signer_address,
};

// abigen!(AoriV2, "src/aori/abi/AoriV2.json");

sol!(AoriV2, "src/abi/AoriV2.json");

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

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriMatchingDetails {
        AoriOrder makerOrder;
        AoriOrder takerOrder;

        bytes makerSignature;
        bytes takerSignature;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 blockDeadline;

        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 seatNumber;
        address seatHolder;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 seatPercentOfFees;
    }

    #[derive(Default, Debug, Deserialize, Serialize)]
    struct AoriMatchingDetailsHashingData {
        bytes makerSignature;
        bytes takerSignature;

        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 blockDeadline;

        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 seatNumber;
        address seatHolder;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 seatPercentOfFees;
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

        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 chainId;

        Query query;
        #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
        uint256 limit;

        bytes32 orderHash;
        address offerer;

        SortBy sortBy;

        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 inputAmount;
        #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
        uint256 outputAmount;

        address zone;
        bool allowStaleQuotes;
    }
);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderView {
    pub order_hash: B256,
    pub offerer: Address,

    pub order: AoriOrder,
    pub signature: String,

    pub input_token: Address,
    #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
    pub input_amount: U256,
    #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
    pub input_chain_id: U256,
    pub input_zone: Address,
    pub output_token: Address,
    #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
    pub output_amount: U256,
    #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
    pub output_chain_id: U256,
    pub output_zone: Address,

    pub rate: String,
    #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
    pub created_at: U256,
    #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
    pub last_updated_at: U256,
    pub is_active: bool,
    pub is_public: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DetailsToExecute {
    pub matching_hash: B256,
    pub matching: AoriMatchingDetails,
    pub matching_signature: String,
    pub maker_order_hash: B256,
    #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
    pub maker_chain_id: U256,
    pub maker_zone: Address,
    pub taker_order_hash: B256,
    #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
    pub taker_chain_id: U256,
    pub taker_zone: Address,
    #[serde(serialize_with = "U256_as_u32", deserialize_with = "U256_from_u32")]
    pub chain_id: U256,
    pub to: Address,
    #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
    pub value: U256,
    #[serde(serialize_with = "bytes_as_string", deserialize_with = "bytes_from_string")]
    pub data: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker_permit_signature: Option<String>,
    pub maker: Address,
    pub taker: Address,
    pub input_token: Address,
    #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
    pub input_amount: U256,
    pub output_token: Address,
    #[serde(serialize_with = "U256_as_String", deserialize_with = "U256_from_String")]
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

// pub fn get_matching_order(order: AoriOrder) -> AoriOrder {

// }

pub fn to_order_view(
    order: AoriOrder,
    signature: String,
    is_active: bool,
    is_public: bool,
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
        is_public,
        is_active,
    }
}

pub fn to_details_to_execute(
    matching: AoriMatchingDetails,
    matching_signature: String,
    to: Address,
    value: U256,
    data: Vec<u8>,
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
            matching.seatPercentOfFees,
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
    maker_signature: Bytes,
    taker_signature: Bytes,
    block_deadline: U256,
    seat_number: U256,
    seat_holder: Address,
    seat_percent_of_fees: U256,
) -> B256 {
    keccak256(
        (AoriMatchingDetailsHashingData {
            makerSignature: maker_signature,
            takerSignature: taker_signature,
            blockDeadline: block_deadline,

            seatNumber: seat_number,
            seatHolder: seat_holder,
            seatPercentOfFees: seat_percent_of_fees,
        })
        .abi_encode_packed(),
    )
}

pub fn calldata_to_settle_orders(matching: AoriMatchingDetails) -> Vec<u8> {
    // TODO: fix
    let mut calldata = Vec::new();
    calldata.extend(matching.abi_encode_packed());
    calldata
}

pub fn document_to_order(document: bson::Document) -> anyhow::Result<OrderView> {
    let bson: Bson = Bson::Document(document);
    let json: Value = match bson {
        Bson::Document(document) => {
            serde_json::to_value(document).expect("Failed to convert Bson to Json")
        }
        _ => {
            let e =
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to convert Bson to Json");
            error!("{}", e);
            return Err(e.into());
        }
    };

    println!("{:?}", json);

    // Deserialize the JSON value into the IntermediateRoot struct
    let order_view: OrderView = match serde_json::from_value(json) {
        Ok(intermediate) => intermediate,
        Err(e) => {
            error!("Failed to deserialize JSON: {}", e);
            return Err(e.into());
        }
    };

    Ok(order_view)
}

pub fn blank_out_signature(order: OrderView) -> OrderView {
    let mut order2 = order.clone();
    order2.signature = "".to_string();
    order2
}

// async function validateOrder(order, signature) {
//     // Check if chain is supported
//     if (!constants_1.SUPPORTED_AORI_CHAINS.has(order.inputChainId))
//         return `Input chain ${order.inputChainId} not supported`;
//     if (!constants_1.SUPPORTED_AORI_CHAINS.has(order.outputChainId))
//         return `Output chain ${order.outputChainId} not supported`;
//     if (signature == undefined || signature == "" || signature == null)
//         return "No signature provided";
//     if (order.inputToken === order.outputToken && order.inputChainId === order.outputChainId)
//         return `Input (${order.inputToken}) and output (${order.outputToken}) tokens must be different if they are on the same chain`;
//     // TODO: reconsider this
//     if (order.inputAmount == "0")
//         return `Input amount cannot be zero`;
//     if (order.outputAmount == "0")
//         return `Output amount cannot be zero`;
//     if (!isZoneSupported(order.inputChainId, order.inputZone))
//         return `Input zone ${order.inputZone} on ${order.inputChainId} not supported`;
//     if (!isZoneSupported(order.outputChainId, order.outputZone))
//         return `Output zone ${order.outputZone} on ${order.outputChainId} not supported`;
//     if (BigInt(order.startTime) > BigInt(order.endTime))
//         return `Start time (${order.startTime}) cannot be after end (${order.endTime}) time`;
//     if (BigInt(order.endTime) < BigInt(Math.floor(Date.now() / 1000)))
//         return `End time (${order.endTime}) cannot be in the past`;
//     // Verify that the signature of the taker order is valid
//     let orderMessageSigner;
//     try {
//         orderMessageSigner = getOrderSigner(order, signature);
//     }
//     catch (e) {
//         return `Signature signer could not be retrieved: ${e.message}`;
//     }
//     try {
//         // make isValidSignature call too
//         if (orderMessageSigner.toLowerCase() !== order.offerer.toLowerCase()) {
//             if (!(await (0, providers_1.isValidSignature)(order.inputChainId, order.offerer, getOrderHash(order), signature))) {
//                 return `Signature (${signature}) appears to be invalid via calling isValidSignature on ${order.offerer} on chain ${order.inputChainId} - order hash: ${getOrderHash(order)}`;
//             }
//         }
//     }
//     catch (e) {
//         return `isValidSignature call failed: ${e.message}`;
//     }
//     return null;
// }

// Note: Some() is used to return an error message if the order is invalid
pub async fn validate_order(order: AoriOrder, signature: String) -> Result<String, &'static str> {
    let order2 = order.clone();

    if !SUPPORTED_AORI_CHAINS().contains(&order2.inputChainId) {
        return Err("Input chain not supported");
    }

    if !SUPPORTED_AORI_CHAINS().contains(&order2.outputChainId) {
        return Err("Output chain not supported");
    }

    if signature == "" || signature == "0x" {
        return Err("No signature provided");
    }

    if order2.inputToken == order2.outputToken && order2.inputChainId == order2.outputChainId {
        return Err("Input token and output token must be different if they are on the same chain");
    }

    if order2.inputAmount == U256::ZERO {
        return Err("Input amount cannot be zero");
    }

    if order2.outputAmount == U256::ZERO {
        return Err("Output amount cannot be zero");
    }

    if order2.startTime > order2.endTime {
        return Err("Start time cannot be after end time");
    }

    if order2.endTime < U256::from(Utc::now().timestamp()) {
        return Err("End time cannot be in the past");
    }

    // Verify that the signature of the taker order is valid
    let order_message_signer = match get_order_signer(order, &signature).await {
        Ok(signer) => signer,
        Err(e) => return Err("Signature signer could not be retrieved"),
    };

    // TODO: add in isValidCall to validate the signature for vaults

    return Ok(order_message_signer.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{keccak256, Address, U256};
    use alloy_sol_types::SolValue;
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

    // #[test]
    // fn serialize_view_orderbook_query() {
    //     // hash wallet address and sign first
    //     let key = Wallet::<SigningKey>::random_with(&mut rand::thread_rng());
    //     let address = key.address().to_string();
    //     let signature = key.sign_message_sync(address.as_bytes()).unwrap();

    //     let query = ViewOrderbookQuery {
    //         offerer: "0x0000000000000000000000000000000000000001".parse::<Address>().unwrap(),
    //         orderHash: alloy_primitives::FixedBytes([0u8; 32]), /* Assuming a 32-byte order hash
    //                                                              * placeholder */
    //         query: Query {
    //             base: "0x0000000000000000000000000000000000000002".parse::<Address>().unwrap(),
    //             quote: "0x0000000000000000000000000000000000000003".parse::<Address>().unwrap(),
    //         },
    //         chainId: U256::from(1),
    //         sortBy: SortBy::createdAtAsc,
    //         inputAmount: U256::from(1000000000000000000_u64),
    //         outputAmount: U256::from(2000000000000000000_u64),
    //         limit: U256::from(10),
    //         zone: "0x0000000000000000000000000000000000000004".parse::<Address>().unwrap(),
    //         allowStaleQuotes: false,
    //     };

    //     let serialized = serde_json::to_string(&query).unwrap();
    //     println!("Serialized ViewOrderbookQuery: {}", serialized);

    //     // Deserialize the query
    //     let deserialized: ViewOrderbookQuery = serde_json::from_str(&serialized).unwrap();
    //     println!("Deserialized ViewOrderbookQuery: {:?}", deserialized);
    // }
}
