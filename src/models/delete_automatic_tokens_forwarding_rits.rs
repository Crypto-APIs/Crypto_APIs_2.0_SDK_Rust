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
pub struct DeleteAutomaticTokensForwardingRits {
    /// Defines the `propertyId` of the Omni Layer token.
    #[serde(rename = "propertyId")]
    pub property_id: i32,
    /// Represents the specific `contractAddress` of the Token that will be forwarded.
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

impl DeleteAutomaticTokensForwardingRits {
    pub fn new(property_id: i32, contract_address: String) -> DeleteAutomaticTokensForwardingRits {
        DeleteAutomaticTokensForwardingRits {
            property_id,
            contract_address,
        }
    }
}


