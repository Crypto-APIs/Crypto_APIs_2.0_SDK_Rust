# ListLatestMinedBlocksRibsbc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bits** | **String** | A sub-unit of BCH equal to 0.000001 BCH, or 100 Satoshi, and is the same as microbitcoincash (μBCH). Bits have two-decimal precision. | 
**chainwork** | **String** | Represents a hexadecimal number of all the hashes necessary to produce the current chain. E.g., when converting 0000000000000000000000000000000000000000000086859f7a841475b236fd to a decimal you get 635262017308958427068157 hashes, or 635262 exahashes. | 
**difficulty** | **String** | Represents a mathematical value of how hard it is to find a valid hash for this block. | 
**merkle_root** | **String** | Defines the single and final (root) node of a Merkle tree. It is the combined hash of all transactions' hashes that are part of a blockchain block. | 
**nonce** | **i32** | Represents a random value that can be adjusted to satisfy the proof of work | 
**size** | **i32** | Represents a random value that can be adjusted to satisfy the proof of work | 
**version** | **i32** | Represents the version of the specific block on the blockchain. | 
**version_hex** | **String** | Is the hexadecimal string representation of the block's version. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


