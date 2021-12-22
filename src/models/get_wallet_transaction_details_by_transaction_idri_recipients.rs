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
pub struct GetWalletTransactionDetailsByTransactionIdriRecipients {
    /// The address which receives this transaction. In UTXO-based protocols like Bitcoin there could be several senders while in account-based protocols like Ethereum there is always only one recipient.
    #[serde(rename = "address")]
    pub address: String,
    /// Represents the amount received to this address.
    #[serde(rename = "amount")]
    pub amount: String,
}

impl GetWalletTransactionDetailsByTransactionIdriRecipients {
    pub fn new(address: String, amount: String) -> GetWalletTransactionDetailsByTransactionIdriRecipients {
        GetWalletTransactionDetailsByTransactionIdriRecipients {
            address,
            amount,
        }
    }
}


