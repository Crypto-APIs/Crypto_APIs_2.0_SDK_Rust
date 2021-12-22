/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAddressDetailsFromCallbackRiTotalSpent {
    /// Defines the total amount of all spent by this address coins, based on confirmed transactions.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Represents the unit of the total spent amount.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl GetAddressDetailsFromCallbackRiTotalSpent {
    pub fn new() -> GetAddressDetailsFromCallbackRiTotalSpent {
        GetAddressDetailsFromCallbackRiTotalSpent {
            amount: None,
            unit: None,
        }
    }
}


