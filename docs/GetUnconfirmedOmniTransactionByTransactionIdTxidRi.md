# GetUnconfirmedOmniTransactionByTransactionIdTxidRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | Defines the amount of the sent tokens. | 
**divisible** | **bool** | Defines whether the attribute can be divisible or not, as boolean. E.g., if it is \"true\", the attribute is divisible. | 
**mined** | **bool** | Defines whether the transaction has been mined or not, as boolean. E.g. if set to \"true\", it means the transaction is mined. | 
**property_id** | **i32** | Represents the identifier of the tokens to send. | 
**recipients** | [**Vec<crate::models::GetUnconfirmedOmniTransactionByTransactionIdTxidRiRecipients>**](GetUnconfirmedOmniTransactionByTransactionIDTxidRI_recipients.md) | Represents an object of addresses that receive the transactions. | 
**senders** | [**Vec<crate::models::GetUnconfirmedOmniTransactionByTransactionIdTxidRiSenders>**](GetUnconfirmedOmniTransactionByTransactionIDTxidRI_senders.md) | Represents an object of addresses that provide the funds. | 
**sent** | **bool** | Defines whether the transaction has been sent or not, as boolean. E.g. if set to \"true\", it means the transaction is sent. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_id** | **String** | String representation of the transaction identifier (txid) | 
**_type** | **String** | Defines the type of the transaction as a string. | 
**type_int** | **i32** | Defines the type of the transaction as a number. | 
**version** | **i32** | Defines the specific version. | 
**fee** | [**crate::models::ListUnconfirmedOmniTransactionsByAddressRiFee**](ListUnconfirmedOmniTransactionsByAddressRI_fee.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


