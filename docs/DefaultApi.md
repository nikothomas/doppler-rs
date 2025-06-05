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

> models::ActivityLogsList200Response activity_logs_list(page, per_page)
List

Activity Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**String**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 20]

### Return type

[**models::ActivityLogsList200Response**](activity_logs_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_logs_retrieve

> models::ActivityLogsRetrieve200Response activity_logs_retrieve(log)
Retrieve

Activity Log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log** | **String** | Unique identifier for the log object. | [required] |[default to LOG_ID]

### Return type

[**models::ActivityLogsRetrieve200Response**](activity_logs_retrieve_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## audit_get_user

> models::AuditGetUser200Response audit_get_user(workplace_user_id, settings)
Workplace User

Get a specific user in a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workplace_user_id** | **String** | The ID of the workplace user | [required] |
**settings** | Option<**bool**> | If true, the api will return more information if the user has e.g. SAML enabled and/or Multi Factor Auth enabled |  |

### Return type

[**models::AuditGetUser200Response**](audit_get_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_me

> models::AuthMe200Response auth_me()
Me

Get information about a token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthMe200Response**](auth_me_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_oidc

> models::AuthOidc200Response auth_oidc(auth_oidc_request)
OIDC (Service Account Identity)

Authenticate via a Service Account Identity with OIDC. Returns a short lived API token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_oidc_request** | Option<[**AuthOidcRequest**](AuthOidcRequest.md)> |  |  |

### Return type

[**models::AuthOidc200Response**](auth_oidc_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## auth_revoke

> auth_revoke(auth_revoke_request)
Revoke

Revoke an auth token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_revoke_request** | Option<[**AuthRevokeRequest**](AuthRevokeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_create

> models::ChangeRequestPoliciesUpdate200Response change_request_policies_create(change_request_policies_update_request)
Create

Create a new change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_request_policies_update_request** | Option<[**ChangeRequestPoliciesUpdateRequest**](ChangeRequestPoliciesUpdateRequest.md)> |  |  |

### Return type

[**models::ChangeRequestPoliciesUpdate200Response**](change_request_policies_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_delete

> models::ConfigsDelete200Response change_request_policies_delete(slug)
Delete

Delete an existing change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The unique identifier of the policy. | [required] |

### Return type

[**models::ConfigsDelete200Response**](configs_delete_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_get

> models::ChangeRequestPoliciesGet200Response change_request_policies_get(slug)
Retrieve

Fetch an existing change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Unique id of the policy | [required] |

### Return type

[**models::ChangeRequestPoliciesGet200Response**](change_request_policies_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_list

> models::ChangeRequestPoliciesList200Response change_request_policies_list()
List

List existing change request policies

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ChangeRequestPoliciesList200Response**](change_request_policies_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_request_policies_update

> models::ChangeRequestPoliciesUpdate200Response change_request_policies_update(slug, change_request_policies_update_request)
Update

Update an existing change request policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The unique identifier of the policy. | [required] |
**change_request_policies_update_request** | Option<[**ChangeRequestPoliciesUpdateRequest**](ChangeRequestPoliciesUpdateRequest.md)> |  |  |

### Return type

[**models::ChangeRequestPoliciesUpdate200Response**](change_request_policies_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logs_get

> models::ConfigLogsGet200Response config_logs_get(project, config, log)
Retrieve

Config Log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**log** | **String** | Unique identifier for the log object. | [required] |[default to LOG_ID]

### Return type

[**models::ConfigLogsGet200Response**](config_logs_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logs_list

> models::ConfigLogsList200Response config_logs_list(project, config, page, per_page)
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

[**models::ConfigLogsList200Response**](config_logs_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_logs_rollback

> models::ConfigLogsRollback200Response config_logs_rollback(project, config, log)
Rollback

Config Log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**log** | **String** | Unique identifier for the log object. | [required] |[default to LOG_ID]

### Return type

[**models::ConfigLogsRollback200Response**](config_logs_rollback_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_add_trusted_ip

> models::ConfigsAddTrustedIp200Response configs_add_trusted_ip(project, config, configs_add_trusted_ip_request)
Add



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**config** | **String** |  | [required] |
**configs_add_trusted_ip_request** | Option<[**ConfigsAddTrustedIpRequest**](ConfigsAddTrustedIpRequest.md)> |  |  |

### Return type

[**models::ConfigsAddTrustedIp200Response**](configs_add_trusted_ip_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_clone

> models::ConfigsGet200Response configs_clone(configs_clone_request)
Clone

Create a new branch config by cloning another. This duplicates a branch config and all its secrets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configs_clone_request** | Option<[**ConfigsCloneRequest**](ConfigsCloneRequest.md)> |  |  |

### Return type

[**models::ConfigsGet200Response**](configs_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_create

> models::ConfigsGet200Response configs_create(configs_create_request)
Create

Create a new branch config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configs_create_request** | Option<[**ConfigsCreateRequest**](ConfigsCreateRequest.md)> |  |  |

### Return type

[**models::ConfigsGet200Response**](configs_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_delete

> models::ConfigsDelete200Response configs_delete(project, config)
Delete

Permanently delete the config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project. | [required] |
**config** | **String** | Name of the config. | [required] |

### Return type

[**models::ConfigsDelete200Response**](configs_delete_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_delete_trusted_ip

> configs_delete_trusted_ip(project, config, configs_add_trusted_ip_request)
Delete



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**config** | **String** |  | [required] |
**configs_add_trusted_ip_request** | Option<[**ConfigsAddTrustedIpRequest**](ConfigsAddTrustedIpRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_get

> models::ConfigsGet200Response configs_get(project, config)
Retrieve

Fetch a config's details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]

### Return type

[**models::ConfigsGet200Response**](configs_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_inheritable

> models::ConfigsInheritable200Response configs_inheritable(configs_inheritable_request)
Inheritable

Update the inheritability of a config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configs_inheritable_request** | Option<[**ConfigsInheritableRequest**](ConfigsInheritableRequest.md)> |  |  |

### Return type

[**models::ConfigsInheritable200Response**](configs_inheritable_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_inherits

> models::ConfigsInherits200Response configs_inherits(configs_inherits_request)
Inherits



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configs_inherits_request** | Option<[**ConfigsInheritsRequest**](ConfigsInheritsRequest.md)> |  |  |

### Return type

[**models::ConfigsInherits200Response**](configs_inherits_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_list

> models::ConfigsList200Response configs_list(project, environment, page, per_page)
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

[**models::ConfigsList200Response**](configs_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_list_trusted_ips

> models::ConfigsListTrustedIps200Response configs_list_trusted_ips(project, config)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** |  | [required] |
**config** | **String** |  | [required] |

### Return type

[**models::ConfigsListTrustedIps200Response**](configs_list_trusted_ips_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_lock

> models::ConfigsLock200Response configs_lock(configs_unlock_request)
Lock

Prevent the config from being renamed or deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configs_unlock_request** | Option<[**ConfigsUnlockRequest**](ConfigsUnlockRequest.md)> |  |  |

### Return type

[**models::ConfigsLock200Response**](configs_lock_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_unlock

> models::ConfigsGet200Response configs_unlock(configs_unlock_request)
Unlock

Allow the config to be renamed and/or deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configs_unlock_request** | Option<[**ConfigsUnlockRequest**](ConfigsUnlockRequest.md)> |  |  |

### Return type

[**models::ConfigsGet200Response**](configs_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configs_update

> models::ConfigsUpdate200Response configs_update(configs_update_request)
Update

Modify an existing config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configs_update_request** | Option<[**ConfigsUpdateRequest**](ConfigsUpdateRequest.md)> |  |  |

### Return type

[**models::ConfigsUpdate200Response**](configs_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_secrets_issue_lease

> models::DynamicSecretsIssueLease200Response dynamic_secrets_issue_lease(dynamic_secrets_issue_lease_request)
Issue Lease

Issue a lease for a dynamic secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dynamic_secrets_issue_lease_request** | Option<[**DynamicSecretsIssueLeaseRequest**](DynamicSecretsIssueLeaseRequest.md)> |  |  |

### Return type

[**models::DynamicSecretsIssueLease200Response**](dynamic_secrets_issue_lease_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dynamic_secrets_revoke_lease

> models::ConfigsDelete200Response dynamic_secrets_revoke_lease(dynamic_secrets_revoke_lease_request)
Revoke Lease



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dynamic_secrets_revoke_lease_request** | Option<[**DynamicSecretsRevokeLeaseRequest**](DynamicSecretsRevokeLeaseRequest.md)> |  |  |

### Return type

[**models::ConfigsDelete200Response**](configs_delete_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_create

> models::EnvironmentsCreate200Response environments_create(project, environments_create_request)
Create

Environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**environments_create_request** | Option<[**EnvironmentsCreateRequest**](EnvironmentsCreateRequest.md)> |  |  |

### Return type

[**models::EnvironmentsCreate200Response**](environments_create_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_get

> models::EnvironmentsCreate200Response environments_get(project, environment)
Retrieve

Environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**environment** | **String** | The environment's slug | [required] |

### Return type

[**models::EnvironmentsCreate200Response**](environments_create_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_list

> models::EnvironmentsList200Response environments_list(project)
List

Environments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |

### Return type

[**models::EnvironmentsList200Response**](environments_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## environments_rename

> models::EnvironmentsCreate200Response environments_rename(project, environment, environments_rename_request)
Rename

Environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project's name | [required] |
**environment** | **String** | The environment's slug | [required] |
**environments_rename_request** | Option<[**EnvironmentsRenameRequest**](EnvironmentsRenameRequest.md)> |  |  |

### Return type

[**models::EnvironmentsCreate200Response**](environments_create_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_add_member

> groups_add_member(slug, groups_add_member_request)
Add Member



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |
**groups_add_member_request** | Option<[**GroupsAddMemberRequest**](GroupsAddMemberRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_create

> models::GroupsGet200Response groups_create(groups_create_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groups_create_request** | Option<[**GroupsCreateRequest**](GroupsCreateRequest.md)> |  |  |

### Return type

[**models::GroupsGet200Response**](groups_get_200_response.md)

### Authorization

No authorization required

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

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_get

> models::GroupsGet200Response groups_get(slug)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |

### Return type

[**models::GroupsGet200Response**](groups_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_list

> models::GroupsList200Response groups_list(page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::GroupsList200Response**](groups_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_update

> models::GroupsGet200Response groups_update(slug, groups_update_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The group's slug | [required] |
**groups_update_request** | Option<[**GroupsUpdateRequest**](GroupsUpdateRequest.md)> |  |  |

### Return type

[**models::GroupsGet200Response**](groups_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_create

> models::IntegrationsGet200Response integrations_create(integrations_create_request)
Create

Create a new external integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integrations_create_request** | Option<[**IntegrationsCreateRequest**](IntegrationsCreateRequest.md)> |  |  |

### Return type

[**models::IntegrationsGet200Response**](integrations_get_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_get

> models::IntegrationsGet200Response integrations_get(integration)
Retrieve

Retrieve an existing integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration** | **String** | The integration slug | [required] |

### Return type

[**models::IntegrationsGet200Response**](integrations_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_list

> models::IntegrationsList200Response integrations_list()
List

List all existing integrations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IntegrationsList200Response**](integrations_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integrations_update

> serde_json::Value integrations_update(integration, integrations_update_request)
Update

Update an existing integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration** | **String** | The slug of the integration to update | [required] |
**integrations_update_request** | Option<[**IntegrationsUpdateRequest**](IntegrationsUpdateRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invites_list

> models::InvitesList200Response invites_list(page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::InvitesList200Response**](invites_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_add

> models::ProjectMembersAdd200Response project_members_add(project, project_members_add_request)
Add



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Project slug | [required] |
**project_members_add_request** | Option<[**ProjectMembersAddRequest**](ProjectMembersAddRequest.md)> |  |  |

### Return type

[**models::ProjectMembersAdd200Response**](project_members_add_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_get

> models::ProjectMembersAdd200Response project_members_get(project, r#type, slug)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Project slug | [required] |
**r#type** | **String** |  | [required] |
**slug** | **String** | Member's slug | [required] |

### Return type

[**models::ProjectMembersAdd200Response**](project_members_add_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_list

> models::ProjectMembersList200Response project_members_list(project, page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Project slug | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ProjectMembersList200Response**](project_members_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_members_update

> models::ProjectMembersAdd200Response project_members_update(r#type, slug, project, project_members_update_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**slug** | **String** | Member's slug | [required] |
**project** | **String** | Project slug | [required] |
**project_members_update_request** | Option<[**ProjectMembersUpdateRequest**](ProjectMembersUpdateRequest.md)> |  |  |

### Return type

[**models::ProjectMembersAdd200Response**](project_members_add_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_create

> models::ProjectRolesUpdate200Response project_roles_create(project_roles_create_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_roles_create_request** | Option<[**ProjectRolesCreateRequest**](ProjectRolesCreateRequest.md)> |  |  |

### Return type

[**models::ProjectRolesUpdate200Response**](project_roles_update_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_get

> models::ProjectRolesGet200Response project_roles_get(role)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |

### Return type

[**models::ProjectRolesGet200Response**](project_roles_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_list

> models::ProjectRolesList200Response project_roles_list()
List



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProjectRolesList200Response**](project_roles_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_list_permissions

> models::ProjectRolesListPermissions200Response project_roles_list_permissions()
List Permissions



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProjectRolesListPermissions200Response**](project_roles_list_permissions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## project_roles_update

> models::ProjectRolesUpdate200Response project_roles_update(role, project_roles_update_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |
**project_roles_update_request** | Option<[**ProjectRolesUpdateRequest**](ProjectRolesUpdateRequest.md)> |  |  |

### Return type

[**models::ProjectRolesUpdate200Response**](project_roles_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_create

> models::ProjectsCreate200Response projects_create(projects_create_request)
Create

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_create_request** | Option<[**ProjectsCreateRequest**](ProjectsCreateRequest.md)> |  |  |

### Return type

[**models::ProjectsCreate200Response**](projects_create_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_delete

> projects_delete(projects_delete_request)
Delete

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_delete_request** | Option<[**ProjectsDeleteRequest**](ProjectsDeleteRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_get

> models::ProjectsCreate200Response projects_get(project)
Retrieve

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]

### Return type

[**models::ProjectsCreate200Response**](projects_create_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list

> models::ProjectsList200Response projects_list(page, per_page)
List

Projects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number |  |[default to 1]
**per_page** | Option<**i32**> | Items per page |  |[default to 20]

### Return type

[**models::ProjectsList200Response**](projects_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_update

> models::ProjectsCreate200Response projects_update(projects_update_request)
Update

Project

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_update_request** | Option<[**ProjectsUpdateRequest**](ProjectsUpdateRequest.md)> |  |  |

### Return type

[**models::ProjectsCreate200Response**](projects_create_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_member

> models::GroupsGet200Response retrieve_member(group_slug, member_type, member_slug)
Retrieve Member



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_slug** | **String** | The group's slug | [required] |
**member_type** | **String** |  | [required] |
**member_slug** | **String** | The member's slug | [required] |

### Return type

[**models::GroupsGet200Response**](groups_get_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_download

> models::SecretsDownload200Response secrets_download(project, config, format, name_transformer, include_dynamic_secrets, dynamic_secrets_ttl_sec, secrets)
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

[**models::SecretsDownload200Response**](secrets_download_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_get

> models::SecretsGet200Response secrets_get(project, config, name)
Retrieve

Secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]
**name** | **String** | Name of the secret. | [required] |[default to SECRET_NAME]

### Return type

[**models::SecretsGet200Response**](secrets_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_list

> models::SecretsList200Response secrets_list(project, config, accepts, include_dynamic_secrets, dynamic_secrets_ttl_sec, secrets, include_managed_secrets)
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

[**models::SecretsList200Response**](secrets_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_names

> models::SecretsNames200Response secrets_names(project, config, include_dynamic_secrets, include_managed_secrets)
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

[**models::SecretsNames200Response**](secrets_names_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_update

> models::SecretsUpdate200Response secrets_update(secrets_update_request)
Update

Secrets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secrets_update_request** | Option<[**SecretsUpdateRequest**](SecretsUpdateRequest.md)> |  |  |

### Return type

[**models::SecretsUpdate200Response**](secrets_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secrets_update_note

> models::SecretsUpdateNote200Response secrets_update_note(project, secrets_update_note_request)
Update Note

Set a note on a secret

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> | Unique identifier for the project object. |  |[default to PROJECT_NAME]
**secrets_update_note_request** | Option<[**SecretsUpdateNoteRequest**](SecretsUpdateNoteRequest.md)> |  |  |

### Return type

[**models::SecretsUpdateNote200Response**](secrets_update_note_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_account_tokens_create

> models::ServiceAccountTokensCreate200Response service_account_tokens_create(service_account, service_account_tokens_create_request)
Create

Generate a new service account API token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | **String** | Slug of the service account | [required] |
**service_account_tokens_create_request** | Option<[**ServiceAccountTokensCreateRequest**](ServiceAccountTokensCreateRequest.md)> |  |  |

### Return type

[**models::ServiceAccountTokensCreate200Response**](service_account_tokens_create_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_account_tokens_get

> models::ServiceAccountTokensGet200Response service_account_tokens_get(service_account, api_token)
Retrieve

Retrieve information about a single service account API token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | **String** | Slug of the service account | [required] |
**api_token** | **String** | Slug of the API token | [required] |

### Return type

[**models::ServiceAccountTokensGet200Response**](service_account_tokens_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_account_tokens_list

> models::ServiceAccountTokensList200Response service_account_tokens_list(service_account, page, per_page)
List

List information about existing service account API tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_account** | **String** | Slug of the service account | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ServiceAccountTokensList200Response**](service_account_tokens_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_create

> models::ServiceAccountsGet200Response service_accounts_create(service_accounts_update_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_accounts_update_request** | Option<[**ServiceAccountsUpdateRequest**](ServiceAccountsUpdateRequest.md)> |  |  |

### Return type

[**models::ServiceAccountsGet200Response**](service_accounts_get_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_get

> models::ServiceAccountsGet200Response service_accounts_get(slug)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Slug of the service account | [required] |

### Return type

[**models::ServiceAccountsGet200Response**](service_accounts_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_list

> models::ServiceAccountsList200Response service_accounts_list(page, per_page)
List



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**per_page** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::ServiceAccountsList200Response**](service_accounts_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_accounts_update

> models::ServiceAccountsUpdate200Response service_accounts_update(slug, service_accounts_update_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Slug of the service account | [required] |
**service_accounts_update_request** | Option<[**ServiceAccountsUpdateRequest**](ServiceAccountsUpdateRequest.md)> |  |  |

### Return type

[**models::ServiceAccountsUpdate200Response**](service_accounts_update_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_tokens_create

> models::ServiceTokensCreate200Response service_tokens_create(service_tokens_create_request)
Create

Service Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_tokens_create_request** | Option<[**ServiceTokensCreateRequest**](ServiceTokensCreateRequest.md)> |  |  |

### Return type

[**models::ServiceTokensCreate200Response**](service_tokens_create_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_tokens_delete

> models::ConfigsDelete200Response service_tokens_delete(service_tokens_delete_request)
Delete

Service Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_tokens_delete_request** | Option<[**ServiceTokensDeleteRequest**](ServiceTokensDeleteRequest.md)> |  |  |

### Return type

[**models::ConfigsDelete200Response**](configs_delete_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_tokens_list

> models::ServiceTokensList200Response service_tokens_list(project, config)
List

Service Tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | Unique identifier for the project object. | [required] |[default to PROJECT_NAME]
**config** | **String** | Name of the config object. | [required] |[default to CONFIG_NAME]

### Return type

[**models::ServiceTokensList200Response**](service_tokens_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syncs_create

> models::SyncsCreate200Response syncs_create(project, config, syncs_create_request)
Create

Create a new secrets sync.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project slug | [required] |
**config** | **String** | The config slug | [required] |
**syncs_create_request** | Option<[**SyncsCreateRequest**](SyncsCreateRequest.md)> |  |  |

### Return type

[**models::SyncsCreate200Response**](syncs_create_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syncs_get

> models::SyncsCreate200Response syncs_get(project, config, sync)
Retrieve

Retrieve an existing secrets sync.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | **String** | The project slug | [required] |
**config** | **String** | The config slug | [required] |
**sync** | **String** | The sync slug | [required] |

### Return type

[**models::SyncsCreate200Response**](syncs_create_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get

> models::UsersGet200Response users_get(slug)
Retrieve

Get a specific user in a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The slug of the workplace user | [required] |

### Return type

[**models::UsersGet200Response**](users_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> models::UsersList200Response users_list(page, email)
List

Get all users within a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of users to fetch |  |[default to 1]
**email** | Option<**String**> | Filter results to only include the user with the provided email address |  |

### Return type

[**models::UsersList200Response**](users_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_update

> models::UsersGet200Response users_update(slug, users_update_request)
Update

Update a specific user for a workplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The slug of the workplace user. | [required] |
**users_update_request** | Option<[**UsersUpdateRequest**](UsersUpdateRequest.md)> |  |  |

### Return type

[**models::UsersGet200Response**](users_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_add

> serde_json::Value webhooks_add(project, webhooks_add_request)
Add

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | Option<**String**> | The project's name |  |
**webhooks_add_request** | Option<[**WebhooksAddRequest**](WebhooksAddRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

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

No authorization required

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

No authorization required

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

No authorization required

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

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_update

> serde_json::Value webhooks_update(slug, project, webhooks_update_request)
Update

Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | Webhook's slug | [required] |
**project** | Option<**String**> | The project's name |  |
**webhooks_update_request** | Option<[**WebhooksUpdateRequest**](WebhooksUpdateRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_get

> models::WorkplaceGet200Response workplace_get()
Retrieve



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkplaceGet200Response**](workplace_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_create

> models::WorkplaceRolesCreate200Response workplace_roles_create(workplace_roles_create_request)
Create



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workplace_roles_create_request** | Option<[**WorkplaceRolesCreateRequest**](WorkplaceRolesCreateRequest.md)> |  |  |

### Return type

[**models::WorkplaceRolesCreate200Response**](workplace_roles_create_200_response.md)

### Authorization

No authorization required

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

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_get

> models::WorkplaceRolesGet200Response workplace_roles_get(role)
Retrieve



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier | [required] |

### Return type

[**models::WorkplaceRolesGet200Response**](workplace_roles_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_list

> models::WorkplaceRolesList200Response workplace_roles_list()
List



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkplaceRolesList200Response**](workplace_roles_list_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_list_permissions

> models::WorkplaceRolesListPermissions200Response workplace_roles_list_permissions()
List Permissions



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkplaceRolesListPermissions200Response**](workplace_roles_list_permissions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_roles_update

> models::WorkplaceRolesCreate200Response workplace_roles_update(role, workplace_roles_update_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** | The role's unique identifier, which is the initial name the role was given | [required] |
**workplace_roles_update_request** | Option<[**WorkplaceRolesUpdateRequest**](WorkplaceRolesUpdateRequest.md)> |  |  |

### Return type

[**models::WorkplaceRolesCreate200Response**](workplace_roles_create_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workplace_update

> models::WorkplaceGet200Response workplace_update(workplace_update_request)
Update



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workplace_update_request** | Option<[**WorkplaceUpdateRequest**](WorkplaceUpdateRequest.md)> |  |  |

### Return type

[**models::WorkplaceGet200Response**](workplace_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

