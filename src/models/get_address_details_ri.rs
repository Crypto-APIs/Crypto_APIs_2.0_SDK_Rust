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
pub struct GetAddressDetailsRi {
    /// Represents the total number of confirmed coins transactions for this address, both incoming and outgoing. Applies for coins only **and not** tokens transfers e.g. for Ethereum. `transactionsCount` could result as less than incoming and outgoing transactions put together (e.g. in Bitcoin), due to the fact that one and the same address could be in senders and receivers addresses.
    #[serde(rename = "transactionsCount")]
    pub transactions_count: i32,
    #[serde(rename = "confirmedBalance")]
    pub confirmed_balance: Box<crate::models::GetAddressDetailsRiConfirmedBalance>,
    #[serde(rename = "totalReceived")]
    pub total_received: Box<crate::models::GetAddressDetailsRiTotalReceived>,
    #[serde(rename = "totalSpent")]
    pub total_spent: Box<crate::models::GetAddressDetailsRiTotalSpent>,
    /// Numeric representation of the received transaction count of the address
    #[serde(rename = "incomingTransactionsCount")]
    pub incoming_transactions_count: i32,
    /// Numeric representation of the sent transaction count of the address
    #[serde(rename = "outgoingTransactionsCount")]
    pub outgoing_transactions_count: i32,
}

impl GetAddressDetailsRi {
    pub fn new(transactions_count: i32, confirmed_balance: crate::models::GetAddressDetailsRiConfirmedBalance, total_received: crate::models::GetAddressDetailsRiTotalReceived, total_spent: crate::models::GetAddressDetailsRiTotalSpent, incoming_transactions_count: i32, outgoing_transactions_count: i32) -> GetAddressDetailsRi {
        GetAddressDetailsRi {
            transactions_count,
            confirmed_balance: Box::new(confirmed_balance),
            total_received: Box::new(total_received),
            total_spent: Box::new(total_spent),
            incoming_transactions_count,
            outgoing_transactions_count,
        }
    }
}


