# ListConfirmedTransactionsByAddressRibsz

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**binding_sig** | **String** | It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions. | 
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**join_split_pub_key** | **String** | Represents an encoding of a JoinSplitSig public validating key. | 
**join_split_sig** | **String** | Is used to sign transactions that contain at least one JoinSplit description. | 
**locktime** | **i64** | Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**size** | **i32** | Represents the total size of this transaction. | 
**v_join_split** | [**Vec<crate::models::ListConfirmedTransactionsByAddressRibszVJoinSplit>**](ListConfirmedTransactionsByAddressRIBSZ_vJoinSplit.md) | Represents a sequence of JoinSplit descriptions using BCTV14 proofs. | 
**v_shielded_output** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedOutput>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedOutput.md) | Object Array representation of transaction output descriptions | 
**v_shielded_spend** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedSpend>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedSpend.md) | Object Array representation of transaction spend descriptions | 
**value_balance** | **String** | Defines the transaction value balance. | 
**version** | **i32** | Defines the version of the transaction. | 
**version_group_id** | **String** | Represents the transaction version group ID. | 
**vin** | [**Vec<crate::models::ListConfirmedTransactionsByAddressRibszVin>**](ListConfirmedTransactionsByAddressRIBSZ_vin.md) | Object Array representation of transaction inputs | 
**vout** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVout>**](GetTransactionDetailsByTransactionIDRIBSZ_vout.md) | Object Array representation of transaction outputs | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


