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
pub struct CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRi {
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey", skip_serializing_if = "Option::is_none")]
    pub callback_secret_key: Option<String>,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`.
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// Represents an optional note to add a free text in, explaining or providing additional detail on the transaction request.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Defines the destination for the transaction, i.e. the recipient(s).
    #[serde(rename = "recipient")]
    pub recipient: Vec<crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRiRecipient>,
    #[serde(rename = "sender")]
    pub sender: Box<crate::models::CreateSingleTransactionRequestFromAddressWithoutFeePriorityRiSender>,
    #[serde(rename = "tokenTypeSpecificData")]
    pub token_type_specific_data: Box<crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRis>,
    /// Represents a unique identifier of the transaction request (the request sent to make a transaction), which helps in identifying which callback and which `referenceId` concern that specific transaction request.
    #[serde(rename = "transactionRequestId")]
    pub transaction_request_id: String,
}

impl CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRi {
    pub fn new(recipient: Vec<crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRiRecipient>, sender: crate::models::CreateSingleTransactionRequestFromAddressWithoutFeePriorityRiSender, token_type_specific_data: crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRis, transaction_request_id: String) -> CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRi {
        CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRi {
            callback_secret_key: None,
            callback_url: None,
            note: None,
            recipient,
            sender: Box::new(sender),
            token_type_specific_data: Box::new(token_type_specific_data),
            transaction_request_id,
        }
    }
}


