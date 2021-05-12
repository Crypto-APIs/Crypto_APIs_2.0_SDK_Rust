# AddressTokensTransactionConfirmedEachConfirmationDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | 
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | 
**address** | **String** | Defines the specific address to which the transaction has been sent. | 
**mined_in_block** | [**crate::models::AddressTokensTransactionConfirmedEachConfirmationDataItemMinedInBlock**](AddressTokensTransactionConfirmedEachConfirmation_data_item_minedInBlock.md) |  | 
**transaction_id** | **String** | Defines the unique ID of the specific transaction, i.e. its identification number. | 
**current_confirmations** | **i32** | Defines the number of currently received confirmations for the transaction. | 
**target_confirmations** | **i32** | Defines the number of confirmation transactions requested as callbacks, i.e. the system can notify till the n-th confirmation. | 
**token_type** | **String** | Defines the type of token sent with the transaction, e.g. ERC 20. | 
**token** | [**crate::models::AddressTokensTransactionConfirmedEachConfirmationToken**](AddressTokensTransactionConfirmedEachConfirmationToken.md) |  | 
**direction** | **String** | Defines whether the transaction is \"incoming\" or \"outgoing\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


