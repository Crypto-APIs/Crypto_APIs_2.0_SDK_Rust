# ListOmniTransactionsByBlockHashResponseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | Defines the amount of the sent tokens. | 
**divisible** | **bool** | Defines whether the attribute can be divisible or not, as boolean. E.g., if it is \"true\", the attribute is divisible. | 
**mined_in_block_hash** | **String** | Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**mined_in_block_height** | **i32** | Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block. | 
**position_in_block** | **i32** | Represents the index position of the transaction in the specific block. | 
**property_id** | **i32** | Represents the identifier of the tokens to send. | 
**recipients** | [**Vec<crate::models::ListOmniTransactionsByAddressResponseItemRecipients>**](ListOmniTransactionsByAddressResponseItem_recipients.md) | Represents an object of addresses that receive the transactions. | 
**senders** | [**Vec<crate::models::ListOmniTransactionsByAddressResponseItemSenders>**](ListOmniTransactionsByAddressResponseItem_senders.md) | Represents an object of addresses that provide the funds. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | 
**_type** | **String** | Defines the type of the transaction as a string. | 
**type_int** | **i32** | Defines the type of the transaction as a number. | 
**valid** | **bool** | Defines whether the transaction is valid or not, as boolean. E.g., if it is \"true\", the transaction is valid. | 
**version** | **i32** | Defines the specific version. | 
**fee** | [**crate::models::ListOmniTransactionsByBlockHashResponseItemFee**](ListOmniTransactionsByBlockHashResponseItem_fee.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


