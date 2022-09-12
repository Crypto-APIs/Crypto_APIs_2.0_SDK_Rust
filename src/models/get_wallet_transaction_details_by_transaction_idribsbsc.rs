/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetWalletTransactionDetailsByTransactionIdribsbsc : Binance Smart Chain



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetWalletTransactionDetailsByTransactionIdribsbsc {
    /// Represents the specific transaction contract
    #[serde(rename = "contract")]
    pub contract: String,
    /// Represents the amount of gas used by this specific transaction alone.
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: Box<crate::models::GetTransactionDetailsByTransactionIdribsbscGasPrice>,
    /// Defines the unit of the gas price amount, e.g. BTC, ETH, XRP.
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    /// Represents additional information that is required for the transaction.
    #[serde(rename = "inputData")]
    pub input_data: String,
    /// Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address.
    #[serde(rename = "nonce")]
    pub nonce: i32,
}

impl GetWalletTransactionDetailsByTransactionIdribsbsc {
    /// Binance Smart Chain
    pub fn new(contract: String, gas_limit: String, gas_price: crate::models::GetTransactionDetailsByTransactionIdribsbscGasPrice, gas_used: String, input_data: String, nonce: i32) -> GetWalletTransactionDetailsByTransactionIdribsbsc {
        GetWalletTransactionDetailsByTransactionIdribsbsc {
            contract,
            gas_limit,
            gas_price: Box::new(gas_price),
            gas_used,
            input_data,
            nonce,
        }
    }
}


