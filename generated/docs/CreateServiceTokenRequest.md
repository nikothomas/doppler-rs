# CreateServiceTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [default to CONFIG_NAME]
**name** | **String** | Name of the service token. | [default to TOKEN_NAME]
**expire_at** | Option<[**chrono::DateTime<chrono::Utc>**](chrono::DateTime<chrono::Utc>.md)> | Unix timestamp of when token should expire. | [optional]
**access** | Option<**String**> | Token's capabilities. | [optional][default to Read]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


