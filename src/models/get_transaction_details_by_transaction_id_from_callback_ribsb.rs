/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetTransactionDetailsByTransactionIdFromCallbackRibsb : Bitcoin



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionDetailsByTransactionIdFromCallbackRibsb {
    /// Represents the time at which a particular transaction can be added to the blockchain.
    #[serde(rename = "locktime")]
    pub locktime: i32,
    /// Represents the total size of this transaction.
    #[serde(rename = "size")]
    pub size: i32,
    /// Represents the virtual size of this transaction.
    #[serde(rename = "vSize")]
    pub v_size: i32,
    /// Represents the transaction version number.
    #[serde(rename = "version")]
    pub version: i32,
    /// Represents the transaction inputs.
    #[serde(rename = "vin")]
    pub vin: Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibsbVin>,
    /// Represents the transaction outputs.
    #[serde(rename = "vout")]
    pub vout: Vec<crate::models::GetTransactionDetailsByTransactionIdribsbVout>,
}

impl GetTransactionDetailsByTransactionIdFromCallbackRibsb {
    /// Bitcoin
    pub fn new(locktime: i32, size: i32, v_size: i32, version: i32, vin: Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibsbVin>, vout: Vec<crate::models::GetTransactionDetailsByTransactionIdribsbVout>) -> GetTransactionDetailsByTransactionIdFromCallbackRibsb {
        GetTransactionDetailsByTransactionIdFromCallbackRibsb {
            locktime,
            size,
            v_size,
            version,
            vin,
            vout,
        }
    }
}


