# ListUnspentTransactionOutputsByAddressRiBlockchainSpecific

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**v_size** | **i32** | Represents the virtual size of this transaction | 
**binding_sig** | **String** | It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions. | 
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**join_split_pub_key** | **String** | Represents an encoding of a JoinSplitSig public validating key. | 
**join_split_sig** | **String** | Is used to sign transactions that contain at least one JoinSplit description. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**v_join_split** | Option<[**Vec<crate::models::ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVJoinSplit>**](ListUnspentTransactionOutputsByAddressRI_blockchainSpecific_vJoinSplit.md)> | Represents a sequence of JoinSplit descriptions using BCTV14 proofs. | [optional]
**v_shielded_output** | Option<[**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedOutput>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedOutput.md)> | Object Array representation of transaction output descriptions | [optional]
**v_shielded_spend** | [**Vec<crate::models::ListUnspentTransactionOutputsByAddressRiBlockchainSpecificVShieldedSpend>**](ListUnspentTransactionOutputsByAddressRI_blockchainSpecific_vShieldedSpend.md) | Object Array representation of transaction spend descriptions | 
**value_balance** | **String** | Defines the transaction value balance. | 
**version_group_id** | **String** | Represents the transaction version group ID. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


