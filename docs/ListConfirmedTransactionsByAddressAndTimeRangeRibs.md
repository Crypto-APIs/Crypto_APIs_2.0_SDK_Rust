# ListConfirmedTransactionsByAddressAndTimeRangeRibs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i64** | Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid. | 
**size** | **i32** | Represents the total size of this transaction. | 
**v_size** | **i32** | Represents the virtual size of this transaction. | 
**version** | **i32** | Defines the version of the transaction. | 
**vin** | [**Vec<crate::models::ListConfirmedTransactionsByAddressRibszVin>**](ListConfirmedTransactionsByAddressRIBSZ_vin.md) | Object Array representation of transaction inputs | 
**vout** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVout>**](GetTransactionDetailsByTransactionIDRIBSZ_vout.md) | Object Array representation of transaction outputs | 
**contract** | **String** | Represents the specific transaction contract. | 
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_price** | [**crate::models::ListConfirmedTransactionsByAddressRibsbscGasPrice**](ListConfirmedTransactionsByAddressRIBSBSC_gasPrice.md) |  | 
**gas_used** | **String** | Represents the exact unit of gas that was used for the transaction. | 
**input_data** | **String** | Represents additional information that is required for the transaction. | 
**nonce** | **i32** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**transaction_status** | **String** | String representation of the transaction status | 
**binding_sig** | **String** | It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions. | 
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**join_split_pub_key** | **String** | Represents an encoding of a JoinSplitSig public validating key. | 
**join_split_sig** | **String** | Is used to sign transactions that contain at least one JoinSplit description. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**v_join_split** | [**Vec<crate::models::ListConfirmedTransactionsByAddressRibszVJoinSplit>**](ListConfirmedTransactionsByAddressRIBSZ_vJoinSplit.md) | Represents a sequence of JoinSplit descriptions using BCTV14 proofs. | 
**v_shielded_output** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedOutput>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedOutput.md) | Object Array representation of transaction output descriptions | 
**v_shielded_spend** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedSpend>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedSpend.md) | Object Array representation of transaction spend descriptions | 
**value_balance** | **String** | Defines the transaction value balance. | 
**version_group_id** | **String** | Represents the transaction version group ID. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


