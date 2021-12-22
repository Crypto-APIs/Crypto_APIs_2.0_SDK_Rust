# AddressInternalTransactionConfirmedEachConfirmationDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | 
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | 
**address** | **String** | Defines the specific address of the internal transaction. | 
**mined_in_block** | [**crate::models::AddressInternalTransactionConfirmedEachConfirmationDataItemMinedInBlock**](AddressInternalTransactionConfirmedEachConfirmation_data_item_minedInBlock.md) |  | 
**parent_transaction_id** | **String** | Defines the Parent Transaction's unique ID. | 
**operation_id** | **String** | Defines the specific operation's unique ID. | 
**current_confirmations** | **i32** | Defines the number of currently received confirmations for the transaction. | 
**target_confirmations** | **i32** | Defines the number of confirmation transactions requested as callbacks, i.e. the system can notify till the n-th confirmation. | 
**amount** | **String** | Defines the amount of coins sent with the confirmed transaction. | 
**unit** | **String** | Defines the unit of the transaction, e.g. Gwei. | 
**direction** | **String** | Defines whether the transaction is \"incoming\" or \"outgoing\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


