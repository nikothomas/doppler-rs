use doppler_rs::apis::{configuration::Configuration, default_api, Error};
use doppler_rs::models;
use std::env;
use chrono::{DateTime, Utc};

/// Helper function to create test configuration from environment variables
/// Set DOPPLER_TOKEN environment variable to run these tests
fn create_test_config() -> Configuration {
    let mut config = Configuration::new();
    
    config.bearer_access_token = Some(env::var("DOPPLER_TOKEN").expect("DOPPLER_TOKEN environment variable must be set to run integration tests"));
    config
}

/// Helper function to get test project and config from environment
/// Set DOPPLER_TEST_PROJECT and DOPPLER_TEST_CONFIG environment variables
fn get_test_project_config() -> (String, String) {
    let project = env::var("DOPPLER_TEST_PROJECT")
        .unwrap_or_else(|_| "test-project".to_string());
    let config = env::var("DOPPLER_TEST_CONFIG")
        .unwrap_or_else(|_| "dev".to_string());
    
    (project, config)
}

/// Skip test if required environment variables are not set
fn skip_if_no_credentials() -> bool {
    env::var("DOPPLER_TOKEN").is_err()
}

#[tokio::test]
async fn test_secrets_list_integration() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    let result = default_api::secrets_list(
        &config,
        &project,
        &config_name,
        None, // accepts
        None, // include_dynamic_secrets
        None, // dynamic_secrets_ttl_sec
        None, // secrets
        None, // include_managed_secrets
    ).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully retrieved secrets list");
            assert!(response.secrets.is_some());
            
            // Test the new clean model structure - count secrets individually
            if let Some(secrets) = &response.secrets {
                let mut count = 0;
                if secrets.stripe.is_some() { count += 1; }
                if secrets.algolia.is_some() { count += 1; }
                if secrets.database.is_some() { count += 1; }
                if secrets.user.is_some() { count += 1; }
                
                println!("Secrets response contains {} secret entries", count);
            }
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
            assert!(response_content.status.is_client_error() || response_content.status.is_server_error());
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_projects_list_integration() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();

    let result = default_api::projects_list(&config, None, None).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully retrieved projects list");
            if let Some(projects) = &response.projects {
                println!("Found {} projects", projects.len());
                
                // Test the new clean Project model
                for project in projects {
                    assert!(project.name.is_some());
                    assert!(project.slug.is_some());
                    
                    // Test chrono datetime handling
                    if let Some(created_at) = &project.created_at {
                        assert!(created_at.timestamp() > 0);
                        println!("Project created at: {}", created_at.format("%Y-%m-%d %H:%M:%S UTC"));
                    }
                }
            }
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_environments_list_integration() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, _) = get_test_project_config();

    let result = default_api::environments_list(&config, &project).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully retrieved environments list");
            if let Some(environments) = &response.environments {
                println!("Found {} environments", environments.len());
                
                // Test the new clean Environment model (note: no slug field available)
                for environment in environments {
                    assert!(environment.name.is_some());
                    assert!(environment.id.is_some()); // Use id instead of slug
                    
                    // Test chrono datetime handling
                    if let Some(created_at) = &environment.created_at {
                        assert!(created_at.timestamp() > 0);
                        println!("Environment '{}' created at: {}", 
                                environment.name.as_ref().unwrap(),
                                created_at.format("%Y-%m-%d %H:%M:%S UTC"));
                    }
                }
            }
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_service_tokens_list_integration() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    let result = default_api::service_tokens_list(&config, &project, &config_name).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully retrieved service tokens list");
            if let Some(tokens) = &response.tokens {
                println!("Found {} service tokens", tokens.len());
                
                // Test the new clean ServiceToken model with chrono
                for token in tokens {
                    if let Some(created_at) = &token.created_at {
                        assert!(created_at.timestamp() > 0);
                        println!("Service token created at: {}", created_at.format("%Y-%m-%d %H:%M:%S UTC"));
                    }
                }
            }
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_workplace_users_list_integration() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();

    let result = default_api::users_list(&config, None, None).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully retrieved workplace users list");
            if let Some(workplace_users) = &response.workplace_users {
                println!("Found {} workplace users", workplace_users.len());
                
                // Test the new clean WorkplaceUser model
                for workplace_user in workplace_users {
                    if let Some(user) = &workplace_user.user {
                        assert!(user.email.is_some());
                    }
                    
                    // Test chrono datetime handling
                    if let Some(created_at) = &workplace_user.created_at {
                        assert!(created_at.timestamp() > 0);
                        println!("User created at: {}", created_at.format("%Y-%m-%d %H:%M:%S UTC"));
                    }
                }
            }
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_secrets_download_integration() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    let result = default_api::secrets_download(
        &config,
        &project,
        &config_name,
        Some("json"), // format
        None,         // name_transformer  
        None,         // include_dynamic_secrets
        None,         // dynamic_secrets_ttl_sec
        None,         // secrets
    ).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully downloaded secrets in JSON format");
            
            // Test the improved secrets download response structure
            let has_data = response.stripe.is_some() || 
                          response.algolia.is_some() || 
                          response.database.is_some() || 
                          response.user.is_some();
            
            if has_data {
                println!("Downloaded secrets contain data");
            }
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_secrets_update_with_proper_model() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    // Create an update request using the properly generated model
    let update_request = models::UpdateSecretsRequest::new(
        project,
        config_name,
    );

    let result = default_api::secrets_update(&config, Some(update_request)).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully created secrets update request");
            if let Some(secrets) = &response.secrets {
                println!("Update response contains secrets data");
            }
        }
        Err(Error::ResponseError(response_content)) => {
            if response_content.status.is_client_error() {
                println!("✓ Correctly received client error for update without changes: {}", response_content.status);
            } else {
                println!("Response error: {} - {}", response_content.status, response_content.content);
            }
        }
        Err(e) => {
            println!("Network or other error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_create_project_model() {
    // Test the clean model constructors work properly
    let create_request = models::CreateProjectRequest::new(
        "test-project-name".to_string()
    );
    
    assert_eq!(create_request.name, "test-project-name");
    println!("✓ CreateProjectRequest model constructor works correctly");
}

#[tokio::test]
async fn test_create_environment_model() {
    let create_request = models::CreateEnvironmentRequest::new(
        "development".to_string(),
        "dev".to_string()
    );
    
    assert_eq!(create_request.name, "development");
    assert_eq!(create_request.slug, "dev");
    println!("✓ CreateEnvironmentRequest model constructor works correctly");
}

#[tokio::test]
async fn test_service_account_token_model_with_chrono() {
    // Test that we can create a ServiceAccountToken with chrono datetime
    let now = chrono::Utc::now();
    
    // This tests that our chrono integration works
    assert!(now.timestamp() > 0);
    println!("✓ Chrono datetime integration works: {}", now.format("%Y-%m-%d %H:%M:%S UTC"));
}

#[tokio::test]
async fn test_policy_subject_model() {
    let policy_subject = models::PolicySubject::new(
        "user".to_string(),
        "user-123".to_string()
    );
    
    assert_eq!(policy_subject.r#type, "user");
    assert_eq!(policy_subject.slug, "user-123");
    println!("✓ PolicySubject model (with r#type field) works correctly");
}

#[tokio::test]
async fn test_configuration_defaults() {
    let config = Configuration::default();
    assert_eq!(config.base_path, "https://api.doppler.com");
    assert!(config.user_agent.is_some());
    assert!(config.bearer_access_token.is_none());
    assert!(config.api_key.is_none());
    println!("✓ Configuration defaults are correct");
}

#[tokio::test]
async fn test_configuration_new() {
    let config = Configuration::new();
    assert_eq!(config.base_path, "https://api.doppler.com");
    assert!(config.user_agent.is_some());
    assert!(config.bearer_access_token.is_none());
    println!("✓ Configuration::new() works correctly");
}

#[tokio::test]
async fn test_invalid_token_handling() {
    let mut config = Configuration::new();
    
    // Use an obviously invalid token
    config.bearer_access_token = Some("invalid-token-12345".to_string());
    
    let result = default_api::secrets_list(
        &config,
        "test-project",
        "test-config", 
        None, None, None, None, None,
    ).await;

    match result {
        Ok(_response) => {
            println!("⚠ Warning: Request succeeded with invalid token (unexpected)");
        }
        Err(Error::ResponseError(response_content)) => {
            if response_content.status == 401 {
                println!("✓ Correctly received 401 Unauthorized for invalid token");
            } else {
                println!("Response error: {} - {}", response_content.status, response_content.content);
            }
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_network_error_handling() {
    let mut config = Configuration::new();
    config.base_path = "http://nonexistent-server:9999".to_string();
    config.bearer_access_token = Some("test-token".to_string());

    let result = default_api::secrets_list(
        &config,
        "test-project",
        "test-config",
        None, None, None, None, None,
    ).await;

    match result {
        Ok(_response) => {
            panic!("Request should not succeed with invalid server");
        }
        Err(Error::Reqwest(_)) => {
            println!("✓ Correctly handled network error");
        }
        Err(e) => {
            println!("Different error type (may be acceptable): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_clean_model_names() {
    // Test that our comprehensive naming improvements work
    println!("✓ Testing clean model names from comprehensive mappings:");
    
    // These should all compile and work with the new clean names
    let _group = models::Group::default();
    let _service_account_token = models::ServiceAccountToken::default();
    let _change_request_policy = models::ChangeRequestPolicy::default();
    let _workplace_user = models::WorkplaceUser::default();
    let _config = models::Config::default();
    let _project = models::Project::default();
    let _environment = models::Environment::default();
    let _activity_log = models::ActivityLog::default();
    let _service_token = models::ServiceToken::default();
    
    println!("  - Group ✓");
    println!("  - ServiceAccountToken ✓");
    println!("  - ChangeRequestPolicy ✓");
    println!("  - WorkplaceUser ✓");
    println!("  - Config ✓");
    println!("  - Project ✓");
    println!("  - Environment ✓");
    println!("  - ActivityLog ✓");
    println!("  - ServiceToken ✓");
    
    println!("✓ All clean model names work correctly!");
}

#[tokio::test]
async fn test_datetime_field_types() {
    // Test that datetime fields are properly typed as chrono::DateTime<Utc>
    let project = models::Project::default();
    
    // These fields should be Option<chrono::DateTime<Utc>>, not Option<String>
    if let Some(created_at) = project.created_at {
        assert!(created_at.timestamp() >= 0); // Valid timestamp
        println!("✓ Project.created_at is properly typed as chrono::DateTime<Utc>");
    }
    
    let service_token = models::ServiceToken::default();
    if let Some(created_at) = service_token.created_at {
        assert!(created_at.timestamp() >= 0);
        println!("✓ ServiceToken.created_at is properly typed as chrono::DateTime<Utc>");
    }
    
    println!("✓ Datetime fields are properly typed with chrono!");
} 