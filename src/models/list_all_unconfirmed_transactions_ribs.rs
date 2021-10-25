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
pub struct ListAllUnconfirmedTransactionsRibs {
    /// Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid.
    #[serde(rename = "locktime")]
    pub locktime: i32,
    /// Represents the total size of this transaction.
    #[serde(rename = "size")]
    pub size: i32,
    /// Represents the virtual size of this transaction.
    #[serde(rename = "vSize")]
    pub v_size: i32,
    /// Defines the version of the transaction.
    #[serde(rename = "version")]
    pub version: i32,
    /// Object Array representation of transaction inputs
    #[serde(rename = "vin")]
    pub vin: Vec<crate::models::ListUnconfirmedTransactionsByAddressRibszVin>,
    /// Object Array representation of transaction outputs
    #[serde(rename = "vout")]
    pub vout: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVout>,
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::ListAllUnconfirmedTransactionsRibsecFee>,
    /// Represents the amount of gas used by this specific transaction alone.
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: Box<crate::models::ListConfirmedTransactionsByAddressRibsbscGasPrice>,
    /// Represents additional information that is required for the transaction.
    #[serde(rename = "inputData")]
    pub input_data: String,
    /// Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address.
    #[serde(rename = "nonce")]
    pub nonce: i32,
    /// Defines the transaction status.
    #[serde(rename = "transactionStatus")]
    pub transaction_status: String,
    /// It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions.
    #[serde(rename = "bindingSig")]
    pub binding_sig: String,
    /// Represents a block height after which the transaction will expire.
    #[serde(rename = "expiryHeight")]
    pub expiry_height: i32,
    /// Represents an encoding of a JoinSplitSig public validating key.
    #[serde(rename = "joinSplitPubKey")]
    pub join_split_pub_key: String,
    /// Is used to sign transactions that contain at least one JoinSplit description.
    #[serde(rename = "joinSplitSig")]
    pub join_split_sig: String,
    /// \"Overwinter\" is the network upgrade for the Zcash blockchain.
    #[serde(rename = "overwintered")]
    pub overwintered: bool,
    /// Represents a sequence of JoinSplit descriptions using BCTV14 proofs.
    #[serde(rename = "vJoinSplit")]
    pub v_join_split: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVJoinSplit>,
    /// Object Array representation of transaction output descriptions
    #[serde(rename = "vShieldedOutput")]
    pub v_shielded_output: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVShieldedOutput>,
    /// Object Array representation of transaction spend descriptions
    #[serde(rename = "vShieldedSpend")]
    pub v_shielded_spend: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVShieldedSpend>,
    /// Defines the transaction value balance.
    #[serde(rename = "valueBalance")]
    pub value_balance: String,
    /// Represents the transaction version group ID.
    #[serde(rename = "versionGroupId")]
    pub version_group_id: String,
}

impl ListAllUnconfirmedTransactionsRibs {
    pub fn new(locktime: i32, size: i32, v_size: i32, version: i32, vin: Vec<crate::models::ListUnconfirmedTransactionsByAddressRibszVin>, vout: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVout>, fee: crate::models::ListAllUnconfirmedTransactionsRibsecFee, gas_limit: String, gas_price: crate::models::ListConfirmedTransactionsByAddressRibsbscGasPrice, input_data: String, nonce: i32, transaction_status: String, binding_sig: String, expiry_height: i32, join_split_pub_key: String, join_split_sig: String, overwintered: bool, v_join_split: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVJoinSplit>, v_shielded_output: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVShieldedOutput>, v_shielded_spend: Vec<crate::models::ListConfirmedTransactionsByAddressRibszVShieldedSpend>, value_balance: String, version_group_id: String) -> ListAllUnconfirmedTransactionsRibs {
        ListAllUnconfirmedTransactionsRibs {
            locktime,
            size,
            v_size,
            version,
            vin,
            vout,
            fee: Box::new(fee),
            gas_limit,
            gas_price: Box::new(gas_price),
            input_data,
            nonce,
            transaction_status,
            binding_sig,
            expiry_height,
            join_split_pub_key,
            join_split_sig,
            overwintered,
            v_join_split,
            v_shielded_output,
            v_shielded_spend,
            value_balance,
            version_group_id,
        }
    }
}

