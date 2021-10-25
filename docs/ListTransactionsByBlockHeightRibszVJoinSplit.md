# ListTransactionsByBlockHeightRibszVJoinSplit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anchor** | **String** | Defines a Merkle tree root of a note commitment tree which uniquely identifies a note commitment tree state given the assumed security properties of the Merkle tree’s  hash function. | 
**cipher_texts** | **Vec<String>** |  | 
**commitments** | **Vec<String>** |  | 
**macs** | **Vec<String>** |  | 
**nullifiers** | **Vec<String>** |  | 
**one_time_pub_key** | **String** | Defines the one time public key. | 
**proof** | **String** | Defines the proof. | 
**random_seed** | **String** | Represents a 256-bit seed that must be chosen independently at random for each JoinSplit description. | 
**v_pub_new** | **String** | Defines the value that the joinSplit transfer will insert into the transparent transaction value pool. | 
**v_pub_old** | **String** | Defines the value that the joinSplit transfer will remove from the transparent transaction value pool. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


