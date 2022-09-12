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
pub struct TokensForwardingSuccessToken {
    /// Specifies the name of the token.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies an identifier of the token, where up to five alphanumeric characters can be used for it.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Defines how many decimals can be used to break the token.
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<String>,
    /// Defines the amount of tokens sent with the confirmed transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Specifies the address of the contract.
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// Specifies the ID of the token.
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// Defines the ID of the property for Omni Layer.
    #[serde(rename = "propertyId")]
    pub property_id: String,
    /// Defines the type of the transaction.
    #[serde(rename = "transactionType")]
    pub transaction_type: String,
    /// The transaction ID used to create the token.
    #[serde(rename = "createdByTransactionId")]
    pub created_by_transaction_id: String,
}

impl TokensForwardingSuccessToken {
    pub fn new(name: String, symbol: String, amount: String, contract_address: String, token_id: String, property_id: String, transaction_type: String, created_by_transaction_id: String) -> TokensForwardingSuccessToken {
        TokensForwardingSuccessToken {
            name,
            symbol,
            decimals: None,
            amount,
            contract_address,
            token_id,
            property_id,
            transaction_type,
            created_by_transaction_id,
        }
    }
}


