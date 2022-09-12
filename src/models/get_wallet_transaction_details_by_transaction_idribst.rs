/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetWalletTransactionDetailsByTransactionIdribst : Tron



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetWalletTransactionDetailsByTransactionIdribst {
    /// String representation of the amount value
    #[serde(rename = "amount")]
    pub amount: String,
    /// Numeric representation of the transaction used bandwidth
    #[serde(rename = "bandwidthUsed")]
    pub bandwidth_used: String,
    /// Numeric representation of the transaction contract
    #[serde(rename = "contract")]
    pub contract: String,
    /// String representation of the transaction used energy
    #[serde(rename = "energyUsed")]
    pub energy_used: String,
    #[serde(rename = "hasInternalTransactions")]
    pub has_internal_transactions: bool,
    #[serde(rename = "hasTokenTransfers")]
    pub has_token_transfers: bool,
    /// Numeric representation of the transaction input
    #[serde(rename = "input")]
    pub input: String,
    /// String representation of the transaction status
    #[serde(rename = "status")]
    pub status: String,
}

impl GetWalletTransactionDetailsByTransactionIdribst {
    /// Tron
    pub fn new(amount: String, bandwidth_used: String, contract: String, energy_used: String, has_internal_transactions: bool, has_token_transfers: bool, input: String, status: String) -> GetWalletTransactionDetailsByTransactionIdribst {
        GetWalletTransactionDetailsByTransactionIdribst {
            amount,
            bandwidth_used,
            contract,
            energy_used,
            has_internal_transactions,
            has_token_transfers,
            input,
            status,
        }
    }
}

