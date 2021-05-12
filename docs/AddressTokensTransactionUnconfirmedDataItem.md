# AddressTokensTransactionUnconfirmedDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | 
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | 
**address** | **String** | Defines the specific address to which the token transaction has been sent and is pending confirmation. | 
**transaction_id** | **String** | Defines the unique ID of the specific transaction, i.e. its identification number. | 
**token_type** | **String** | Defines the type of token sent with the transaction, e.g. ERC 20. | 
**token** | [**crate::models::AddressTokensTransactionUnconfirmedToken**](AddressTokensTransactionUnconfirmedToken.md) |  | 
**direction** | **String** | Defines whether the transaction is \"incoming\" or \"outgoing\". | 
**first_seen_in_mempool_timestamp** | **i32** | Defines the exact time the transaction has been first accepted into the mempool to await confirmation as timestamp. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


