# \TeamYearApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_team_year_v3_team_year_team_year_get**](TeamYearApi.md#read_team_year_v3_team_year_team_year_get) | **GET** /v3/team_year/{team}/{year} | Query a single team year
[**read_team_years_v3_team_years_get**](TeamYearApi.md#read_team_years_v3_team_years_get) | **GET** /v3/team_years | Query multiple team years



## read_team_year_v3_team_year_team_year_get

> serde_json::Value read_team_year_v3_team_year_team_year_get(team, year)
Query a single team year

Returns a single Team Year object. Requires a team number and year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | **i32** |  | [required] |
**year** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_team_years_v3_team_years_get

> Vec<serde_json::Value> read_team_years_v3_team_years_get(team, year, country, state, district, metric, ascending, limit, offset)
Query multiple team years

Returns up to 1000 team years at a time. Specify limit and offset to page through results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team** | Option<**i32**> | Team number (no prefix), e.g. 5511. |  |
**year** | Option<**i32**> | Four-digit year |  |
**country** | Option<**String**> | Capitalized country name, e.g. `USA` or `Canada`. |  |
**state** | Option<**String**> | Capitalized two-letter state code, e.g. `NC`. |  |
**district** | Option<**String**> | One of [`fma`, `fnc`, `fit`, `fin`, `fim`, `ne`, `chs`, `ont`, `pnw`, `pch`, `isr`] |  |
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

