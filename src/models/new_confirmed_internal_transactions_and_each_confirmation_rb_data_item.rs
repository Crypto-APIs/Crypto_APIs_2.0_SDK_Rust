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
pub struct NewConfirmedInternalTransactionsAndEachConfirmationRbDataItem {
    /// Defines the specific address of the internal transaction.
    #[serde(rename = "address")]
    pub address: String,
    /// Specifies a flag that permits or denies the creation of duplicate addresses.
    #[serde(rename = "allowDuplicates")]
    pub allow_duplicates: bool,
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey")]
    pub callback_secret_key: String,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs.
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    /// Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block.
    #[serde(rename = "confirmationsCount")]
    pub confirmations_count: i32,
}

impl NewConfirmedInternalTransactionsAndEachConfirmationRbDataItem {
    pub fn new(address: String, allow_duplicates: bool, callback_secret_key: String, callback_url: String, confirmations_count: i32) -> NewConfirmedInternalTransactionsAndEachConfirmationRbDataItem {
        NewConfirmedInternalTransactionsAndEachConfirmationRbDataItem {
            address,
            allow_duplicates,
            callback_secret_key,
            callback_url,
            confirmations_count,
        }
    }
}


