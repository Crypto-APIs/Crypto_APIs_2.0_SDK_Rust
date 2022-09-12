# CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_secret_key** | Option<**String**> | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security). | [optional]
**callback_url** | Option<**String**> | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`. | [optional]
**note** | Option<**String**> | Represents an optional note to add a free text in, explaining or providing additional detail on the transaction request. | [optional]
**recipient** | [**Vec<crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRiRecipient>**](CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRI_recipient.md) | Defines the destination for the transaction, i.e. the recipient(s). | 
**sender** | [**crate::models::CreateSingleTransactionRequestFromAddressWithoutFeePriorityRiSender**](CreateSingleTransactionRequestFromAddressWithoutFeePriorityRI_sender.md) |  | 
**token_type_specific_data** | [**crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRis**](CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRIS.md) |  | 
**transaction_request_id** | **String** | Represents a unique identifier of the transaction request (the request sent to make a transaction), which helps in identifying which callback and which `referenceId` concern that specific transaction request. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


