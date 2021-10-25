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
pub struct SyncHdWalletXPubYPubZPubRb {
    /// In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "data")]
    pub data: Box<crate::models::SyncHdWalletXPubYPubZPubRbData>,
}

impl SyncHdWalletXPubYPubZPubRb {
    pub fn new(data: crate::models::SyncHdWalletXPubYPubZPubRbData) -> SyncHdWalletXPubYPubZPubRb {
        SyncHdWalletXPubYPubZPubRb {
            context: None,
            data: Box::new(data),
        }
    }
}


