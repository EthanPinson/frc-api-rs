# \TeamEventApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_team_event_v3_team_event_team_event_get**](TeamEventApi.md#read_team_event_v3_team_event_team_event_get) | **GET** /v3/team_event/{team}/{event} | Query a single team event
[**read_team_events_v3_team_events_get**](TeamEventApi.md#read_team_events_v3_team_events_get) | **GET** /v3/team_events | Query multiple team events



## read_team_event_v3_team_event_team_event_get

> serde_json::Value read_team_event_v3_team_event_team_event_get(team, event)
Query a single team event

Returns a single Team Event object. Requires a team number and event key, e.g. `5511` and `2019ncwak`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | **i32** |  | [required] |
**event** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_team_events_v3_team_events_get

> Vec<serde_json::Value> read_team_events_v3_team_events_get(team, year, event, country, state, district, r#type, week, metric, ascending, limit, offset)
Query multiple team events

Returns up to 1000 team events at a time. Specify limit and offset to page through results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | Option<**i32**> | Team number (no prefix), e.g. 5511. |  |
**year** | Option<**i32**> | Four-digit year |  |
**event** | Option<**String**> | Event key, e.g. `2019ncwak`. |  |
**country** | Option<**String**> | Capitalized country name, e.g. `USA` or `Canada`. |  |
**state** | Option<**String**> | Capitalized two-letter state code, e.g. `NC`. |  |
**district** | Option<**String**> | One of [`fma`, `fnc`, `fit`, `fin`, `fim`, `ne`, `chs`, `ont`, `pnw`, `pch`, `isr`] |  |
**r#type** | Option<**String**> | One of [`regional`, `district`, `district_cmp`, `cmp_division`, or `cmp_finals`]. |  |
**week** | Option<**i32**> | Week of the competition season. 8 is CMP |  |
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

