# \TeamMatchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_team_match_v3_team_match_team_match_get**](TeamMatchApi.md#read_team_match_v3_team_match_team_match_get) | **GET** /v3/team_match/{team}/{match} | Query a single team match
[**read_team_matches_v3_team_matches_get**](TeamMatchApi.md#read_team_matches_v3_team_matches_get) | **GET** /v3/team_matches | Query multiple team matches



## read_team_match_v3_team_match_team_match_get

> serde_json::Value read_team_match_v3_team_match_team_match_get(team, r#match)
Query a single team match

Returns a single Team Match object. Requires a team number and match key, e.g. `5511` and `2019ncwak_f1m1`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | **i32** |  | [required] |
**r#match** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_team_matches_v3_team_matches_get

> Vec<serde_json::Value> read_team_matches_v3_team_matches_get(team, year, event, week, r#match, elim, metric, ascending, limit, offset)
Query multiple team matches

Returns up to 1000 team matches at a time. Specify limit and offset to page through results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | Option<**i32**> | Team number (no prefix), e.g. 5511. |  |
**year** | Option<**i32**> | Four-digit year |  |
**event** | Option<**String**> | Event key, e.g. `2019ncwak`. |  |
**week** | Option<**i32**> | Week of the competition season. 8 is CMP |  |
**r#match** | Option<**String**> | Match key, e.g. `2019ncwak_f1m1`. |  |
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

