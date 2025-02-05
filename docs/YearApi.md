# \YearApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_year_v3_year_year_get**](YearApi.md#read_year_v3_year_year_get) | **GET** /v3/year/{year} | Query a single year
[**read_years_v3_years_get**](YearApi.md#read_years_v3_years_get) | **GET** /v3/years | Query multiple years



## read_year_v3_year_year_get

> serde_json::Value read_year_v3_year_year_get(year)
Query a single year

Returns a single Year object. Requires a four-digit year, e.g. `2019`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_years_v3_years_get

> Vec<serde_json::Value> read_years_v3_years_get(metric, ascending, limit, offset)
Query multiple years

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | Option<**String**> | How to sort the returned values. Any column in the table is valid. |  |
**ascending** | Option<**bool**> | Whether to sort the returned values in ascending order. Default is ascending |  |
**limit** | Option<**i32**> | Maximum number of events to return. Default is 1000. |  |
**offset** | Option<**i32**> | Offset from the first result to return. |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

