# GetWalletTransactionDetailsByTransactionIdribsz

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**binding_sig** | **String** | It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions. | 
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**join_split_pub_key** | **String** | Represents an encoding of a JoinSplitSig public validating key. | 
**join_split_sig** | **String** | Is used to sign transactions that contain at least one JoinSplit description. | 
**locktime** | **i32** | Represents the time at which a particular transaction can be added to the blockchain. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**size** | **i32** | Represents the total size of this transaction. | 
**v_join_split** | Option<[**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVJoinSplit>**](GetTransactionDetailsByTransactionIDRIBSZ_vJoinSplit.md)> | Represents a sequence of JoinSplit descriptions using BCTV14 proofs. | [optional]
**v_shielded_output** | Option<[**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedOutput>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedOutput.md)> | Object Array representation of transaction output descriptions | [optional]
**v_shielded_spend** | Option<[**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedSpend>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedSpend.md)> | Object Array representation of transaction spend descriptions | [optional]
**value_balance** | **String** | String representation of the transaction value balance | 
**version** | **i32** | Represents the transaction version number. | 
**version_group_id** | **String** | Represents the transaction version group ID. | 
**vin** | [**Vec<crate::models::GetWalletTransactionDetailsByTransactionIdribszVin>**](GetWalletTransactionDetailsByTransactionIDRIBSZ_vin.md) | Object Array representation of transaction inputs | 
**vout** | [**Vec<crate::models::ListTransactionsByBlockHeightRibszVout>**](ListTransactionsByBlockHeightRIBSZ_vout.md) | Object Array representation of transaction outputs | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


