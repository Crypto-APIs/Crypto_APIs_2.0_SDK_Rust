/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConvertBitcoinCashAddressRb {
    /// In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "data")]
    pub data: Box<crate::models::ConvertBitcoinCashAddressRbData>,
}

impl ConvertBitcoinCashAddressRb {
    pub fn new(data: crate::models::ConvertBitcoinCashAddressRbData) -> ConvertBitcoinCashAddressRb {
        ConvertBitcoinCashAddressRb {
            context: None,
            data: Box::new(data),
        }
    }
}


