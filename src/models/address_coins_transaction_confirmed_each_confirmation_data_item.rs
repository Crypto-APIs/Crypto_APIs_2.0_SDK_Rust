/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// AddressCoinsTransactionConfirmedEachConfirmationDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressCoinsTransactionConfirmedEachConfirmationDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Defines the specific address to which the transaction has been sent.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "minedInBlock")]
    pub mined_in_block: Box<crate::models::AddressCoinsTransactionConfirmedEachConfirmationDataItemMinedInBlock>,
    /// Defines the unique ID of the specific transaction, i.e. its identification number.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Defines the number of currently received confirmations for the transaction.
    #[serde(rename = "currentConfirmations")]
    pub current_confirmations: i32,
    /// Defines the number of confirmation transactions requested as callbacks, i.e. the system can notify till the n-th confirmation.
    #[serde(rename = "targetConfirmations")]
    pub target_confirmations: i32,
    /// Defines the amount of coins sent with the confirmed transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the unit of the transaction, e.g. BTC.
    #[serde(rename = "unit")]
    pub unit: String,
    /// Defines whether the transaction is \"incoming\" or \"outgoing\".
    #[serde(rename = "direction")]
    pub direction: Direction,
}

impl AddressCoinsTransactionConfirmedEachConfirmationDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, address: String, mined_in_block: crate::models::AddressCoinsTransactionConfirmedEachConfirmationDataItemMinedInBlock, transaction_id: String, current_confirmations: i32, target_confirmations: i32, amount: String, unit: String, direction: Direction) -> AddressCoinsTransactionConfirmedEachConfirmationDataItem {
        AddressCoinsTransactionConfirmedEachConfirmationDataItem {
            blockchain,
            network,
            address,
            mined_in_block: Box::new(mined_in_block),
            transaction_id,
            current_confirmations,
            target_confirmations,
            amount,
            unit,
            direction,
        }
    }
}

/// Defines whether the transaction is \"incoming\" or \"outgoing\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "incoming")]
    Incoming,
    #[serde(rename = "outgoing")]
    Outgoing,
}

