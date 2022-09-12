/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// ListConfirmedTransactionsByAddressAndTimeRangeRibsl : Litecoin



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListConfirmedTransactionsByAddressAndTimeRangeRibsl {
    /// Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid.
    #[serde(rename = "locktime")]
    pub locktime: i64,
    /// Represents the total size of this transaction.
    #[serde(rename = "size")]
    pub size: i32,
    /// Represents the virtual size of this transaction.
    #[serde(rename = "vSize")]
    pub v_size: i32,
    /// Represents the transaction's version number.
    #[serde(rename = "version")]
    pub version: i32,
    /// Represents the transaction inputs.
    #[serde(rename = "vin")]
    pub vin: Vec<crate::models::ListConfirmedTransactionsByAddressRibslVin>,
    /// Represents the transaction outputs.
    #[serde(rename = "vout")]
    pub vout: Vec<crate::models::GetTransactionDetailsByTransactionIdribslVout>,
}

impl ListConfirmedTransactionsByAddressAndTimeRangeRibsl {
    /// Litecoin
    pub fn new(locktime: i64, size: i32, v_size: i32, version: i32, vin: Vec<crate::models::ListConfirmedTransactionsByAddressRibslVin>, vout: Vec<crate::models::GetTransactionDetailsByTransactionIdribslVout>) -> ListConfirmedTransactionsByAddressAndTimeRangeRibsl {
        ListConfirmedTransactionsByAddressAndTimeRangeRibsl {
            locktime,
            size,
            v_size,
            version,
            vin,
            vout,
        }
    }
}


