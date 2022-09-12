# GetTransactionDetailsByTransactionIdFromCallbackRibs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i64** | Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid. | 
**size** | **i32** | Represents the total size of this transaction. | 
**v_size** | **i32** | Represents the virtual size of this transaction. | 
**version** | **i32** | Defines the version of the transaction. | 
**vin** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVin>**](GetTransactionDetailsByTransactionIDRIBSZ_vin.md) | Object Array representation of transaction inputs | 
**vout** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibszVout>**](GetTransactionDetailsByTransactionIDFromCallbackRIBSZ_vout.md) | Object Array representation of transaction outputs | 
**contract** | **String** | Represents the specific transaction contract. | 
**gas_limit** | **i32** | Represents the maximum amount of gas allowed in the block in order to determine how many transactions it can fit. | 
**gas_price** | **String** | Represents the price offered to the miner to purchase this amount of gas. | 
**gas_used** | **String** | Represents the exact unit of gas that was used for the transaction. | 
**input_data** | **String** | Represents additional information that is required for the transaction. | 
**nonce** | **i32** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**transaction_status** | **String** | Represents the status of this transaction. | 
**binding_sig** | **String** | It is used to enforce balance of Spend and Output transfers, in order to prevent their replay across transactions. | 
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**join_split_pub_key** | **String** | Represents an encoding of a JoinSplitSig public validating key. | 
**join_split_sig** | **String** | Is used to sign transactions that contain at least one JoinSplit description. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**v_join_split** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVJoinSplit>**](GetTransactionDetailsByTransactionIDRIBSZ_vJoinSplit.md) | Represents a sequence of JoinSplit descriptions using BCTV14 proofs. | 
**v_shielded_output** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedOutput>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedOutput.md) | Object Array representation of transaction output descriptions | 
**v_shielded_spend** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdribszVShieldedSpend>**](GetTransactionDetailsByTransactionIDRIBSZ_vShieldedSpend.md) | Object Array representation of transaction spend descriptions | 
**value_balance** | **String** | String representation of the transaction value balance | 
**version_group_id** | **String** | Represents the transaction version group ID | 
**additional_data** | **String** | Represents additional data that may be needed. | 
**destination_tag** | Option<**i64**> | Defines the destination tag value. | [optional]
**offer** | [**crate::models::GetXrpRippleTransactionDetailsByTransactionIdriOffer**](GetXRPRippleTransactionDetailsByTransactionIDRI_offer.md) |  | 
**receive** | [**crate::models::GetXrpRippleTransactionDetailsByTransactionIdriReceive**](GetXRPRippleTransactionDetailsByTransactionIDRI_receive.md) |  | 
**sequence** | **i64** | Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime. | 
**status** | **String** | Defines the status of the transaction. | 
**_type** | **String** | Defines the type of the transaction. | 
**value** | [**crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibsxValue**](GetTransactionDetailsByTransactionIDFromCallbackRIBSX_value.md) |  | 
**amount** | **String** | Representation of the amount value. | 
**bandwidth_used** | [**crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibstBandwidthUsed**](GetTransactionDetailsByTransactionIDFromCallbackRIBST_bandwidthUsed.md) |  | 
**energy_used** | [**crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibstEnergyUsed**](GetTransactionDetailsByTransactionIDFromCallbackRIBST_energyUsed.md) |  | 
**has_internal_transactions** | **bool** | Defines if the transaction includes internal transactions (true) or not (false). | 
**has_token_transfers** | **String** | Defines if the transaction includes token transfers (true) or not (false). | 
**input** | **String** | Represents additional information that is required for the transaction. | 
**recipients** | **String** | Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**senders** | **String** | Represents a list of sender addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**gas** | **String** | Represents the price offered to the miner to purchase this amount of gas. | 
**txid** | **String** | Represents the unique identifier of a transaction, i.e. it could be transactionId in UTXO-based protocols like Bitcoin, and transaction hash in Ethereum blockchain. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


