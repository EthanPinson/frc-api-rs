# \MatchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_match_v3_match_match_get**](MatchApi.md#read_match_v3_match_match_get) | **GET** /v3/match/{match} | Query a single match
[**read_matches_v3_matches_get**](MatchApi.md#read_matches_v3_matches_get) | **GET** /v3/matches | Query multiple matches



## read_match_v3_match_match_get

> serde_json::Value read_match_v3_match_match_get(r#match)
Query a single match

Returns a single Match object. Requires a match key, e.g. `2019ncwak_f1m1`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#match** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_matches_v3_matches_get

> Vec<serde_json::Value> read_matches_v3_matches_get(team, year, event, week, elim, metric, ascending, limit, offset)
Query multiple matches

Returns up to 1000 matches at a time. Specify limit and offset to page through results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | Option<**i32**> | Team number (no prefix), e.g. 5511. |  |
**year** | Option<**i32**> | Four-digit year |  |
**event** | Option<**String**> | Event key, e.g. `2019ncwak`. |  |
**week** | Option<**i32**> | Week of the competition season. 8 is CMP |  |
**elim** | Option<**bool**> | Whether the match is an elimination match. |  |
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

