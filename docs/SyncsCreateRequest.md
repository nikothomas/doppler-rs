# SyncsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**integration** | **String** | The integration slug which the sync will use | 
**data** | [**serde_json::Value**](.md) | Configuration data for the sync | 
**import_option** | Option<**String**> | An option indicating if and how Doppler should attempt to import secrets from the sync destination | [optional][default to None]
**await_initial_sync** | Option<**bool**> | Causes sync creation to wait for the initial sync to complete before returning. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


