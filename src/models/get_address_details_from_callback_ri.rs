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
pub struct GetAddressDetailsFromCallbackRi {
    /// Defines the count of the incoming transactions.
    #[serde(rename = "incomingTransactionsCount")]
    pub incoming_transactions_count: i32,
    /// Defines the count of the outgoing transactions.
    #[serde(rename = "outgoingTransactionsCount")]
    pub outgoing_transactions_count: i32,
    /// Represents the total number of confirmed coins transactions for this address, both incoming and outgoing. Applies for coins only **and not** tokens transfers e.g. for Ethereum. `transactionsCount` could result as less than incoming and outgoing transactions put together (e.g. in Bitcoin), due to the fact that one and the same address could be in senders and receivers addresses.
    #[serde(rename = "transactionsCount")]
    pub transactions_count: i32,
    #[serde(rename = "confirmedBalance")]
    pub confirmed_balance: Box<crate::models::GetAddressDetailsRiConfirmedBalance>,
    #[serde(rename = "totalReceived", skip_serializing_if = "Option::is_none")]
    pub total_received: Option<Box<crate::models::GetAddressDetailsFromCallbackRiTotalReceived>>,
    #[serde(rename = "totalSpent", skip_serializing_if = "Option::is_none")]
    pub total_spent: Option<Box<crate::models::GetAddressDetailsFromCallbackRiTotalSpent>>,
    /// Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime.
    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

impl GetAddressDetailsFromCallbackRi {
    pub fn new(incoming_transactions_count: i32, outgoing_transactions_count: i32, transactions_count: i32, confirmed_balance: crate::models::GetAddressDetailsRiConfirmedBalance) -> GetAddressDetailsFromCallbackRi {
        GetAddressDetailsFromCallbackRi {
            incoming_transactions_count,
            outgoing_transactions_count,
            transactions_count,
            confirmed_balance: Box::new(confirmed_balance),
            total_received: None,
            total_spent: None,
            sequence: None,
        }
    }
}

