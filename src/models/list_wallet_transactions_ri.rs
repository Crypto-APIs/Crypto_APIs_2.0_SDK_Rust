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
pub struct ListWalletTransactionsRi {
    /// Defines the direction of the transaction, e.g. incoming.
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::ListWalletTransactionsRiFee>,
    /// Represents fungible tokens'es detailed information
    #[serde(rename = "fungibleTokens", skip_serializing_if = "Option::is_none")]
    pub fungible_tokens: Option<Vec<crate::models::ListWalletTransactionsRiFungibleTokens>>,
    #[serde(rename = "internalTransactions", skip_serializing_if = "Option::is_none")]
    pub internal_transactions: Option<Vec<crate::models::ListWalletTransactionsRiInternalTransactions>>,
    /// Represents non-fungible tokens'es detailed information.
    #[serde(rename = "nonFungibleTokens", skip_serializing_if = "Option::is_none")]
    pub non_fungible_tokens: Option<Vec<crate::models::ListWalletTransactionsRiNonFungibleTokens>>,
    /// Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::ListWalletTransactionsRiRecipients>,
    /// Represents a list of sender addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "senders")]
    pub senders: Vec<crate::models::ListWalletTransactionsRiSenders>,
    /// Defines the status of the transaction, if it is confirmed or unconfirmed.
    #[serde(rename = "status")]
    pub status: String,
    /// Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the unique TD of the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "value")]
    pub value: Box<crate::models::ListWalletTransactionsRiValue>,
}

impl ListWalletTransactionsRi {
    pub fn new(direction: String, fee: crate::models::ListWalletTransactionsRiFee, recipients: Vec<crate::models::ListWalletTransactionsRiRecipients>, senders: Vec<crate::models::ListWalletTransactionsRiSenders>, status: String, timestamp: i32, transaction_id: String, value: crate::models::ListWalletTransactionsRiValue) -> ListWalletTransactionsRi {
        ListWalletTransactionsRi {
            direction,
            fee: Box::new(fee),
            fungible_tokens: None,
            internal_transactions: None,
            non_fungible_tokens: None,
            recipients,
            senders,
            status,
            timestamp,
            transaction_id,
            value: Box::new(value),
        }
    }
}


