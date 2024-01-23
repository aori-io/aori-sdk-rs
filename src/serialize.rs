// the json rpc backend is using decimal numbers for most fields, while alloy prefers hex
// therefore this sdk provides de/serializers such that all the rust code can use hex
// while sending and receiving from backend server is using decimals.

use alloy_primitives::U256;
use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;

pub fn serialize_u256_as_u32<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert U256 to u64, this will panic if the value is too large to fit into u64
    let value_u32 = value.to::<u32>();
    // Serialize the u64 value
    serializer.serialize_u32(value_u32)
}
pub fn serialize_u256_as_string<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert U256 to u64, this will panic if the value is too large to fit into u64
    let value_string = value.to_string();
    // Serialize the u64 value
    serializer.serialize_str(&value_string)
}
// Deserialization function for U256 from a u64
pub fn deserialize_u256_from_u32<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let value_u32 = u32::deserialize(deserializer)?;
    Ok(U256::from(value_u32))
}

// Deserialization function for U256 from a String
pub fn deserialize_u256_from_string<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let value_string = String::deserialize(deserializer)?;
    U256::from_str(&value_string).map_err(serde::de::Error::custom)
}
