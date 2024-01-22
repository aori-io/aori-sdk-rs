use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriPingResponse(pub String);

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AoriAuthResponse {
    pub auth: String,
}