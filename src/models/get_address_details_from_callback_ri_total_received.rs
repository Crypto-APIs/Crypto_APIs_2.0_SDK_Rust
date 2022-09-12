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
pub struct GetAddressDetailsFromCallbackRiTotalReceived {
    /// Defines the total amount of all coins received to the address, based on confirmed transactions.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Represents the unit of the total received amount.
    #[serde(rename = "unit")]
    pub unit: String,
}

impl GetAddressDetailsFromCallbackRiTotalReceived {
    pub fn new(amount: String, unit: String) -> GetAddressDetailsFromCallbackRiTotalReceived {
        GetAddressDetailsFromCallbackRiTotalReceived {
            amount,
            unit,
        }
    }
}


