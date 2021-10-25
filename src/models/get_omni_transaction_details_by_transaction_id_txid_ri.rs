/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOmniTransactionDetailsByTransactionIdTxidRi {
    /// Defines the amount of the sent tokens.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines whether the attribute can be divisible or not, as boolean. E.g., if it is \"true\", the attribute is divisible.
    #[serde(rename = "divisible")]
    pub divisible: bool,
    /// Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "minedInBlockHash")]
    pub mined_in_block_hash: String,
    /// Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block.
    #[serde(rename = "minedInBlockHeight")]
    pub mined_in_block_height: i32,
    /// Represents the identifier of the tokens to send.
    #[serde(rename = "propertyId")]
    pub property_id: i32,
    /// Represents an object of addresses that receive the transactions.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::ListOmniTransactionsByAddressRiRecipients>,
    /// Represents an object of addresses that provide the funds.
    #[serde(rename = "senders")]
    pub senders: Vec<crate::models::GetOmniTransactionDetailsByTransactionIdTxidRiSenders>,
    /// Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Defines the type of the transaction as a string.
    #[serde(rename = "type")]
    pub _type: String,
    /// Defines the type of the transaction as a number.
    #[serde(rename = "typeInt")]
    pub type_int: i32,
    /// Defines whether the transaction is valid or not, as boolean. E.g. if set to \"true\", it means the transaction is valid.
    #[serde(rename = "valid")]
    pub valid: bool,
    /// Defines the specific version.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::ListUnconfirmedOmniTransactionsByAddressRiFee>,
}

impl GetOmniTransactionDetailsByTransactionIdTxidRi {
    pub fn new(amount: String, divisible: bool, mined_in_block_hash: String, mined_in_block_height: i32, property_id: i32, recipients: Vec<crate::models::ListOmniTransactionsByAddressRiRecipients>, senders: Vec<crate::models::GetOmniTransactionDetailsByTransactionIdTxidRiSenders>, timestamp: i32, transaction_id: String, _type: String, type_int: i32, valid: bool, version: i32, fee: crate::models::ListUnconfirmedOmniTransactionsByAddressRiFee) -> GetOmniTransactionDetailsByTransactionIdTxidRi {
        GetOmniTransactionDetailsByTransactionIdTxidRi {
            amount,
            divisible,
            mined_in_block_hash,
            mined_in_block_height,
            property_id,
            recipients,
            senders,
            timestamp,
            transaction_id,
            _type,
            type_int,
            valid,
            version,
            fee: Box::new(fee),
        }
    }
}


