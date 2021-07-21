# GetZilliqaBlockDetailsByBlockHeightRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**difficulty** | **String** | Defines how difficult it is for a specific miner to mine the block. | 
**ds_block** | **i32** | Represents the Directory Service block which contains metadata about the miners who participate in the consensus protocol. | 
**ds_difficulty** | **String** | Defines how difficult it is to mine the dsBlocks. | 
**ds_leader** | **String** | Represents a part of the DS Committee which leads the consensus protocol for the epoch. | 
**gas_limit** | **i32** | Represents the maximum amount of gas allowed in the block in order to determine how many transactions it can fit. | 
**gas_used** | **i32** | Defines how much of the gas for the block has been used. | 
**micro_blocks** | **Vec<String>** |  | 
**next_block_hash** | **String** | Defines the hash of the next block from the specific blockchain. | 
**previous_block_hash** | **String** | Represents the hash of the previous block, also known as the parent block. | 
**timestamp** | **i32** | Defines the exact date/time when this block was mined in Unix Timestamp. | 
**transactions_count** | **i32** | Represents the total number of all transactions as part of this block. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


