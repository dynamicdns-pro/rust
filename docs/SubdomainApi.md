# \SubdomainApi

All URIs are relative to *https://dynamicdns.pro/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update**](SubdomainApi.md#update) | **POST** /update/{subdomain}/record | 
[**updateip**](SubdomainApi.md#updateip) | **POST** /update/{subdomain} | update the ip address with the connecting ip address



## update

> models::Update200Response update(subdomain, update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subdomain** | **String** |  | [required] |
**update_request** | Option<[**UpdateRequest**](UpdateRequest.md)> |  |  |

### Return type

[**models::Update200Response**](update_200_response.md)

### Authorization

[http](../README.md#http)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

