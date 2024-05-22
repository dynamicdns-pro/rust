# \SubdomainApi

All URIs are relative to *http://dynamicdns.pro/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**updateip**](SubdomainApi.md#updateip) | **POST** /update/{subdomain} | update the ip address with the connecting ip address



## updateip

> models::Updateip200Response updateip(subdomain, body)
update the ip address with the connecting ip address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subdomain** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::Updateip200Response**](updateip_200_response.md)

### Authorization

[http](../README.md#http)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

