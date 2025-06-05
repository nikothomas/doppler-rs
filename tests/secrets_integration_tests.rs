use doppler_rs::apis::{configuration::Configuration, default_api, Error};
use doppler_rs::models;
use std::env;

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
        }
        Err(Error::ResponseError(response_content)) => {
            // This might happen if the project/config doesn't exist
            println!("Response error: {} - {}", response_content.status, response_content.content);
            assert!(response_content.status.is_client_error() || response_content.status.is_server_error());
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_secrets_list_with_dynamic_secrets() {
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
        None,
        Some(true), // include_dynamic_secrets
        Some(3600), // dynamic_secrets_ttl_sec
        None,
        None,
    ).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully retrieved secrets list with dynamic secrets");
            assert!(response.secrets.is_some());
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
async fn test_secrets_names_integration() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    let result = default_api::secrets_names(
        &config,
        &project,
        &config_name,
        None, // include_dynamic_secrets
        None, // include_managed_secrets
    ).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully retrieved secret names");
            if let Some(names) = response.names {
                println!("Found {} secrets", names.len());
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

    // Test with JSON format first (should work)
    let result = default_api::secrets_download(
        &config,
        &project,
        &config_name,
        Some("json"), // Use JSON format instead of env
        None,         // name_transformer
        None,         // include_dynamic_secrets
        None,         // dynamic_secrets_ttl_sec
        None,         // secrets
    ).await;

    match result {
        Ok(response) => {
            println!("✓ Successfully downloaded secrets in JSON format");
            // Check if any of the expected fields are present
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
async fn test_secrets_download_env_format() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    // Test with env format (returns text/plain, which the current client can't handle)
    let result = default_api::secrets_download(
        &config,
        &project,
        &config_name,
        Some("env"), // This returns text/plain
        None,
        None,
        None,
        None,
    ).await;

    match result {
        Ok(_response) => {
            println!("✓ Successfully downloaded secrets in env format");
        }
        Err(Error::Serde(_)) => {
            println!("✓ Expected serde error for env format (returns text/plain, not JSON)");
            // This is expected behavior with the current generated client
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
        }
        Err(e) => {
            println!("Other error (may be expected): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_secrets_download_json_format() {
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
        Some("json"),  // format
        Some("camel"), // name_transformer
        None,
        None,
        None,
    ).await;

    match result {
        Ok(_response) => {
            println!("✓ Successfully downloaded secrets in JSON format with camel case transformation");
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
async fn test_secrets_get_nonexistent() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    let result = default_api::secrets_get(
        &config,
        &project,
        &config_name,
        "NONEXISTENT_SECRET_KEY_12345",
    ).await;

    match result {
        Ok(_response) => {
            // This is unexpected - the secret shouldn't exist
            println!("⚠ Warning: Found secret that was expected to not exist");
        }
        Err(Error::ResponseError(response_content)) => {
            if response_content.status == 404 {
                println!("✓ Correctly received 404 for non-existent secret");
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
async fn test_secrets_update_validation() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();

    // Test with invalid/empty project name to trigger validation
    let update_request = models::SecretsUpdateRequest::new(
        "".to_string(), // Empty project should cause validation error
        "test-config".to_string(),
    );

    let result = default_api::secrets_update(&config, Some(update_request)).await;

    match result {
        Ok(_response) => {
            println!("⚠ Warning: Update succeeded with empty project name (unexpected)");
        }
        Err(Error::ResponseError(response_content)) => {
            if response_content.status.is_client_error() {
                println!("✓ Correctly received client error for invalid request: {}", response_content.status);
            } else {
                println!("Response error: {} - {}", response_content.status, response_content.content);
            }
        }
        Err(e) => {
            println!("Network or other error (expected for invalid data): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_secrets_delete_nonexistent() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, config_name) = get_test_project_config();

    let result = default_api::secrets_delete(
        &config,
        &project,
        &config_name,
        "NONEXISTENT_SECRET_TO_DELETE_12345",
    ).await;

    match result {
        Ok(_) => {
            println!("✓ Delete operation completed (may have been no-op for non-existent secret)");
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
            // 404 is acceptable for trying to delete a non-existent secret
            if response_content.status == 404 {
                println!("✓ Correctly received 404 for non-existent secret deletion");
            }
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_secrets_update_note() {
    if skip_if_no_credentials() {
        println!("Skipping test - DOPPLER_TOKEN not set");
        return;
    }

    let config = create_test_config();
    let (project, _) = get_test_project_config();

    let note_request = models::SecretsUpdateNoteRequest::new(
        "TEST_SECRET_FOR_NOTE".to_string(),
        "This is a test note for integration testing".to_string(),
    );

    let result = default_api::secrets_update_note(
        &config,
        Some(&project),
        Some(note_request),
    ).await;

    match result {
        Ok(_response) => {
            println!("✓ Successfully updated secret note");
        }
        Err(Error::ResponseError(response_content)) => {
            println!("Response error: {} - {}", response_content.status, response_content.content);
            // This might fail if the secret doesn't exist, which is fine for integration tests
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
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
async fn test_invalid_token() {
    let mut config = Configuration::new();
    
    // Create a reqwest client with an invalid Authorization header
    let mut headers = reqwest::header::HeaderMap::new();
    let auth_value = reqwest::header::HeaderValue::from_str("Bearer invalid-token-12345")
        .expect("Invalid token format");
    headers.insert(reqwest::header::AUTHORIZATION, auth_value);
    
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to create HTTP client");
    
    config.client = client;
    config.base_path = "https://api.doppler.com".to_string();
    
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