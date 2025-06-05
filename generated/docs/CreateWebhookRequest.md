# CreateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The webhook URL. Must be https | 
**secret** | Option<**String**> | See: https://docs.doppler.com/docs/webhooks#verify-webhook-with-request-signing | [optional]
**authentication** | Option<[**models::WebhookAuthentication**](WebhookAuthentication.md)> |  | [optional]
**payload** | Option<**String**> | See: https://docs.doppler.com/docs/webhooks#default-payload | [optional]
**enable_configs** | Option<**Vec<String>**> | Config slugs that the webhook should be enabled for | [optional]
**name** | Option<**String**> | The name of the webhook. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


