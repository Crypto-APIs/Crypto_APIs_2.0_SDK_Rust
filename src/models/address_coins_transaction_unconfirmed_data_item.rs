/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// AddressCoinsTransactionUnconfirmedDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressCoinsTransactionUnconfirmedDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Defines the specific address to which the coin transaction has been sent and is pending confirmation.
    #[serde(rename = "address")]
    pub address: String,
    /// Defines the unique ID of the specific transaction, i.e. its identification number.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Defines the amount of coins sent with the transaction that is pending confirmation.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the unit of the transaction, e.g. BTC.
    #[serde(rename = "unit")]
    pub unit: Unit,
    /// Defines whether the transaction is \"incoming\" or \"outgoing\".
    #[serde(rename = "direction")]
    pub direction: Direction,
    /// Defines the exact time the transaction has been first accepted into the mempool to await confirmation as timestamp.
    #[serde(rename = "firstSeenInMempoolTimestamp")]
    pub first_seen_in_mempool_timestamp: i32,
}

impl AddressCoinsTransactionUnconfirmedDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, address: String, transaction_id: String, amount: String, unit: Unit, direction: Direction, first_seen_in_mempool_timestamp: i32) -> AddressCoinsTransactionUnconfirmedDataItem {
        AddressCoinsTransactionUnconfirmedDataItem {
            blockchain,
            network,
            address,
            transaction_id,
            amount,
            unit,
            direction,
            first_seen_in_mempool_timestamp,
        }
    }
}

/// Defines the unit of the transaction, e.g. BTC.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Unit {
    #[serde(rename = "btc")]
    Btc,
    #[serde(rename = "satoshi")]
    Satoshi,
    #[serde(rename = "wei")]
    Wei,
    #[serde(rename = "gwei")]
    Gwei,
    #[serde(rename = "eth")]
    Eth,
    #[serde(rename = "doge")]
    Doge,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "etc")]
    Etc,
    #[serde(rename = "xrp")]
    Xrp,
    #[serde(rename = "zil")]
    Zil,
}
/// Defines whether the transaction is \"incoming\" or \"outgoing\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "incoming")]
    Incoming,
    #[serde(rename = "outgoing")]
    Outgoing,
}

