# SecretChangeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the secret. | 
**original_name** | **String** | The original name of the secret. Use `null` (an actual `null`, not the string `null`) or omit this parameter for new secrets. If it differs from `name` then a rename is inferred. | 
**value** | **String** | The value the secret should have. Use `null` (an actual `null`, not the string `null`) to leave the existing secret value unchanged. | 
**original_value** | Option<**String**> | The value you expect the secret to have before `name` is applied. If specified, the request will only be processed if the provided value matches what's found in Doppler. | [optional]
**visibility** | Option<**String**> | Must be set to either `masked`, `unmasked`, or `restricted`. | [optional]
**original_visibility** | Option<**String**> | Must be set to either `masked`, `unmasked`, or `restricted`. The visibility you expect the secret to have before `visibility` is applied. If specified, the request will only be processed if the provided visibility matches what's found in Doppler. | [optional]
**should_promote** | Option<**bool**> | Defaults to `false`. Can only be set to `true` if the config being updated is a branch config. If set to `true`, the provided secret will be set in both the branch config as well as the root config in that environment. | [optional]
**should_delete** | Option<**bool**> | Defaults to `false`. If set to `true`, will delete the secret matching the `name` field. | [optional]
**should_converge** | Option<**bool**> | Defaults to `false`. Can only be set to `true` if the config being updated is a branch config and there is a secret with the same name in the root config. In this case, the branch secret will inherit the value and visibility type from the root secret. | [optional]
**value_type** | Option<[**models::SecretValueType**](SecretValueType.md)> |  | [optional]
**original_value_type** | Option<[**models::OriginalSecretValueType**](OriginalSecretValueType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


