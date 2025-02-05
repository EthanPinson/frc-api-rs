# \TeamApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_team_v3_team_team_get**](TeamApi.md#read_team_v3_team_team_get) | **GET** /v3/team/{team} | Query a single team
[**read_teams_v3_teams_get**](TeamApi.md#read_teams_v3_teams_get) | **GET** /v3/teams | Query multiple teams



## read_team_v3_team_team_get

> serde_json::Value read_team_v3_team_team_get(team)
Query a single team

Returns a single Team object. Requires a team number (no prefix).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_teams_v3_teams_get

> Vec<serde_json::Value> read_teams_v3_teams_get(country, state, district, active, metric, ascending, limit, offset)
Query multiple teams

Returns up to 1000 teams at a time. Specify limit and offset to page through results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country** | Option<**String**> | Capitalized country name, e.g. `USA` or `Canada`. |  |
**state** | Option<**String**> | Capitalized two-letter state code, e.g. `NC`. |  |
**district** | Option<**String**> | One of [`fma`, `fnc`, `fit`, `fin`, `fim`, `ne`, `chs`, `ont`, `pnw`, `pch`, `isr`] |  |
**active** | Option<**bool**> | Whether the team has played in the last year. |  |
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

