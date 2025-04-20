# TestMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_request_time** | Option<**String**> | Timestamp of the first request | [optional]
**last_request_time** | Option<**String**> | Timestamp of the last request | [optional]
**request_distribution** | [**Vec<models::TimeBucket>**](TimeBucket.md) | Time-bucketed request counts (each entry is a 1-second window) | 
**requests_per_second** | **f64** | Calculated requests per second | 
**session_id** | **String** | ID of the test session | 
**total_requests** | **i32** | Total number of requests received | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


