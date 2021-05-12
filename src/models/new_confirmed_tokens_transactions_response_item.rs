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
pub struct NewConfirmedTokensTransactionsResponseItem {
    /// Represents the address of the transaction, per which the result is returned.
    #[serde(rename = "address")]
    pub address: String,
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs.
    #[serde(rename = "callbackSecretKey")]
    pub callback_secret_key: String,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block.
    #[serde(rename = "confirmationsCount")]
    pub confirmations_count: String,
    /// Defines the specific time/date when the subscription was created in Unix Timestamp.
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i32,
    /// Defines the type of the specific event available for the customer to subscribe to for callback notification.
    #[serde(rename = "eventType")]
    pub event_type: String,
    /// Defines whether the subscription is active or not. Set as boolean.
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Represents a unique ID used to reference the specific callback subscription.
    #[serde(rename = "referenceId")]
    pub reference_id: String,
    /// Represents the unique identification string that defines the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

impl NewConfirmedTokensTransactionsResponseItem {
    pub fn new(address: String, callback_secret_key: String, callback_url: String, confirmations_count: String, created_timestamp: i32, event_type: String, is_active: bool, reference_id: String, transaction_id: String) -> NewConfirmedTokensTransactionsResponseItem {
        NewConfirmedTokensTransactionsResponseItem {
            address,
            callback_secret_key,
            callback_url,
            confirmations_count,
            created_timestamp,
            event_type,
            is_active,
            reference_id,
            transaction_id,
        }
    }
}


