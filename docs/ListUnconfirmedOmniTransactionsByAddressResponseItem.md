# ListUnconfirmedOmniTransactionsByAddressResponseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | Defines the amount of the sent tokens. | 
**divisible** | **bool** | Defines whether the attribute can be divisible or not, as boolean. E.g., if it is \"true\", the attribute is divisible. | 
**mined** | **bool** | Defines whether the transaction has been mined or not, as boolean. E.g. if set to \"true\", it means the transaction is mined. | 
**property_id** | **i32** | Represents the identifier of the tokens to send. | 
**recipients** | [**Vec<crate::models::ListOmniTransactionsByAddressResponseItemRecipients>**](ListOmniTransactionsByAddressResponseItem_recipients.md) | Represents an object of addresses that receive the transactions. | 
**senders** | [**Vec<crate::models::ListUnconfirmedOmniTransactionsByAddressResponseItemSenders>**](ListUnconfirmedOmniTransactionsByAddressResponseItem_senders.md) | Represents an object of addresses that provide the funds. | 
**sent** | **bool** | Defines whether the transaction has been sent or not, as boolean. E.g. if set to \"true\", it means the transaction is sent. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | 
**_type** | **String** | Defines the type of the transaction as a string. | 
**type_int** | **i32** | Defines the type of the transaction as a number. | 
**version** | **i32** | Defines the specific version. | 
**fee** | [**crate::models::ListUnconfirmedOmniTransactionsByAddressResponseItemFee**](ListUnconfirmedOmniTransactionsByAddressResponseItem_fee.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


