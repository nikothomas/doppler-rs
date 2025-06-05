# \DefaultApi

All URIs are relative to *https://api.doppler.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activity_logs_list**](DefaultApi.md#activity_logs_list) | **GET** /v3/logs | List
[**activity_logs_retrieve**](DefaultApi.md#activity_logs_retrieve) | **GET** /v3/logs/log | Retrieve
[**audit_get_user**](DefaultApi.md#audit_get_user) | **GET** /v3/workplace/users/{workplace_user_id} | Workplace User
[**auth_me**](DefaultApi.md#auth_me) | **GET** /v3/me | Me
[**auth_oidc**](DefaultApi.md#auth_oidc) | **POST** /v3/auth/oidc | OIDC (Service Account Identity)
[**auth_revoke**](DefaultApi.md#auth_revoke) | **POST** /v3/auth/revoke | Revoke
[**change_request_policies_create**](DefaultApi.md#change_request_policies_create) | **POST** /v3/workplace/change_request_policies | Create
[**change_request_policies_delete**](DefaultApi.md#change_request_policies_delete) | **DELETE** /v3/workplace/change_request_policies/change_request_policy/{slug} | Delete
[**change_request_policies_get**](DefaultApi.md#change_request_policies_get) | **GET** /v3/workplace/change_request_policies/change_request_policy/{slug} | Retrieve
[**change_request_policies_list**](DefaultApi.md#change_request_policies_list) | **GET** /v3/workplace/change_request_policies | List
[**change_request_policies_update**](DefaultApi.md#change_request_policies_update) | **POST** /v3/workplace/change_request_policies/change_request_policy/{slug} | Update
[**config_logs_get**](DefaultApi.md#config_logs_get) | **GET** /v3/configs/config/logs/log | Retrieve
[**config_logs_list**](DefaultApi.md#config_logs_list) | **GET** /v3/configs/config/logs | List
[**config_logs_rollback**](DefaultApi.md#config_logs_rollback) | **POST** /v3/configs/config/logs/log/rollback | Rollback
[**configs_add_trusted_ip**](DefaultApi.md#configs_add_trusted_ip) | **POST** /v3/configs/config/trusted_ips | Add
[**configs_clone**](DefaultApi.md#configs_clone) | **POST** /v3/configs/config/clone | Clone
[**configs_create**](DefaultApi.md#configs_create) | **POST** /v3/configs | Create
[**configs_delete**](DefaultApi.md#configs_delete) | **DELETE** /v3/configs/config | Delete
[**configs_delete_trusted_ip**](DefaultApi.md#configs_delete_trusted_ip) | **DELETE** /v3/configs/config/trusted_ips | Delete
[**configs_get**](DefaultApi.md#configs_get) | **GET** /v3/configs/config | Retrieve
[**configs_inheritable**](DefaultApi.md#configs_inheritable) | **POST** /v3/configs/config/inheritable | Inheritable
[**configs_inherits**](DefaultApi.md#configs_inherits) | **POST** /v3/configs/config/inherits | Inherits
[**configs_list**](DefaultApi.md#configs_list) | **GET** /v3/configs | List
[**configs_list_trusted_ips**](DefaultApi.md#configs_list_trusted_ips) | **GET** /v3/configs/config/trusted_ips | List
[**configs_lock**](DefaultApi.md#configs_lock) | **POST** /v3/configs/config/lock | Lock
[**configs_unlock**](DefaultApi.md#configs_unlock) | **POST** /v3/configs/config/unlock | Unlock
[**configs_update**](DefaultApi.md#configs_update) | **POST** /v3/configs/config | Update
[**dynamic_secrets_issue_lease**](DefaultApi.md#dynamic_secrets_issue_lease) | **POST** /v3/configs/config/dynamic_secrets/dynamic_secret/leases | Issue Lease
[**dynamic_secrets_revoke_lease**](DefaultApi.md#dynamic_secrets_revoke_lease) | **DELETE** /v3/configs/config/dynamic_secrets/dynamic_secret/leases/lease | Revoke Lease
[**environments_create**](DefaultApi.md#environments_create) | **POST** /v3/environments | Create
[**environments_delete**](DefaultApi.md#environments_delete) | **DELETE** /v3/environments/environment | Delete
[**environments_get**](DefaultApi.md#environments_get) | **GET** /v3/environments/environment | Retrieve
[**environments_list**](DefaultApi.md#environments_list) | **GET** /v3/environments | List
[**environments_rename**](DefaultApi.md#environments_rename) | **PUT** /v3/environments/environment | Rename
[**get_options**](DefaultApi.md#get_options) | **GET** /v3/integrations/integration/options | Get Options
[**groups_add_member**](DefaultApi.md#groups_add_member) | **POST** /v3/workplace/groups/group/{slug}/members | Add Member
[**groups_create**](DefaultApi.md#groups_create) | **POST** /v3/workplace/groups | Create
[**groups_delete**](DefaultApi.md#groups_delete) | **DELETE** /v3/workplace/groups/group/{slug} | Delete
[**groups_delete_member**](DefaultApi.md#groups_delete_member) | **DELETE** /v3/workplace/groups/group/{slug}/members/{type}/{member_slug} | Delete Member
[**groups_get**](DefaultApi.md#groups_get) | **GET** /v3/workplace/groups/group/{slug} | Retrieve
[**groups_list**](DefaultApi.md#groups_list) | **GET** /v3/workplace/groups | List
[**groups_update**](DefaultApi.md#groups_update) | **PATCH** /v3/workplace/groups/group/{slug} | Update
[**integrations_create**](DefaultApi.md#integrations_create) | **POST** /v3/integrations | Create
[**integrations_delete**](DefaultApi.md#integrations_delete) | **DELETE** /v3/integrations/integration | Delete
[**integrations_get**](DefaultApi.md#integrations_get) | **GET** /v3/integrations/integration | Retrieve
[**integrations_list**](DefaultApi.md#integrations_list) | **GET** /v3/integrations | List
[**integrations_update**](DefaultApi.md#integrations_update) | **PUT** /v3/integrations/integration | Update
[**invites_list**](DefaultApi.md#invites_list) | **GET** /v3/workplace/invites | List
[**project_members_add**](DefaultApi.md#project_members_add) | **POST** /v3/projects/project/members | Add
[**project_members_delete**](DefaultApi.md#project_members_delete) | **DELETE** /v3/projects/project/members/member/{type}/{slug} | Delete
[**project_members_get**](DefaultApi.md#project_members_get) | **GET** /v3/projects/project/members/member/{type}/{slug} | Retrieve
[**project_members_list**](DefaultApi.md#project_members_list) | **GET** /v3/projects/project/members | List
[**project_members_update**](DefaultApi.md#project_members_update) | **PATCH** /v3/projects/project/members/member/{type}/{slug} | Update
[**project_roles_create**](DefaultApi.md#project_roles_create) | **POST** /v3/projects/roles | Create
[**project_roles_delete**](DefaultApi.md#project_roles_delete) | **DELETE** /v3/projects/roles/role/{role} | Delete
[**project_roles_get**](DefaultApi.md#project_roles_get) | **GET** /v3/projects/roles/role/{role} | Retrieve
[**project_roles_list**](DefaultApi.md#project_roles_list) | **GET** /v3/projects/roles | List
[**project_roles_list_permissions**](DefaultApi.md#project_roles_list_permissions) | **GET** /v3/projects/permissions | List Permissions
[**project_roles_update**](DefaultApi.md#project_roles_update) | **PATCH** /v3/projects/roles/role/{role} | Update
[**projects_create**](DefaultApi.md#projects_create) | **POST** /v3/projects | Create
[**projects_delete**](DefaultApi.md#projects_delete) | **DELETE** /v3/projects/project | Delete
[**projects_get**](DefaultApi.md#projects_get) | **GET** /v3/projects/project | Retrieve
[**projects_list**](DefaultApi.md#projects_list) | **GET** /v3/projects | List
[**projects_update**](DefaultApi.md#projects_update) | **POST** /v3/projects/project | Update
[**retrieve_member**](DefaultApi.md#retrieve_member) | **GET** /v3/workplace/groups/group/{group_slug}/members/{member_type}/{member_slug} | Retrieve Member
[**secrets_delete**](DefaultApi.md#secrets_delete) | **DELETE** /v3/configs/config/secret | Delete
[**secrets_download**](DefaultApi.md#secrets_download) | **GET** /v3/configs/config/secrets/download | Download
[**secrets_get**](DefaultApi.md#secrets_get) | **GET** /v3/configs/config/secret | Retrieve
[**secrets_list**](DefaultApi.md#secrets_list) | **GET** /v3/configs/config/secrets | List
[**secrets_names**](DefaultApi.md#secrets_names) | **GET** /v3/configs/config/secrets/names | List Names
[**secrets_update**](DefaultApi.md#secrets_update) | **POST** /v3/configs/config/secrets | Update
[**secrets_update_note**](DefaultApi.md#secrets_update_note) | **POST** /v3/projects/project/note | Update Note
[**service_account_tokens_create**](DefaultApi.md#service_account_tokens_create) | **POST** /v3/workplace/service_accounts/service_account/{service_account}/tokens | Create
[**service_account_tokens_delete**](DefaultApi.md#service_account_tokens_delete) | **DELETE** /v3/workplace/service_accounts/service_account/{service_account}/tokens/token/{api_token} | Delete
[**service_account_tokens_get**](DefaultApi.md#service_account_tokens_get) | **GET** /v3/workplace/service_accounts/service_account/{service_account}/tokens/token/{api_token} | Retrieve
[**service_account_tokens_list**](DefaultApi.md#service_account_tokens_list) | **GET** /v3/workplace/service_accounts/service_account/{service_account}/tokens | List
[**service_accounts_create**](DefaultApi.md#service_accounts_create) | **POST** /v3/workplace/service_accounts | Create
[**service_accounts_delete**](DefaultApi.md#service_accounts_delete) | **DELETE** /v3/workplace/service_accounts/service_account/{slug} | Delete
[**service_accounts_get**](DefaultApi.md#service_accounts_get) | **GET** /v3/workplace/service_accounts/service_account/{slug} | Retrieve
[**service_accounts_list**](DefaultApi.md#service_accounts_list) | **GET** /v3/workplace/service_accounts | List
[**service_accounts_update**](DefaultApi.md#service_accounts_update) | **PATCH** /v3/workplace/service_accounts/service_account/{slug} | Update
[**service_tokens_create**](DefaultApi.md#service_tokens_create) | **POST** /v3/configs/config/tokens | Create
[**service_tokens_delete**](DefaultApi.md#service_tokens_delete) | **DELETE** /v3/configs/config/tokens/token | Delete
[**service_tokens_list**](DefaultApi.md#service_tokens_list) | **GET** /v3/configs/config/tokens | List
[**syncs_create**](DefaultApi.md#syncs_create) | **POST** /v3/configs/config/syncs | Create
[**syncs_delete**](DefaultApi.md#syncs_delete) | **DELETE** /v3/configs/config/syncs/sync | Delete
[**syncs_get**](DefaultApi.md#syncs_get) | **GET** /v3/configs/config/syncs/sync | Retrieve
[**users_get**](DefaultApi.md#users_get) | **GET** /v3/workplace/users/{slug} | Retrieve
[**users_list**](DefaultApi.md#users_list) | **GET** /v3/workplace/users | List
[**users_update**](DefaultApi.md#users_update) | **PATCH** /v3/workplace/users/{slug} | Update
[**webhooks_add**](DefaultApi.md#webhooks_add) | **POST** /v3/webhooks | Add
[**webhooks_delete**](DefaultApi.md#webhooks_delete) | **DELETE** /v3/webhooks/webhook/{slug} | Delete
[**webhooks_disable**](DefaultApi.md#webhooks_disable) | **POST** /v3/webhooks/webhook/{slug}/disable | Disable
[**webhooks_enable**](DefaultApi.md#webhooks_enable) | **POST** /v3/webhooks/webhook/{slug}/enable | Enable
[**webhooks_get**](DefaultApi.md#webhooks_get) | **GET** /v3/webhooks/webhook/{slug} | Retrieve
[**webhooks_list**](DefaultApi.md#webhooks_list) | **GET** /v3/webhooks | List
[**webhooks_update**](DefaultApi.md#webhooks_update) | **PATCH** /v3/webhooks/webhook/{slug} | Update
[**workplace_get**](DefaultApi.md#workplace_get) | **GET** /v3/workplace | Retrieve
[**workplace_roles_create**](DefaultApi.md#workplace_roles_create) | **POST** /v3/workplace/roles | Create
[**workplace_roles_delete**](DefaultApi.md#workplace_roles_delete) | **DELETE** /v3/workplace/roles/role/{role} | Delete
[**workplace_roles_get**](DefaultApi.md#workplace_roles_get) | **GET** /v3/workplace/roles/role/{role} | Retrieve
[**workplace_roles_list**](DefaultApi.md#workplace_roles_list) | **GET** /v3/workplace/roles | List
[**workplace_roles_list_permissions**](DefaultApi.md#workplace_roles_list_permissions) | **GET** /v3/workplace/permissions | List Permissions
[**workplace_roles_update**](DefaultApi.md#workplace_roles_update) | **PATCH** /v3/workplace/roles/role/{role} | Update
[**workplace_update**](DefaultApi.md#workplace_update) | **POST** /v3/workplace | Update



## activity_logs_list

> models::ActivityLogsListResponse activity_logs_list(page, per_page)
List

Activity Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**String**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 20]

### Return type

[**models::ActivityLogsListResponse**](ActivityLogsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_logs_retrieve

> models::ActivityLogGetResponse activity_logs_retrieve(log)
Retrieve

Activity Log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log** | **String** | Unique identifier for the log object. | [required] |[default to LOG_ID]

### Return type

[**models::ActivityLogGetResponse**](ActivityLogGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## audit_get_user

> models::AuditUserGetResponse audit_get_user(workplace_user_id, settings)
Workplace User

Get a specific user in a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workplace_user_id** | **String** | The ID of the workplace user | [required] |
**settings** | Option<**bool**> | If true, the api will return more information if the user has e.g. SAML enabled and/or Multi Factor Auth enabled |  |

### Return type

[**models::AuditUserGetResponse**](AuditUserGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_me

> models::AuthMeResponse auth_me()
Me

Get information about a token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthMeResponse**](AuthMeResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_oidc

> models::OidcAuthResponse auth_oidc(oidc_auth_request)
OIDC (Service Account Identity)

Authenticate via a Service Account Identity with OIDC. Returns a short lived API token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**oidc_auth_request** | Option<[**OidcAuthRequest**](OidcAuthRequest.md)> |  |  |

### Return type

[**models::OidcAuthResponse**](OidcAuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_revoke

> auth_revoke(revoke_auth_request)
Revoke

Revoke an auth token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_auth_request** | Option<[**RevokeAuthRequest**](RevokeAuthRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_create

> models::ChangeRequestPolicyUpdateResponse change_request_policies_create(update_change_request_policy_request)
Create

Create a new change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_change_request_policy_request** | Option<[**UpdateChangeRequestPolicyRequest**](UpdateChangeRequestPolicyRequest.md)> |  |  |

### Return type

[**models::ChangeRequestPolicyUpdateResponse**](ChangeRequestPolicyUpdateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_delete

> models::ConfigDeleteResponse change_request_policies_delete(slug)
Delete

Delete an existing change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The unique identifier of the policy. | [required] |

### Return type

[**models::ConfigDeleteResponse**](ConfigDeleteResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_get

> models::ChangeRequestPolicyGetResponse change_request_policies_get(slug)
Retrieve

Fetch an existing change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique id of the policy | [required] |

### Return type

[**models::ChangeRequestPolicyGetResponse**](ChangeRequestPolicyGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_list

> models::ChangeRequestPoliciesListResponse change_request_policies_list()
List

List existing change request policies

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChangeRequestPoliciesListResponse**](ChangeRequestPoliciesListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_update

> models::ChangeRequestPolicyUpdateResponse change_request_policies_update(slug, update_change_request_policy_request)
Update

Update an existing change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The unique identifier of the policy. | [required] |
**update_change_request_policy_request** | Option<[**UpdateChangeRequestPolicyRequest**](UpdateChangeRequestPolicyRequest.md)> |  |  |

### Return type

[**models::ChangeRequestPolicyUpdateResponse**](ChangeRequestPolicyUpdateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logs_get

> models::ConfigLogGetResponse config_logs_get(project, config, log)
Retrieve

Config Log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**log** | **String** | Unique identifier for the log object. | [required] |[default to LOG_ID]

### Return type

[**models::ConfigLogGetResponse**](ConfigLogGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logs_list

> models::ConfigLogsListResponse config_logs_list(project, config, page, per_page)
List

Config Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 20]

### Return type

[**models::ConfigLogsListResponse**](ConfigLogsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logs_rollback

> models::ConfigLogRollbackResponse config_logs_rollback(project, config, log)
Rollback

Config Log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**log** | **String** | Unique identifier for the log object. | [required] |[default to LOG_ID]

### Return type

[**models::ConfigLogRollbackResponse**](ConfigLogRollbackResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_add_trusted_ip

> models::TrustedIpAddResponse configs_add_trusted_ip(project, config, add_trusted_ip_request)
Add



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**config** | **String** |  | [required] |
**add_trusted_ip_request** | Option<[**AddTrustedIpRequest**](AddTrustedIpRequest.md)> |  |  |

### Return type

[**models::TrustedIpAddResponse**](TrustedIpAddResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_clone

> models::ConfigGetResponse configs_clone(clone_config_request)
Clone

Create a new branch config by cloning another. This duplicates a branch config and all its secrets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**clone_config_request** | Option<[**CloneConfigRequest**](CloneConfigRequest.md)> |  |  |

### Return type

[**models::ConfigGetResponse**](ConfigGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_create

> models::ConfigGetResponse configs_create(create_config_request)
Create

Create a new branch config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_config_request** | Option<[**CreateConfigRequest**](CreateConfigRequest.md)> |  |  |

### Return type

[**models::ConfigGetResponse**](ConfigGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_delete

> models::ConfigDeleteResponse configs_delete(project, config)
Delete

Permanently delete the config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project. | [required] |
**config** | **String** | Name of the config. | [required] |

### Return type

[**models::ConfigDeleteResponse**](ConfigDeleteResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_delete_trusted_ip

> configs_delete_trusted_ip(project, config, add_trusted_ip_request)
Delete



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**config** | **String** |  | [required] |
**add_trusted_ip_request** | Option<[**AddTrustedIpRequest**](AddTrustedIpRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_get

> models::ConfigGetResponse configs_get(project, config)
Retrieve

Fetch a config's details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]

### Return type

[**models::ConfigGetResponse**](ConfigGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_inheritable

> models::ConfigInheritableResponse configs_inheritable(config_inheritable_request)
Inheritable

Update the inheritability of a config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_inheritable_request** | Option<[**ConfigInheritableRequest**](ConfigInheritableRequest.md)> |  |  |

### Return type

[**models::ConfigInheritableResponse**](ConfigInheritableResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_inherits

> models::ConfigInheritsResponse configs_inherits(config_inherits_request)
Inherits



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_inherits_request** | Option<[**ConfigInheritsRequest**](ConfigInheritsRequest.md)> |  |  |

### Return type

[**models::ConfigInheritsResponse**](ConfigInheritsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_list

> models::ConfigsListResponse configs_list(project, environment, page, per_page)
List

Fetch all configs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**environment** | Option<**String**> | (optional) the environment from which to list configs |  |[default to Environment slug]
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 20]

### Return type

[**models::ConfigsListResponse**](ConfigsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_list_trusted_ips

> models::TrustedIpsListResponse configs_list_trusted_ips(project, config)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**config** | **String** |  | [required] |

### Return type

[**models::TrustedIpsListResponse**](TrustedIpsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_lock

> models::ConfigLockResponse configs_lock(unlock_config_request)
Lock

Prevent the config from being renamed or deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlock_config_request** | Option<[**UnlockConfigRequest**](UnlockConfigRequest.md)> |  |  |

### Return type

[**models::ConfigLockResponse**](ConfigLockResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_unlock

> models::ConfigGetResponse configs_unlock(unlock_config_request)
Unlock

Allow the config to be renamed and/or deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlock_config_request** | Option<[**UnlockConfigRequest**](UnlockConfigRequest.md)> |  |  |

### Return type

[**models::ConfigGetResponse**](ConfigGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_update

> models::ConfigUpdateResponse configs_update(update_config_request)
Update

Modify an existing config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_config_request** | Option<[**UpdateConfigRequest**](UpdateConfigRequest.md)> |  |  |

### Return type

[**models::ConfigUpdateResponse**](ConfigUpdateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_secrets_issue_lease

> models::DynamicSecretLeaseIssueResponse dynamic_secrets_issue_lease(issue_dynamic_secret_lease_request)
Issue Lease

Issue a lease for a dynamic secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_dynamic_secret_lease_request** | Option<[**IssueDynamicSecretLeaseRequest**](IssueDynamicSecretLeaseRequest.md)> |  |  |

### Return type

[**models::DynamicSecretLeaseIssueResponse**](DynamicSecretLeaseIssueResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_secrets_revoke_lease

> models::ConfigDeleteResponse dynamic_secrets_revoke_lease(revoke_dynamic_secret_lease_request)
Revoke Lease



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_dynamic_secret_lease_request** | Option<[**RevokeDynamicSecretLeaseRequest**](RevokeDynamicSecretLeaseRequest.md)> |  |  |

### Return type

[**models::ConfigDeleteResponse**](ConfigDeleteResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_create

> models::EnvironmentCreateResponse environments_create(project, create_environment_request)
Create

Environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**create_environment_request** | Option<[**CreateEnvironmentRequest**](CreateEnvironmentRequest.md)> |  |  |

### Return type

[**models::EnvironmentCreateResponse**](EnvironmentCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_delete

> environments_delete(project, environment)
Delete

Environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**environment** | **String** | The environment's slug | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_get

> models::EnvironmentCreateResponse environments_get(project, environment)
Retrieve

Environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**environment** | **String** | The environment's slug | [required] |

### Return type

[**models::EnvironmentCreateResponse**](EnvironmentCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_list

> models::EnvironmentsListResponse environments_list(project)
List

Environments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |

### Return type

[**models::EnvironmentsListResponse**](EnvironmentsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_rename

> models::EnvironmentCreateResponse environments_rename(project, environment, rename_environment_request)
Rename

Environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**environment** | **String** | The environment's slug | [required] |
**rename_environment_request** | Option<[**RenameEnvironmentRequest**](RenameEnvironmentRequest.md)> |  |  |

### Return type

[**models::EnvironmentCreateResponse**](EnvironmentCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_options

> serde_json::Value get_options(integration)
Get Options



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration** | **String** | The integration slug | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_add_member

> groups_add_member(slug, add_group_member_request)
Add Member



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |
**add_group_member_request** | Option<[**AddGroupMemberRequest**](AddGroupMemberRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_create

> models::GroupGetResponse groups_create(create_group_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_request** | Option<[**CreateGroupRequest**](CreateGroupRequest.md)> |  |  |

### Return type

[**models::GroupGetResponse**](GroupGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_delete

> groups_delete(slug)
Delete



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_delete_member

> groups_delete_member(slug, r#type, member_slug)
Delete Member



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |
**r#type** | **String** |  | [required] |
**member_slug** | **String** | The member's slug | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get

> models::GroupGetResponse groups_get(slug)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |

### Return type

[**models::GroupGetResponse**](GroupGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_list

> models::GroupsListResponse groups_list(page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::GroupsListResponse**](GroupsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_update

> models::GroupGetResponse groups_update(slug, update_group_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |
**update_group_request** | Option<[**UpdateGroupRequest**](UpdateGroupRequest.md)> |  |  |

### Return type

[**models::GroupGetResponse**](GroupGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_create

> models::IntegrationGetResponse integrations_create(create_integration_request)
Create

Create a new external integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_integration_request** | Option<[**CreateIntegrationRequest**](CreateIntegrationRequest.md)> |  |  |

### Return type

[**models::IntegrationGetResponse**](IntegrationGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_delete

> serde_json::Value integrations_delete(integration)
Delete

Delete an existing integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration** | **String** | The slug of the integration to delete | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_get

> models::IntegrationGetResponse integrations_get(integration)
Retrieve

Retrieve an existing integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration** | **String** | The integration slug | [required] |

### Return type

[**models::IntegrationGetResponse**](IntegrationGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_list

> models::IntegrationsListResponse integrations_list()
List

List all existing integrations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IntegrationsListResponse**](IntegrationsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_update

> serde_json::Value integrations_update(integration, update_integration_request)
Update

Update an existing integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration** | **String** | The slug of the integration to update | [required] |
**update_integration_request** | Option<[**UpdateIntegrationRequest**](UpdateIntegrationRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invites_list

> models::InvitesListResponse invites_list(page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::InvitesListResponse**](InvitesListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_add

> models::ProjectMemberAddResponse project_members_add(project, add_project_member_request)
Add



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Project slug | [required] |
**add_project_member_request** | Option<[**AddProjectMemberRequest**](AddProjectMemberRequest.md)> |  |  |

### Return type

[**models::ProjectMemberAddResponse**](ProjectMemberAddResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_delete

> project_members_delete(r#type, slug, project)
Delete



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**slug** | **String** | Member's slug | [required] |
**project** | **String** | Project slug | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_get

> models::ProjectMemberAddResponse project_members_get(project, r#type, slug)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Project slug | [required] |
**r#type** | **String** |  | [required] |
**slug** | **String** | Member's slug | [required] |

### Return type

[**models::ProjectMemberAddResponse**](ProjectMemberAddResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_list

> models::ProjectMembersListResponse project_members_list(project, page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Project slug | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ProjectMembersListResponse**](ProjectMembersListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_update

> models::ProjectMemberAddResponse project_members_update(r#type, slug, project, update_project_member_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**slug** | **String** | Member's slug | [required] |
**project** | **String** | Project slug | [required] |
**update_project_member_request** | Option<[**UpdateProjectMemberRequest**](UpdateProjectMemberRequest.md)> |  |  |

### Return type

[**models::ProjectMemberAddResponse**](ProjectMemberAddResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_create

> models::ProjectRoleUpdateResponse project_roles_create(create_project_role_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_role_request** | Option<[**CreateProjectRoleRequest**](CreateProjectRoleRequest.md)> |  |  |

### Return type

[**models::ProjectRoleUpdateResponse**](ProjectRoleUpdateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_delete

> project_roles_delete(role)
Delete



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_get

> models::ProjectRoleGetResponse project_roles_get(role)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |

### Return type

[**models::ProjectRoleGetResponse**](ProjectRoleGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_list

> models::ProjectRolesListResponse project_roles_list()
List



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProjectRolesListResponse**](ProjectRolesListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_list_permissions

> models::ProjectPermissionsResponse project_roles_list_permissions()
List Permissions



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProjectPermissionsResponse**](ProjectPermissionsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_update

> models::ProjectRoleUpdateResponse project_roles_update(role, update_project_role_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |
**update_project_role_request** | Option<[**UpdateProjectRoleRequest**](UpdateProjectRoleRequest.md)> |  |  |

### Return type

[**models::ProjectRoleUpdateResponse**](ProjectRoleUpdateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_create

> models::ProjectCreateResponse projects_create(create_project_request)
Create

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_project_request** | Option<[**CreateProjectRequest**](CreateProjectRequest.md)> |  |  |

### Return type

[**models::ProjectCreateResponse**](ProjectCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_delete

> projects_delete(delete_project_request)
Delete

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_project_request** | Option<[**DeleteProjectRequest**](DeleteProjectRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_get

> models::ProjectCreateResponse projects_get(project)
Retrieve

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]

### Return type

[**models::ProjectCreateResponse**](ProjectCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list

> models::ProjectsListResponse projects_list(page, per_page)
List

Projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 20]

### Return type

[**models::ProjectsListResponse**](ProjectsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_update

> models::ProjectCreateResponse projects_update(update_project_request)
Update

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_project_request** | Option<[**UpdateProjectRequest**](UpdateProjectRequest.md)> |  |  |

### Return type

[**models::ProjectCreateResponse**](ProjectCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_member

> models::GroupGetResponse retrieve_member(group_slug, member_type, member_slug)
Retrieve Member



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_slug** | **String** | The group's slug | [required] |
**member_type** | **String** |  | [required] |
**member_slug** | **String** | The member's slug | [required] |

### Return type

[**models::GroupGetResponse**](GroupGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_delete

> secrets_delete(project, config, name)
Delete

Secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**name** | **String** | Name of the secret. | [required] |[default to SECRET_NAME]

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_download

> models::SecretsDownloadResponse secrets_download(project, config, format, name_transformer, include_dynamic_secrets, dynamic_secrets_ttl_sec, secrets)
Download

Download Secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. Not required if using a Service Token. | [required] |
**config** | **String** | Name of the config object. Not required if using a Service Token. | [required] |
**format** | Option<**String**> |  |  |[default to json]
**name_transformer** | Option<**String**> | Transform secret names to a different case |  |
**include_dynamic_secrets** | Option<**bool**> | Whether or not to issue leases and include dynamic secret values for the config |  |[default to false]
**dynamic_secrets_ttl_sec** | Option<**i32**> | The number of seconds until dynamic leases expire. Must be used with `include_dynamic_secrets`. Defaults to 1800 (30 minutes). |  |[default to 1800]
**secrets** | Option<**String**> | Comma-delimited list of secrets to include in the download. Defaults to all secrets if left unspecified. |  |

### Return type

[**models::SecretsDownloadResponse**](SecretsDownloadResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_get

> models::SecretGetResponse secrets_get(project, config, name)
Retrieve

Secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**name** | **String** | Name of the secret. | [required] |[default to SECRET_NAME]

### Return type

[**models::SecretGetResponse**](SecretGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_list

> models::SecretsListResponse secrets_list(project, config, accepts, include_dynamic_secrets, dynamic_secrets_ttl_sec, secrets, include_managed_secrets)
List

Secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**accepts** | Option<**String**> | Available options are: **application/json**, **text/plain** |  |[default to application/json]
**include_dynamic_secrets** | Option<**bool**> | Whether or not to issue leases and include dynamic secret values for the config |  |[default to false]
**dynamic_secrets_ttl_sec** | Option<**i32**> | The number of seconds until dynamic leases expire. Must be used with `include_dynamic_secrets`. Defaults to 1800 (30 minutes). |  |
**secrets** | Option<**String**> | A comma-separated list of secrets to include in the response |  |
**include_managed_secrets** | Option<**bool**> | Whether to include Doppler's auto-generated (managed) secrets |  |[default to true]

### Return type

[**models::SecretsListResponse**](SecretsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_names

> models::SecretNamesResponse secrets_names(project, config, include_dynamic_secrets, include_managed_secrets)
List Names

Secret Names

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**include_dynamic_secrets** | Option<**bool**> | Whether or not to issue leases and include dynamic secret values for the config |  |[default to false]
**include_managed_secrets** | Option<**bool**> | Whether to include Doppler's auto-generated (managed) secrets |  |[default to true]

### Return type

[**models::SecretNamesResponse**](SecretNamesResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_update

> models::SecretsUpdateResponse secrets_update(update_secrets_request)
Update

Secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_secrets_request** | Option<[**UpdateSecretsRequest**](UpdateSecretsRequest.md)> |  |  |

### Return type

[**models::SecretsUpdateResponse**](SecretsUpdateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_update_note

> models::SecretNoteResponse secrets_update_note(project, update_secret_note_request)
Update Note

Set a note on a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> | Unique identifier for the project object. |  |[default to PROJECT_NAME]
**update_secret_note_request** | Option<[**UpdateSecretNoteRequest**](UpdateSecretNoteRequest.md)> |  |  |

### Return type

[**models::SecretNoteResponse**](SecretNoteResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_account_tokens_create

> models::ServiceAccountTokenCreateResponse service_account_tokens_create(service_account, create_service_account_token_request)
Create

Generate a new service account API token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | **String** | Slug of the service account | [required] |
**create_service_account_token_request** | Option<[**CreateServiceAccountTokenRequest**](CreateServiceAccountTokenRequest.md)> |  |  |

### Return type

[**models::ServiceAccountTokenCreateResponse**](ServiceAccountTokenCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_account_tokens_delete

> service_account_tokens_delete(service_account, api_token)
Delete

Revoke an existing service account API token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | **String** | Slug of the service account | [required] |
**api_token** | **String** | Slug of the API token | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_account_tokens_get

> models::ServiceAccountTokenGetResponse service_account_tokens_get(service_account, api_token)
Retrieve

Retrieve information about a single service account API token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | **String** | Slug of the service account | [required] |
**api_token** | **String** | Slug of the API token | [required] |

### Return type

[**models::ServiceAccountTokenGetResponse**](ServiceAccountTokenGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_account_tokens_list

> models::ServiceAccountTokensListResponse service_account_tokens_list(service_account, page, per_page)
List

List information about existing service account API tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | **String** | Slug of the service account | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ServiceAccountTokensListResponse**](ServiceAccountTokensListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_create

> models::ServiceAccountGetResponse service_accounts_create(update_service_account_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_service_account_request** | Option<[**UpdateServiceAccountRequest**](UpdateServiceAccountRequest.md)> |  |  |

### Return type

[**models::ServiceAccountGetResponse**](ServiceAccountGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_delete

> service_accounts_delete(slug)
Delete



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Slug of the service account | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_get

> models::ServiceAccountGetResponse service_accounts_get(slug)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Slug of the service account | [required] |

### Return type

[**models::ServiceAccountGetResponse**](ServiceAccountGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_list

> models::ServiceAccountsListResponse service_accounts_list(page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ServiceAccountsListResponse**](ServiceAccountsListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_update

> models::ServiceAccountUpdateResponse service_accounts_update(slug, update_service_account_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Slug of the service account | [required] |
**update_service_account_request** | Option<[**UpdateServiceAccountRequest**](UpdateServiceAccountRequest.md)> |  |  |

### Return type

[**models::ServiceAccountUpdateResponse**](ServiceAccountUpdateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_tokens_create

> models::ServiceTokenCreateResponse service_tokens_create(create_service_token_request)
Create

Service Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_service_token_request** | Option<[**CreateServiceTokenRequest**](CreateServiceTokenRequest.md)> |  |  |

### Return type

[**models::ServiceTokenCreateResponse**](ServiceTokenCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_tokens_delete

> models::ConfigDeleteResponse service_tokens_delete(delete_service_token_request)
Delete

Service Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_service_token_request** | Option<[**DeleteServiceTokenRequest**](DeleteServiceTokenRequest.md)> |  |  |

### Return type

[**models::ConfigDeleteResponse**](ConfigDeleteResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_tokens_list

> models::ServiceTokensListResponse service_tokens_list(project, config)
List

Service Tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]

### Return type

[**models::ServiceTokensListResponse**](ServiceTokensListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syncs_create

> models::SyncCreateResponse syncs_create(project, config, create_sync_request)
Create

Create a new secrets sync.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project slug | [required] |
**config** | **String** | The config slug | [required] |
**create_sync_request** | Option<[**CreateSyncRequest**](CreateSyncRequest.md)> |  |  |

### Return type

[**models::SyncCreateResponse**](SyncCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syncs_delete

> serde_json::Value syncs_delete(project, config, sync, delete_from_target)
Delete

Delete an existing sync.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project slug | [required] |
**config** | **String** | The config slug | [required] |
**sync** | **String** | The sync slug | [required] |
**delete_from_target** | **bool** | Whether or not to delete the synced data from the target integration | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syncs_get

> models::SyncCreateResponse syncs_get(project, config, sync)
Retrieve

Retrieve an existing secrets sync.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project slug | [required] |
**config** | **String** | The config slug | [required] |
**sync** | **String** | The sync slug | [required] |

### Return type

[**models::SyncCreateResponse**](SyncCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get

> models::UserGetResponse users_get(slug)
Retrieve

Get a specific user in a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The slug of the workplace user | [required] |

### Return type

[**models::UserGetResponse**](UserGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> models::UsersListResponse users_list(page, email)
List

Get all users within a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of users to fetch |  |[default to 1]
**email** | Option<**String**> | Filter results to only include the user with the provided email address |  |

### Return type

[**models::UsersListResponse**](UsersListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update

> models::UserGetResponse users_update(slug, update_user_request)
Update

Update a specific user for a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The slug of the workplace user. | [required] |
**update_user_request** | Option<[**UpdateUserRequest**](UpdateUserRequest.md)> |  |  |

### Return type

[**models::UserGetResponse**](UserGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_add

> serde_json::Value webhooks_add(project, create_webhook_request)
Add

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> | The project's name |  |
**create_webhook_request** | Option<[**CreateWebhookRequest**](CreateWebhookRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_delete

> serde_json::Value webhooks_delete(slug, project)
Delete

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Webhook's slug | [required] |
**project** | Option<**String**> | The project's name |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_disable

> serde_json::Value webhooks_disable(slug, project)
Disable

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Webhook's slug | [required] |
**project** | Option<**String**> | The project's name |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_enable

> serde_json::Value webhooks_enable(slug, project)
Enable

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Webhook's slug | [required] |
**project** | Option<**String**> | The project's name |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_get

> serde_json::Value webhooks_get(slug, project)
Retrieve

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Webhook's slug | [required] |
**project** | Option<**String**> | The project's name |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_list

> serde_json::Value webhooks_list(project)
List

Webhooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> | The project's name |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_update

> serde_json::Value webhooks_update(slug, project, update_webhook_request)
Update

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Webhook's slug | [required] |
**project** | Option<**String**> | The project's name |  |
**update_webhook_request** | Option<[**UpdateWebhookRequest**](UpdateWebhookRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_get

> models::WorkplaceGetResponse workplace_get()
Retrieve



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkplaceGetResponse**](WorkplaceGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_create

> models::WorkplaceRoleCreateResponse workplace_roles_create(create_workplace_role_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workplace_role_request** | Option<[**CreateWorkplaceRoleRequest**](CreateWorkplaceRoleRequest.md)> |  |  |

### Return type

[**models::WorkplaceRoleCreateResponse**](WorkplaceRoleCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_delete

> workplace_roles_delete(role)
Delete



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_get

> models::WorkplaceRoleGetResponse workplace_roles_get(role)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |

### Return type

[**models::WorkplaceRoleGetResponse**](WorkplaceRoleGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_list

> models::WorkplaceRolesListResponse workplace_roles_list()
List



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkplaceRolesListResponse**](WorkplaceRolesListResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_list_permissions

> models::WorkplacePermissionsResponse workplace_roles_list_permissions()
List Permissions



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkplacePermissionsResponse**](WorkplacePermissionsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_update

> models::WorkplaceRoleCreateResponse workplace_roles_update(role, update_workplace_role_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier, which is the initial name the role was given | [required] |
**update_workplace_role_request** | Option<[**UpdateWorkplaceRoleRequest**](UpdateWorkplaceRoleRequest.md)> |  |  |

### Return type

[**models::WorkplaceRoleCreateResponse**](WorkplaceRoleCreateResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_update

> models::WorkplaceGetResponse workplace_update(update_workplace_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_workplace_request** | Option<[**UpdateWorkplaceRequest**](UpdateWorkplaceRequest.md)> |  |  |

### Return type

[**models::WorkplaceGetResponse**](WorkplaceGetResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

