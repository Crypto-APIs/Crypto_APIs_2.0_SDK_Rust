# AddressCoinsTransactionConfirmedDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | 
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"mordor\" are test networks. | 
**address** | **String** | Defines the specific address to which the coin transaction has been sent and is confirmed. | 
**mined_in_block** | [**crate::models::AddressCoinsTransactionConfirmedDataItemMinedInBlock**](AddressCoinsTransactionConfirmed_data_item_minedInBlock.md) |  | 
**transaction_id** | **String** | Defines the unique ID of the specific transaction, i.e. its identification number. | 
**amount** | **String** | Defines the amount of coins sent with the confirmed transaction. | 
**unit** | **String** | Defines the unit of the transaction, e.g. BTC. | 
**direction** | **String** | Defines whether the transaction is \"incoming\" or \"outgoing\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


