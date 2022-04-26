# GetAddressDetailsFromCallbackRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**incoming_transactions_count** | **i32** | Defines the count of the incoming transactions. | 
**outgoing_transactions_count** | **i32** | Defines the count of the outgoing transactions. | 
**transactions_count** | **i32** | Represents the total number of confirmed coins transactions for this address, both incoming and outgoing. Applies for coins only **and not** tokens transfers e.g. for Ethereum. `transactionsCount` could result as less than incoming and outgoing transactions put together (e.g. in Bitcoin), due to the fact that one and the same address could be in senders and receivers addresses. | 
**confirmed_balance** | [**crate::models::GetAddressDetailsRiConfirmedBalance**](GetAddressDetailsRI_confirmedBalance.md) |  | 
**total_received** | Option<[**crate::models::GetAddressDetailsFromCallbackRiTotalReceived**](GetAddressDetailsFromCallbackRI_totalReceived.md)> |  | [optional]
**total_spent** | Option<[**crate::models::GetAddressDetailsFromCallbackRiTotalSpent**](GetAddressDetailsFromCallbackRI_totalSpent.md)> |  | [optional]
**sequence** | Option<**i64**> | Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


