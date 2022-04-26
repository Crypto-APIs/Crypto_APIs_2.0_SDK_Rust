# ListUnspentTransactionOutputsByAddressRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | Represents the address that has unspend funds per which the result is returned. | 
**amount** | **String** | Represents the sent/received amount. | 
**index** | **i32** | Represents the index position of the transaction in the block. | 
**is_confirmed** | **bool** | Represents the state of the transaction whether it is confirmed or not confirmed. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


