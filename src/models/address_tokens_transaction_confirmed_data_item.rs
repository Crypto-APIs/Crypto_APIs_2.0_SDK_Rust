/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// AddressTokensTransactionConfirmedDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressTokensTransactionConfirmedDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\",  are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Defines the specific address to which the transaction has been sent.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "minedInBlock")]
    pub mined_in_block: Box<crate::models::AddressTokensTransactionConfirmedDataItemMinedInBlock>,
    /// Defines the unique ID of the specific transaction, i.e. its identification number.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Defines the type of token sent with the transaction, e.g. ERC 20.
    #[serde(rename = "tokenType")]
    pub token_type: TokenType,
    #[serde(rename = "token")]
    pub token: Box<crate::models::AddressTokensTransactionConfirmedToken>,
    /// Defines whether the transaction is \"incoming\" or \"outgoing\".
    #[serde(rename = "direction")]
    pub direction: Direction,
}

impl AddressTokensTransactionConfirmedDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, address: String, mined_in_block: crate::models::AddressTokensTransactionConfirmedDataItemMinedInBlock, transaction_id: String, token_type: TokenType, token: crate::models::AddressTokensTransactionConfirmedToken, direction: Direction) -> AddressTokensTransactionConfirmedDataItem {
        AddressTokensTransactionConfirmedDataItem {
            blockchain,
            network,
            address,
            mined_in_block: Box::new(mined_in_block),
            transaction_id,
            token_type,
            token: Box::new(token),
            direction,
        }
    }
}

/// Defines the type of token sent with the transaction, e.g. ERC 20.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "ERC-20")]
    ERC20,
    #[serde(rename = "ERC-721")]
    ERC721,
    #[serde(rename = "OMNI")]
    OMNI,
    #[serde(rename = "BEP-20")]
    BEP20,
}
/// Defines whether the transaction is \"incoming\" or \"outgoing\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "incoming")]
    Incoming,
    #[serde(rename = "outgoing")]
    Outgoing,
}

