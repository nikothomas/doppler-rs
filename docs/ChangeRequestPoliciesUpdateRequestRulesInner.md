# ChangeRequestPoliciesUpdateRequestRulesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**count** | Option<**i32**> | The number of required reviewers. Only applies to \"RequiredReviewer\" rules. | [optional][default to 1]
**subjects** | Option<[**Vec<models::ChangeRequestPoliciesUpdateRequestRulesInnerSubjectsInner>**](change_request_policies_update_request_rules_inner_subjects_inner.md)> | A list of required reviewers. If specified, only reviews from reviewers in this list will satisfy the policy. Only applies to \"RequiredReviewer\" rules. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


