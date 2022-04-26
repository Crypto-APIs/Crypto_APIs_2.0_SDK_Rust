# DecodeRawTransactionHexRis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i32** | Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid. | 
**transaction_hash** | **String** | Represents the same as transactionId for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols hash is different from transactionId for SegWit transactions. | 
**v_size** | **i32** | Represents the virtual size of this transaction. | 
**version** | **i32** | Represents the transaction version number. | 
**vin** | [**Vec<crate::models::DecodeRawTransactionHexRiszVin>**](DecodeRawTransactionHexRISZ_vin.md) | Represents the Inputs of the transaction | 
**vout** | [**Vec<crate::models::DecodeRawTransactionHexRiszVout>**](DecodeRawTransactionHexRISZ_vout.md) | Represents the Inputs of the transaction | 
**weight** | Option<**i32**> | Represents the size of a block, measured in weight units and including the segwit discount. | [optional]
**approximate_fee** | Option<**String**> | Defines the approximate fee value. When isConfirmed is True - Defines the amount of the transaction fee When isConfirmed is False - For ETH-based blockchains this attribute represents the max fee value. | [optional]
**approximate_minimum_required_fee** | Option<**String**> | Defines the approximate minimum fee that is required for the transaction. | [optional]
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_paid_for_data** | Option<**String**> | Represents the amount of gas paid for the data in the transaction. | [optional]
**gas_price** | Option<**String**> | Represents the price offered to the miner to purchase this amount of gas. | [optional]
**input_data** | Option<**String**> | Represents additional information that is required for the transaction. | [optional]
**max_fee_per_gas** | Option<**String**> | Defines the maximum amount that customer is willing to pay per unit of gas to get his transaction included in a block. | [optional]
**max_fee_priority_per_gas** | Option<**String**> | Represents determined by the user value that is paid directly to miners. | [optional]
**nonce** | **i32** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**r** | Option<**String**> | Represents output of an ECDSA signature. | [optional]
**recipient** | **String** | The address which receives this transaction. In UTXO-based protocols like Bitcoin there could be several senders while in account-based protocols like Ethereum there is always only one recipient. | 
**s** | Option<**String**> | Represents output of an ECDSA signature. | [optional]
**sender** | **String** | Represents the address which sends this transaction. In UTXO-based protocols like Bitcoin there could be several senders while in account-based protocols like Ethereum there is always only one sender. | 
**_type** | **i32** | Specifies the transaction type as one from three options: if response returns a `\"0\"` it means the raw transaction includes legacy transaction data, if it is `\"1\"` - includes access lists for EIP2930, and if it is `\"2\"` - EIP1559 data. | 
**v** | Option<**String**> | Defines the the recovery id. | [optional]
**value** | Option<**String**> | Represents the sent/received amount. | [optional]
**expiry_height** | **i32** | Represents a block height after which the transaction will expire. | 
**overwintered** | **bool** | \"Overwinter\" is the network upgrade for the Zcash blockchain. | 
**saplinged** | **bool** | Defines if the transaction includes sapling or not. | 
**value_balance** | **String** | Defines the transaction value balance. | 
**version_group_id** | **String** | Represents the transaction version group ID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


