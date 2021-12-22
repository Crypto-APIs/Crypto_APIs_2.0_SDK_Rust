# GetWalletTransactionDetailsByTransactionIdribs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i32** | Represents the time at which a particular transaction can be added to the blockchain. | 
**size** | **i32** | Represents the total size of this transaction. | 
**v_size** | **i32** | Represents the virtual size of this transaction. | 
**version** | **i32** | Represents the transaction version number. | 
**vin** | [**Vec<crate::models::GetWalletTransactionDetailsByTransactionIdribszVin>**](GetWalletTransactionDetailsByTransactionIDRIBSZ_vin.md) | Object Array representation of transaction inputs | 
**vout** | [**Vec<crate::models::ListTransactionsByBlockHeightRibszVout>**](ListTransactionsByBlockHeightRIBSZ_vout.md) | Object Array representation of transaction outputs | 
**contract** | **String** | Represents the specific transaction contract | 
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_price** | [**crate::models::GetTransactionDetailsByTransactionIdribsbscGasPrice**](GetTransactionDetailsByTransactionIDRIBSBSC_gasPrice.md) |  | 
**gas_used** | **String** | Defines the unit of the gas price amount, e.g. BTC, ETH, XRP. | 
**input_data** | **String** | Represents additional information that is required for the transaction. | 
**nonce** | **i32** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**transaction_status** | **String** | String representation of the transaction status | 
**binding_sig** | **String** | It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions. | 
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**join_split_pub_key** | **String** | Represents an encoding of a JoinSplitSig public validating key. | 
**join_split_sig** | **String** | Is used to sign transactions that contain at least one JoinSplit description. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**v_join_split** | Option<[**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVJoinSplit>**](GetTransactionDetailsByTransactionIDRIBSZ_vJoinSplit.md)> | Represents a sequence of JoinSplit descriptions using BCTV14 proofs. | [optional]
**v_shielded_output** | Option<[**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedOutput>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedOutput.md)> | Object Array representation of transaction output descriptions | [optional]
**v_shielded_spend** | Option<[**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedSpend>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedSpend.md)> | Object Array representation of transaction spend descriptions | [optional]
**value_balance** | **String** | String representation of the transaction value balance | 
**version_group_id** | **String** | Represents the transaction version group ID. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


