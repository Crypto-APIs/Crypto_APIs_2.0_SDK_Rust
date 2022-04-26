# DecodeRawTransactionHexRise2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approximate_fee** | Option<**String**> | Defines the approximate fee value. When isConfirmed is True - Defines the amount of the transaction fee When isConfirmed is False - For ETH-based blockchains this attribute represents the max fee value. | [optional]
**approximate_minimum_required_fee** | Option<**String**> | Defines the approximate minimum fee that is required for the transaction. | [optional]
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_paid_for_data** | Option<**String**> | Represents the amount of gas paid for the data in the transaction. | [optional]
**gas_price** | Option<**String**> | Represents the price offered to the miner to purchase this amount of gas. | [optional]
**input_data** | Option<**String**> | Represents additional information that is required for the transaction. | [optional]
**nonce** | **i32** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**r** | Option<**String**> | Represents output of an ECDSA signature. | [optional]
**recipient** | **String** | The address which receives this transaction. In UTXO-based protocols like Bitcoin there could be several senders while in account-based protocols like Ethereum there is always only one recipient. | 
**s** | Option<**String**> | Represents output of an ECDSA signature. | [optional]
**sender** | **String** | Represents the address which sends this transaction. In UTXO-based protocols like Bitcoin there could be several senders while in account-based protocols like Ethereum there is always only one sender. | 
**_type** | **i32** | Specifies the transaction type as one from three options: if response returns a `\"0\"` it means the raw transaction includes legacy transaction data, if it is `\"1\"` - includes access lists for EIP2930, and if it is `\"2\"` - EIP1559 data. | 
**v** | Option<**String**> | Defines the the recovery id. | [optional]
**value** | Option<**String**> | Represents the sent/received amount. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


