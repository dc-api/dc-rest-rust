# KeywordRuleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**guild_id** | **String** |  | 
**creator_id** | **String** |  | 
**name** | **String** |  | 
**event_type** | **i32** |  | 
**actions** | [**Vec<models::DefaultKeywordRuleResponseActionsInner>**](DefaultKeywordRuleResponse_actions_inner.md) |  | 
**trigger_type** | **i32** |  | 
**enabled** | Option<**bool**> |  | [optional]
**exempt_roles** | Option<**Vec<String>**> |  | [optional]
**exempt_channels** | Option<**Vec<String>**> |  | [optional]
**trigger_metadata** | [**models::KeywordTriggerMetadataResponse**](KeywordTriggerMetadataResponse.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


