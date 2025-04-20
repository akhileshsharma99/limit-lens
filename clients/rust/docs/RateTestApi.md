# \RateTestApi

All URIs are relative to *http://localhost:6969*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_test_session**](RateTestApi.md#create_test_session) | **POST** /api/test/session | Create a new test session
[**get_all_sessions**](RateTestApi.md#get_all_sessions) | **GET** /api/test/sessions | Get all active test sessions
[**get_test_metrics**](RateTestApi.md#get_test_metrics) | **GET** /api/test/metrics/{session_id} | Get metrics for a test session
[**metrics_dashboard**](RateTestApi.md#metrics_dashboard) | **GET** /dashboard | Web interface for viewing live metrics
[**receive_test_request**](RateTestApi.md#receive_test_request) | **GET** /api/test/request/{session_id} | Receive a test request



## create_test_session

> models::TestSession create_test_session(create_session_request)
Create a new test session

Returns a unique session ID that can be used for rate limit testing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_session_request** | [**CreateSessionRequest**](CreateSessionRequest.md) |  | [required] |

### Return type

[**models::TestSession**](TestSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_sessions

> Vec<models::TestSession> get_all_sessions()
Get all active test sessions

Returns a list of all active test sessions

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TestSession>**](TestSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_test_metrics

> models::TestMetrics get_test_metrics(session_id)
Get metrics for a test session

Returns metrics and analysis of requests received for a session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The test session ID | [required] |

### Return type

[**models::TestMetrics**](TestMetrics.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metrics_dashboard

> metrics_dashboard()
Web interface for viewing live metrics

Serves an HTML page that displays live metrics for all active sessions

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## receive_test_request

> receive_test_request(session_id)
Receive a test request

Records a request for the specified session ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The test session ID | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

