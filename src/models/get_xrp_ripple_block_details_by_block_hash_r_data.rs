/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetXrpRippleBlockDetailsByBlockHashRData {
    #[serde(rename = "item")]
    pub item: Box<crate::models::GetXrpRippleBlockDetailsByBlockHashRi>,
}

impl GetXrpRippleBlockDetailsByBlockHashRData {
    pub fn new(item: crate::models::GetXrpRippleBlockDetailsByBlockHashRi) -> GetXrpRippleBlockDetailsByBlockHashRData {
        GetXrpRippleBlockDetailsByBlockHashRData {
            item: Box::new(item),
        }
    }
}


