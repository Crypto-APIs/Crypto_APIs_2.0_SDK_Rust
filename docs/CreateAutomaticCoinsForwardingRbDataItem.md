# CreateAutomaticCoinsForwardingRbDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_secret_key** | **String** | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. | 
**callback_url** | **String** | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. | 
**confirmations_count** | **i32** | Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block. | 
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\". | 
**minimum_transfer_amount** | **String** | Represents the minimum transfer amount of the currency in the `fromAddress` that can be allowed for an automatic forwarding. | 
**to_address** | **String** | Represents the hash of the address the currency is forwarded to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


