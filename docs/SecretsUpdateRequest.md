# SecretsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [default to CONFIG_NAME]
**secrets** | Option<[**models::SecretsUpdateRequestSecrets**](secrets_update_request_secrets.md)> |  | [optional]
**change_requests** | Option<[**Vec<models::SecretsUpdateRequestChangeRequestsInner>**](secrets_update_request_change_requests_inner.md)> | Either `secrets` or `change_requests` is required (can't use both). Object of secrets you would like to save to the config. Try it with the sample secrets below. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


