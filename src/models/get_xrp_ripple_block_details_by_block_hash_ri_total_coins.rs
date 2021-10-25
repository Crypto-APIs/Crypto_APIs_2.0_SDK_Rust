/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetXrpRippleBlockDetailsByBlockHashRiTotalCoins : Defines the total coins.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetXrpRippleBlockDetailsByBlockHashRiTotalCoins {
    /// Defines the amount of the total coins.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the unit of the total coins.
    #[serde(rename = "unit")]
    pub unit: String,
}

impl GetXrpRippleBlockDetailsByBlockHashRiTotalCoins {
    /// Defines the total coins.
    pub fn new(amount: String, unit: String) -> GetXrpRippleBlockDetailsByBlockHashRiTotalCoins {
        GetXrpRippleBlockDetailsByBlockHashRiTotalCoins {
            amount,
            unit,
        }
    }
}


