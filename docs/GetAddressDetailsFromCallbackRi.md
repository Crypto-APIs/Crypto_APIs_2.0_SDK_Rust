# GetAddressDetailsFromCallbackRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**incoming_transactions_count** | **i32** | Defines the received transaction count to the address. | 
**outgoing_transactions_count** | **i32** | Defines the sent transaction count from the address. | 
**transactions_count** | **i32** | Represents the total number of confirmed coins transactions for this address, both incoming and outgoing. Applies for coins only and not tokens transfers e.g. for Ethereum. transactionsCount could result as less than incoming and outgoing transactions put together (e.g. in Bitcoin), due to the fact that one and the same address could be in senders and receivers addresses. | 
**confirmed_balance** | [**crate::models::GetAddressDetailsFromCallbackRiConfirmedBalance**](GetAddressDetailsFromCallbackRI_confirmedBalance.md) |  | 
**total_received** | [**crate::models::GetAddressDetailsFromCallbackRiTotalReceived**](GetAddressDetailsFromCallbackRI_totalReceived.md) |  | 
**total_spent** | [**crate::models::GetAddressDetailsFromCallbackRiTotalSpent**](GetAddressDetailsFromCallbackRI_totalSpent.md) |  | 
**sequence** | Option<**i64**> | Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


