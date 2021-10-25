/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// BlockMinedData : Specifies all data, as attributes, included into the callback notification, which depends on the `event`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockMinedData {
    /// Represents the Crypto APIs 2.0 product which sends the callback.
    #[serde(rename = "product")]
    pub product: String,
    /// Defines the specific event, for which a callback subscription is set.
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "item")]
    pub item: Box<crate::models::BlockMinedDataItem>,
}

impl BlockMinedData {
    /// Specifies all data, as attributes, included into the callback notification, which depends on the `event`.
    pub fn new(product: String, event: String, item: crate::models::BlockMinedDataItem) -> BlockMinedData {
        BlockMinedData {
            product,
            event,
            item: Box::new(item),
        }
    }
}


