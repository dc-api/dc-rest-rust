# SdkMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | Option<**String**> |  | [optional]
**embeds** | Option<[**Vec<models::RichEmbed>**](RichEmbed.md)> |  | [optional]
**allowed_mentions** | Option<[**models::MessageAllowedMentionsRequest**](MessageAllowedMentionsRequest.md)> |  | [optional]
**sticker_ids** | Option<**Vec<String>**> |  | [optional]
**components** | Option<[**Vec<models::BaseCreateMessageCreateRequestComponentsInner>**](BaseCreateMessageCreateRequest_components_inner.md)> |  | [optional]
**flags** | Option<**i32**> |  | [optional]
**attachments** | Option<[**Vec<models::MessageAttachmentRequest>**](MessageAttachmentRequest.md)> |  | [optional]
**poll** | Option<[**models::PollCreateRequest**](PollCreateRequest.md)> |  | [optional]
**confetti_potion** | Option<[**serde_json::Value**](.md)> |  | [optional]
**message_reference** | Option<[**models::MessageReferenceRequest**](MessageReferenceRequest.md)> |  | [optional]
**nonce** | Option<[**models::BasicMessageResponseNonce**](BasicMessageResponse_nonce.md)> |  | [optional]
**enforce_nonce** | Option<**bool**> |  | [optional]
**tts** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


