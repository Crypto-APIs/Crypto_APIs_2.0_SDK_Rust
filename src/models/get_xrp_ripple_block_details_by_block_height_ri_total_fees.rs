/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetXrpRippleBlockDetailsByBlockHeightRiTotalFees : Defines the total fees included in the specific block.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetXrpRippleBlockDetailsByBlockHeightRiTotalFees {
    /// Defines the amount of all fees included in the specific block.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the unit of all fees included in the specific block.
    #[serde(rename = "unit")]
    pub unit: String,
}

impl GetXrpRippleBlockDetailsByBlockHeightRiTotalFees {
    /// Defines the total fees included in the specific block.
    pub fn new(amount: String, unit: String) -> GetXrpRippleBlockDetailsByBlockHeightRiTotalFees {
        GetXrpRippleBlockDetailsByBlockHeightRiTotalFees {
            amount,
            unit,
        }
    }
}


