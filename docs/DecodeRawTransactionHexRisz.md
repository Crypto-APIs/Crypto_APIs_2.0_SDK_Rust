# DecodeRawTransactionHexRisz

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**locktime** | **i32** | Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**saplinged** | **bool** | Defines if the transaction includes sapling or not. | 
**transaction_hash** | **String** | Represents the same as transactionId for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols hash is different from transactionId for SegWit transactions. | 
**value_balance** | **String** | Defines the transaction value balance. | 
**version** | **i32** | Represents the transaction version number. | 
**version_group_id** | **String** | Represents the transaction version group ID | 
**vin** | [**Vec<crate::models::DecodeRawTransactionHexRiszVin>**](DecodeRawTransactionHexRISZ_vin.md) | Represents the Inputs of the transaction | 
**vout** | [**Vec<crate::models::DecodeRawTransactionHexRiszVout>**](DecodeRawTransactionHexRISZ_vout.md) | Represents the Inputs of the transaction | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


