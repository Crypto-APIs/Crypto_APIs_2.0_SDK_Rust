# ListLatestMinedBlocksRibs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bits** | **String** | Represents a specific sub-unit of Zcash. Bits have two-decimal precision | 
**chainwork** | **String** | Represents a hexadecimal number of all the hashes necessary to produce the current chain. E.g., when converting 0000000000000000000000000000000000000000000086859f7a841475b236fd to a decimal you get 635262017308958427068157 hashes, or 635262 exahashes. | 
**difficulty** | **String** | Represents a mathematical value of how hard it is to find a valid hash for this block. | 
**merkle_root** | **String** | Defines the single and final (root) node of a Merkle tree. It is the combined hash of all transactions' hashes that are part of a blockchain block. | 
**nonce** | **String** | Represents a random value that can be adjusted to satisfy the proof of work | 
**size** | **i32** | Represents the total size of the block in Bytes. | 
**stripped_size** | **i32** | Defines the numeric representation of the block size excluding the witness data. | 
**version** | **i32** | Represents the transaction version number. | 
**version_hex** | **String** | Is the hexadecimal string representation of the block's version. | 
**weight** | **i32** | Represents a measurement to compare the size of different transactions to each other in proportion to the block size limit. | 
**extra_data** | **String** | Represents any data that can be included by the miner in the block. | 
**gas_limit** | **i32** | Represents the maximum amount of gas allowed in the block in order to determine how many transactions it can fit. | 
**gas_used** | **i32** | Defines how much of the gas for the block has been used. | 
**mined_in_seconds** | **i32** | Specifies the amount of time required for the block to be mined in second | 
**sha3_uncles** | **String** | Defines the combined hash of all uncles for a given parent. | 
**total_difficulty** | **String** | Defines the total difficulty of the chain until this block, i.e. how difficult it is for a specific miner to mine a new block | 
**uncles** | **Vec<String>** |  | 
**ds_block** | **i32** | Represents the Directory Service block which contains metadata about the miners who participate in the consensus protocol. | 
**ds_difficulty** | **String** | Defines how difficult it is to mine the dsBlocks. | 
**ds_leader** | **String** | Represents a part of the DS Committee which leads the consensus protocol for the epoch. | 
**micro_blocks** | **Vec<String>** |  | 
**total_coins** | Option<[**crate::models::ListLatestMinedBlocksRibsxTotalCoins**](ListLatestMinedBlocksRIBSX_totalCoins.md)> |  | [optional]
**total_fees** | [**crate::models::ListLatestMinedBlocksRibsxTotalFees**](ListLatestMinedBlocksRIBSX_totalFees.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


