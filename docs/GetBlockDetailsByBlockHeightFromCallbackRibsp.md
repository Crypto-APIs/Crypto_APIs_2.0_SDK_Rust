# GetBlockDetailsByBlockHeightFromCallbackRibsp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**difficulty** | **String** | Represents a mathematical value of how hard it is to find a valid hash for this block. | 
**extra_data** | **String** | Represents any data that can be included by the miner in the block. | 
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_used** | **String** | Represents the exact unit of gas that was used for the transaction. | 
**mined_in_seconds** | **i32** | Specifies the amount of time required for the block to be mined in seconds. | 
**nonce** | **String** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**sha3_uncles** | **String** | Defines the combined hash of all uncles for a given parent. | 
**size** | **i32** | Represents the total size of the block in Bytes. | 
**total_difficulty** | **String** | Defines the total difficulty of the chain until this block, i.e. how difficult it is for a specific miner to mine a new block. | 
**uncles** | **Vec<String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


