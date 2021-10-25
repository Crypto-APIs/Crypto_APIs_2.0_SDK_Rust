# TokensForwardingSuccessDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | 
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | 
**from_address** | **String** | Represents the hash of the address that provides the tokens. | 
**to_address** | **String** | Represents the hash of the address to forward the tokens to. | 
**spent_fees_amount** | **String** | Represents the amount of the fee spent for the tokens to be forwarded. | 
**spent_fees_unit** | **String** | Represents the unit of the fee spent for the tokens to be forwarded, e.g. BTC. | 
**trigger_transaction_id** | **String** | Defines the unique Transaction ID that triggered the token forwarding. | 
**forwarding_transaction_id** | **String** | Defines the unique Transaction ID that forwarded the tokens. | 
**token_type** | **String** | Defines the type of token sent with the transaction, e.g. ERC 20. | 
**token** | [**crate::models::TokensForwardingSuccessToken**](TokensForwardingSuccessToken.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


