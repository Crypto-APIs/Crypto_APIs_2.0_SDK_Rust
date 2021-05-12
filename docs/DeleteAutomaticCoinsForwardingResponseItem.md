# DeleteAutomaticCoinsForwardingResponseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_url** | **String** | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. | 
**confirmations_count** | **String** | Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block. | 
**created_timestamp** | **String** | Defines the specific time/date when the automatic forwarding was created in Unix Timestamp. | 
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"SLOW\", \"STANDARD\" or \"FAST\". | 
**from_address** | **String** | Represents the hash of the address that forwards the currency. | 
**minimum_transfer_amount** | **String** | Represents the minimum transfer amount of the currency in the `fromAddress` that can be allowed for an automatic forwarding. | 
**reference_id** | **String** | Represents a unique ID used to reference the specific callback subscription. | 
**to_address** | **String** | Represents the hash of the address the currency is forwarded to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


