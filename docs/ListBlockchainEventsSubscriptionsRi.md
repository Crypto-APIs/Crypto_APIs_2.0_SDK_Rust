# ListBlockchainEventsSubscriptionsRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | Represents the address of the transaction. | 
**callback_secret_key** | Option<**String**> | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security). | [optional]
**callback_url** | **String** | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`. | 
**confirmations_count** | **i32** | Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block. | 
**created_timestamp** | **i32** | Defines the specific time/date when the subscription was created in Unix Timestamp. | 
**deactivation_reasons** | Option<[**Vec<crate::models::ListBlockchainEventsSubscriptionsRiDeactivationReasons>**](ListBlockchainEventsSubscriptionsRI_deactivationReasons.md)> | Represents the deactivation reason details, available when a blockchain event subscription has status isActive - false. | [optional]
**event_type** | **String** | Defines the type of the specific event available for the customer to subscribe to for callback notification. | 
**is_active** | **bool** | Defines whether the subscription is active or not. Set as boolean. | 
**reference_id** | **String** | Represents a unique ID used to reference the specific callback subscription. | 
**transaction_id** | Option<**String**> | Represents the unique identification string that defines the transaction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


