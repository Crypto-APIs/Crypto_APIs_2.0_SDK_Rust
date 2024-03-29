/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetXrpRippleTransactionDetailsByTransactionIdriOffer : Defines the specific offer.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetXrpRippleTransactionDetailsByTransactionIdriOffer {
    /// Defines the amount of the offer.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the unit of the offer.
    #[serde(rename = "unit")]
    pub unit: String,
}

impl GetXrpRippleTransactionDetailsByTransactionIdriOffer {
    /// Defines the specific offer.
    pub fn new(amount: String, unit: String) -> GetXrpRippleTransactionDetailsByTransactionIdriOffer {
        GetXrpRippleTransactionDetailsByTransactionIdriOffer {
            amount,
            unit,
        }
    }
}


