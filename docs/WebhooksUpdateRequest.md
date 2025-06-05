# WebhooksUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | The webhook URL. Must be https | [optional]
**secret** | Option<**String**> | See: https://docs.doppler.com/docs/webhooks#verify-webhook-with-request-signing | [optional]
**authentication** | Option<[**models::WebhooksAddRequestAuthentication**](webhooks_add_request_authentication.md)> |  | [optional]
**payload** | Option<**String**> | See: https://docs.doppler.com/docs/webhooks#default-payload | [optional]
**name** | Option<**String**> | Name of the webhook. | [optional]
**enable_configs** | Option<**Vec<String>**> | Config slugs that the webhook should be enabled for | [optional]
**disable_configs** | Option<**Vec<String>**> | Config slugs that the webhook should be disabled for | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


